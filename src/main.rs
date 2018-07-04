// extern crate plist;

// use std::env;

// use std::path::Path;

extern crate clap;
extern crate plist;
extern crate prettytable;
use clap::{App, Arg, SubCommand};
use plist::Plist;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::Table;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
    let matches = App::new("simulator-fd")
        .version("1.0")
        .author("Shane Qi <qizengtai@gmail.com>")
        .about("Find out file location of Xcode simulators.")
        .arg(
            Arg::with_name("simulator directory")
                .short("d")
                .long("simulator-directory")
                .value_name("SIMULATOR_DIRECTORY")
                .help("Sets a custom simulator directory.")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("devices")
                .about("get devices list")
                .version("1.0")
                .author("Shane Qi <qizengtai@gmail.com>"),
        )
        .get_matches();

    let simulator_directory = matches
        .value_of("simulator directory")
        .map(|a| a.to_string())
        .unwrap_or_else(|| {
            let mut default_simualtor_directory = std::env::home_dir()
                .expect("Failed to find home directory.")
                .into_os_string()
                .into_string()
                .expect("Failed to find home directory.");
            default_simualtor_directory += "/Library/Developer/CoreSimulator/Devices";
            return default_simualtor_directory;
        });

    let mut table = Table::new();

    if let Some(_) = matches.subcommand_matches("devices") {
        let mut vec = Vec::<(String, String, String)>::new();
        for entry in read_dir(simulator_directory).expect("Didn't find simulators directory.") {
            if let Ok(entry) = entry {
                let mut path = entry.path();
                if path.is_dir() {
                    path.push("device.plist");
                    if let Ok(file) = File::open(path) {
                        let mut reader = BufReader::new(file);
                        let plist = Plist::from_reader(&mut reader).unwrap();
                        match plist {
                            Plist::Dict(dict) => {
                                let name = dict.get("name").and_then(|e| {
                                    if let Plist::String(name) = e {
                                        Some(name.to_owned())
                                    } else {
                                        None
                                    }
                                });
                                let runtime = dict.get("runtime").and_then(|e| {
                                    if let Plist::String(name) = e {
                                        name.to_owned().split(".").last().map(|s| s.to_string())
                                    } else {
                                        None
                                    }
                                });
                                let udid = dict.get("UDID").and_then(|e| {
                                    if let Plist::String(name) = e {
                                        Some(name.to_owned())
                                    } else {
                                        None
                                    }
                                });
                                if name.is_some() || runtime.is_some() {
                                    vec.push((
                                        name.unwrap_or("".to_string()),
                                        runtime.unwrap_or("".to_string()),
                                        udid.unwrap_or("".to_string()),
                                    ));
                                }
                            }
                            _ => (),
                        };
                    }
                }
            }
        }
        vec.sort_unstable_by(|a, b| {
            let (a_name, a_runtime, _) = a;
            let (b_name, b_runtime, _) = b;
            if a_runtime < b_runtime {
                std::cmp::Ordering::Less
            } else if a_runtime > b_runtime {
                std::cmp::Ordering::Greater
            } else {
                if a_name < b_name {
                    std::cmp::Ordering::Less
                } else if a_name > b_name {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });
        if !vec.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Name"),
                Cell::new("OS"),
                Cell::new("UDID"),
            ]));
        }
        for (name, runtime, udid) in vec {
            table.add_row(Row::new(vec![
                Cell::new(&name),
                Cell::new(&runtime),
                Cell::new(&udid),
            ]));
        }
        table.printstd();
        println!("{}", table.len());
    }
}
