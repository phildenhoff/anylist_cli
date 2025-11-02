use anylist_rs::AnyListClient;
use chrono::Local;
use clap::{ArgMatches, Command};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;

use crate::auth::read_tokens;
use crate::error::CliError;

pub fn command() -> Command {
    Command::new("tail")
        .about("Monitor AnyList events in real-time via WebSocket")
        .long_about(
            "Connect to AnyList's WebSocket server and display events as they occur.\n\n\
             Events are printed to stdout with timestamps. Press Ctrl+C or 'q' to exit gracefully.",
        )
}

pub async fn exec_command(_matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = Arc::new(AnyListClient::from_tokens(tokens)?);

    println!("Connecting to AnyList WebSocket...");

    // Start real-time sync with explicit stdout flushing
    let mut sync = client.start_realtime_sync(move |event| {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        print!("[{}] {:?}\n\r", timestamp, event);
        let _ = io::stdout().flush();
    }).await?;

    println!("Connected! Monitoring events... (Press Ctrl+C or 'q' to quit)\n");
    io::stdout().flush().ok();

    // Enable raw mode for single keypress detection
    enable_raw_mode().map_err(crate::error::CliError::IoError)?;

    let result = monitor_events(&mut sync).await;

    // Always disable raw mode on exit
    disable_raw_mode().ok();

    match result {
        Ok(_) => {
            // Gracefully disconnect
            sync.disconnect().await?;
            println!("\nDisconnected.");
            Ok(())
        }
        Err(e) => {
            sync.disconnect().await.ok();
            Err(e)
        }
    }
}

async fn monitor_events(sync: &mut anylist_rs::RealtimeSync) -> Result<(), CliError> {
    use anylist_rs::ConnectionState;

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    let mut iteration = 0;
    loop {
        iteration += 1;
        tokio::select! {
            _ = &mut ctrl_c => {
                println!("\r\nReceived Ctrl+C, shutting down...");
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // Check for keypresses
                if event::poll(Duration::from_millis(0))
                    .map_err(crate::error::CliError::IoError)? {

                    if let Event::Key(KeyEvent { code, .. }) = event::read()
                        .map_err(crate::error::CliError::IoError)? {

                        match code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                println!("\r\nReceived 'q', shutting down...");
                                break;
                            }
                            KeyCode::Char('c') if cfg!(unix) => {
                                // Ctrl+C on Unix
                                println!("\r\nShutting down...");
                                break;
                            }
                            _ => {}
                        }
                    }
                }

                // Check connection state
                let state = sync.state().await;
                if iteration == 1 || iteration % 10 == 0 {
                    io::stdout().flush().ok();
                }

                // Only exit on Closed or Disconnected, not Reconnecting
                match state {
                    ConnectionState::Closed => {
                        println!("\r\nConnection permanently closed.");
                        break;
                    }
                    ConnectionState::Disconnected => {
                        println!("\r\nConnection disconnected.");
                        break;
                    }
                    ConnectionState::Reconnecting => {
                        // Let it try to reconnect, don't exit
                        if iteration % 50 == 0 {
                            println!("Still attempting to reconnect...");
                        }
                    }
                    ConnectionState::Connected | ConnectionState::Connecting => {
                        // All good, continue
                    }
                }
            }
        }
    }

    Ok(())
}
