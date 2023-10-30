extern crate clap;
use anylist_rs::login;
use clap::{Arg, ArgMatches, Command, SubCommand};

pub fn command() -> Command<'static> {
    return SubCommand::with_name("login")
        .arg(
            Arg::with_name("email")
                .short('e')
                .long("email")
                .value_name("EMAIL")
                .help("Sets the email to use")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Sets the password to use")
                .takes_value(true)
                .required(true),
        );
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let email = matches.value_of("email").unwrap();
    let password = matches.value_of("password").unwrap();
    login::login(email, password).await?;
    Ok(())
}
