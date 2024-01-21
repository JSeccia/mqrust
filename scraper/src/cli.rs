use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction};

pub fn parse_arguments() -> (String, String) {
    let matches = command!()
        .version("0.1")
        .author("Jayjay")
        .about("Scraping and Kafka integration service.")
        .disable_help_flag(true)
        .arg(arg!(-h --host [HOST] "Sets a custom Kafka host").default_value("localhost"))
        .arg(arg!(-p --port [PORT] "Sets a custom Kafka port number").default_value("9092"))
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            command!("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap().clone();
    let port = matches.get_one::<String>("port").unwrap().clone();

    (host, port)
}
