extern crate plist;

use plist::Plist;
use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path_string = get_working_path();
    for entry in read_dir(Path::new(&path_string)).expect("") {
        let mut path = entry.expect("").path();
        if path.is_dir() {
            let path_clone = path.clone();
            let app_id = path_clone.file_name().expect("").to_str().expect("");
            path.push(".com.apple.mobile_container_manager.metadata.plist");
            let file = File::open(path).expect("");
            let mut reader = BufReader::new(file);
            let plist = Plist::from_reader(&mut reader).unwrap();
            match plist {
                Plist::Dict(dict) => {
                    let id = dict.get(&"MCMMetadataIdentifier".to_string()).expect("");
                    match id {
                        Plist::String(bundle_id) => {
                            println!("{} {}", app_id, bundle_id);
                        }
                        _ => (),
                    }
                }
                _ => (),
            };
        }
    }
}

fn get_working_path() -> String {
    let arg_working_path = env::args().nth(1);
    if let Some(path) = arg_working_path {
        return path.as_str().to_string();
    } else {
        return env::current_dir()
            .expect("")
            .to_str()
            .expect("")
            .to_string();
    }
}
