mod single_threaded;

use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new("rusty chat")
    .author("frigginPan")
    .about("Chat server for learning")
    .version("0.1")
    .arg(
        Arg::new("port")
        .long("port")
        .short('p')
        .default_value("8080")
        .value_parser(value_parser!(u32))
        .help("Port for server to run")
        .required(false)
        .num_args(1)
    ).get_matches();
    
    let port = matches.get_one::<u32>("port").unwrap();
    single_threaded::single_threaded_server(port);
}

