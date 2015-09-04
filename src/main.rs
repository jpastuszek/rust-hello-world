extern crate clap;
extern crate regex;

use clap::{Arg, App};
use regex::Regex;

fn main() {
    let matches = App::new("hello-world")
        .version("1.0")
        .author("Jakub Pastuszek <jpastuszek@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("REGEX")
            .short("m")
            .long("match")
            .help("regular expression to match 'hello world' string [default: (.*)]")
            .takes_value(true)
            )
        .get_matches();

    let reg = match Regex::new(matches.value_of("REGEX").unwrap_or("(.*)")) {
        Ok(reg) => reg,
        Err(error) => panic!("Could not compile regex: {}", error)
    };

    match reg.captures("Hello, world!") {
        Some(captures) => {
            for (group_no, group) in captures.iter().enumerate() {
                println!("capture group {}: {}", group_no, group.unwrap_or("<none>"));
            }
        },
        None => println!("Nothing matched!")
    };
}
