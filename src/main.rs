#[macro_use]
extern crate clap;

use std::fs::File;
use std::str::FromStr;
use std::io::{self, Read};
use clap::{Arg, SubCommand};
use toml::{Value};

fn main() {
    let read_toml = |path: &str| -> Result<Value, io::Error> {
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let t = Value::from_str(&buf)?;
        Ok(t)
    };


    let arg_parser = || {
            app_from_crate!()
            .subcommand(SubCommand::with_name("get")
                    .arg(Arg::with_name("key"))
        )
    };

    let toml = read_toml("a.toml").unwrap();

    let app = arg_parser();
    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("get") {
        if let Some(key) = matches.value_of("key") {
            println!("{:?}", toml.get(key).unwrap());
        }
    }
}
