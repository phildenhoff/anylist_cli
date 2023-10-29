extern crate clap;
use clap::{Arg, Command, SubCommand};

pub fn login_subcommand() -> Command<'static> {
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
