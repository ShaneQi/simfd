extern crate clap;
extern crate plist;
extern crate prettytable;
use clap::{App, Arg};
use plist::Plist;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::Table;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
    let matches = App::new("simfd")
        .version("1.0")
        .author("Shane Qi <qizengtai@gmail.com>")
        .about("Find out file location of Xcode simulators.")
        .arg(
            Arg::with_name("QUERY")
                .help("Queries to find app location or simulator device lodation.")
                .multiple(true),
        )
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .help("Search among devices instead of apps."),
        )
        .get_matches();

    let mut queries: Vec<String> = Vec::new();
    if let Some(values) = matches.values_of("QUERY") {
        for q in values {
            queries.push(q.to_owned());
        }
    }

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
    for entry in read_dir(simulator_directory).expect("Didn't find simulators directory.") {
        if let Ok(entry) = entry {
            let mut path = entry.path();
            if path.is_dir() {
                let mut device_path = path.clone();
                path.push("device.plist");
                if let Ok(file) = File::open(path) {
                    if let (Some(device_name), Some(runtime), Some(udid)) = device_plist_info(file)
                    {
                        if matches.occurrences_of("device") > 0 {
                            if match_device_info(&device_name, &runtime, &udid, &queries) {
                                table.add_row(Row::new(vec![
                                    Cell::new(&device_name),
                                    Cell::new(&runtime),
                                    Cell::new(device_path.to_str().unwrap_or("")),
                                ]));
                            }
                        } else {
                            device_path.push("data/Containers/Data/Application");
                            if let Ok(entries) = read_dir(device_path) {
                                for entry in entries {
                                    if let Ok(entry) = entry {
                                        let mut path = entry.path();
                                        let path_clone = path.clone();
                                        let app_path = path_clone
                                            .into_os_string()
                                            .into_string()
                                            .unwrap_or("".to_string());
                                        path.push(
                                            ".com.apple.mobile_container_manager.metadata.plist",
                                        );
                                        if let Ok(file) = File::open(path) {
                                            if let Some(bundle_id) = app_plist_info(file) {
                                                if match_app_info(
                                                    &device_name,
                                                    &runtime,
                                                    &udid,
                                                    &bundle_id,
                                                    &queries,
                                                ) {
                                                    table.add_row(Row::new(vec![
                                                        Cell::new(&bundle_id),
                                                        Cell::new(&device_name),
                                                        Cell::new(&runtime),
                                                        Cell::new(&app_path),
                                                    ]));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if table.len() > 0 {
        table.printstd();
    } else {
        println!("Didn't find any app that matches queries.")
    }
}

fn device_plist_info(file: std::fs::File) -> (Option<String>, Option<String>, Option<String>) {
    if let Ok(Plist::Dict(dict)) = Plist::from_reader(&mut BufReader::new(file)) {
        let name = dict.get("name").and_then(|e| {
            if let Plist::String(name) = e {
                Some(name.to_owned())
            } else {
                None
            }
        });
        let runtime = dict.get("runtime").and_then(|e| {
            if let Plist::String(name) = e {
                name.to_owned()
                    .split(".")
                    .last()
                    .map(|s| s.to_string())
                    .and_then(|s| {
                        let mut components = Vec::<String>::new();
                        for component in s.split("-") {
                            components.push(component.to_string())
                        }
                        if let (Some(os), Some(major), Some(minor)) =
                            (components.get(0), components.get(1), components.get(2))
                        {
                            Some(format!("{} {}.{}", os, major, minor))
                        } else {
                            None
                        }
                    })
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
        (name, runtime, udid)
    } else {
        (None, None, None)
    }
}

fn app_plist_info(file: std::fs::File) -> Option<String> {
    if let Ok(Plist::Dict(dict)) = Plist::from_reader(&mut BufReader::new(file)) {
        dict.get("MCMMetadataIdentifier").and_then(|e| {
            if let Plist::String(name) = e {
                Some(name.to_owned())
            } else {
                None
            }
        })
    } else {
        None
    }
}

fn match_app_info(
    device_name: &String,
    runtime: &String,
    udid: &String,
    bundle_id: &String,
    queries: &Vec<String>,
) -> bool {
    for q in queries {
        if bundle_id.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&bundle_id.to_lowercase())
            || device_name.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&device_name.to_lowercase())
            || runtime.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&runtime.to_lowercase())
            || udid.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&udid.to_lowercase())
        {
        } else {
            return false;
        }
    }
    true
}

fn match_device_info(
    device_name: &String,
    runtime: &String,
    udid: &String,
    queries: &Vec<String>,
) -> bool {
    for q in queries {
        if device_name.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&device_name.to_lowercase())
            || runtime.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&runtime.to_lowercase())
            || udid.to_lowercase().contains(&q.to_lowercase())
            || q.to_lowercase().contains(&udid.to_lowercase())
        {
        } else {
            return false;
        }
    }
    true
}
