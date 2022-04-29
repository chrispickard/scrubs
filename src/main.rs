use clap::{arg, command, Arg};

use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let _cmd = command!()
        .args(&[
            Arg::new("config")
                .short('s')
                .long("config")
                .env("CONFIG")
                .value_name("FILE"),
            arg!(-d --debug ... "turns on debugging information and allows multiples"),
            arg!([input] "an optional input file to use"),
        ])
        .get_matches();
    for entry in WalkDir::new("/home/chrispickard/Videos/yt").into_iter() {
        let entry = entry.unwrap();
        println!("{:?}", entry);
    }
    let stuff = comma::parse_command("echo what hey there delilah").unwrap();
    Command::new(stuff.get(0).unwrap())
        .args(stuff)
        .spawn()
        .unwrap();
}
