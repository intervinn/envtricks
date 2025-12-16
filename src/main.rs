use std::{io::{self, Write}};

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

use crate::file::EnvFile;

mod file;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(
    about = "Move the fields from one file to another",
    long_about = "
Copies `fields` of `source` into `destination`, 
keeping all of `destination` fields that weren't present in `source`. 

If `-a` is present and `fields` is empty, all fields all 
fields of `source` are copied into `destination`

If `-a` is present and `fields` if present, all 
fields except for the ones present in `fields` are 
copied into `destination`

Example:
    Move the fields of .env file into .env.prod:
        dotenvtricks move -a .env .env.prod
    Move the fields of .env file into .env.prod, **EXCEPT** for `SECRET`:
        dotenvtricks move -a .env .env.prod SECRET
    ")]
    Move {
        source: String,
        destination: String,
        #[arg(short, long)]
        all: bool,
        fields: Vec<String>
    },
    #[command(about = "Generate a .env with all values removed")]
    Example {
        source: String,
        destination: String,
    },
    #[command(about = "Removed specified fields from file")]
    Remove {
        source: String,
        fields: Vec<String>
    },
    #[command(about = "Set key to value in file")]
    Set {
        source: String,
        key: String,
        value: String
    },
    #[command(about = "Write value of a field into stdout")]
    Get {
        source: String,
        key: String
    }
}

fn execute(cmd: &Command) -> Result<()> {
    match cmd {
        Command::Example { source, destination } => {
            let mut file = EnvFile::load(source)?;
            for (name, _) in file.fields() {
                file.set_field(&name, "");
            }

            file.write(&destination)?;
        }
        Command::Move { source, destination, all, fields } => {
            let mut src = EnvFile::load(source)?;
            let mut dst = EnvFile::load_or_new(destination)?;

            let src_fields = src.fields();
            for (name, value) in src_fields {
                if *all == true {
                    if !fields.contains(&name) {
                        dst.set_field(&name, &value);
                    }
                } else {
                    if fields.contains(&name) {
                        src.set_field(&name, &value);
                    }
                }
            }

            dst.write(destination)?;
        }
        Command::Remove { source, fields } => {
            let mut src = EnvFile::load(&source)?;
            for field in fields {
                src.set_field(field, "");
            }
            src.write(source)?;
        },
        Command::Set { source, key, value } => {
            let mut src = EnvFile::load(source)?;
            src.set_field(key, value);
            src.write(source)?;
        }
        Command::Get { source, key } => {
            let src = EnvFile::load(source)?;
            let value = src.get_field(key).unwrap_or_default();
            io::stdout().write(value.as_bytes())?;
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    match execute(&cli.command) {
        Err(e) => {
            panic!("command failed: {}", e)
        },
        _ok => {}
    };
}
