// List content of a folder ...  https://doc.rust-lang.org/std/fs/fn.read_dir.html
// Command line stuff *clap*

use std::path::Path;
use std::{fs, io};

use clap::{App, Arg};

fn list_html_files(folder_path: &str) -> std::vec::Vec<std::path::PathBuf> {
    let mut entries = fs::read_dir(folder_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    entries
}

fn main() {
    // let _list_folder_result = list_local_folder();
    let matches = App::new("genaw_extractor")
        .version("0.1.0")
        .about("Tries to extract data from genaw's low carb recipe collection")
        .arg(
            Arg::with_name("genaw_root")
                .short("r")
                .long("root")
                .value_name("GENAWROOT")
                .help("Genaw's root folder, folder where every html is downloaded")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let path_genaw_root = Path::new(matches.value_of("genaw_root").unwrap());
    if !path_genaw_root.exists() {
        panic!("Path not found {:#?}", path_genaw_root);
    }

    if !path_genaw_root.is_dir() {
        panic!("Path is not a directory {:#?}", path_genaw_root);
    }

    let poop = list_html_files(path_genaw_root.to_str().unwrap());
    println!("{:#?}", poop);
}
