// For parsing command line arguments.
use clap::{Arg, Command};

// For sending the exit code to the shell.
use std::process;

fn main() {
    let matches = Command::new("echo")
        .about("Echo the STRING(s) to standard output.")
        .version("echo v0.1.0")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("Artin Mobasher(artart222)")
        .arg(
            Arg::new("STRING(s)")
                .help("list of STRING(s) that you want to echo")
                .takes_value(true)
                .multiple_values(true)
                .required(true),
        )
        .arg(
            Arg::new("n")
                .short('n')
                .help("do not output the trailing newline"),
        )
        .get_matches();

    let strings: Vec<_> = matches.values_of("STRING(s)").unwrap().collect();
    let last_string: &str = strings.last().unwrap();
    for string in strings {
        if matches.is_present("n") {
            print!("{}", string);
            if last_string != string {
                print!(" ");
            }
        } else {
            println!("{}", string);
        }
    }
    process::exit(0);
}
