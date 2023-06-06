mod models;

extern crate plist;
use models::{ItermTheme, WarpTheme};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(first_arg) = args.get(1) {
        let iterm_theme: Result<ItermTheme, _> = plist::from_file(first_arg);
        let iterm_theme = match iterm_theme {
            Ok(theme) => theme,
            Err(err) => {
                eprintln!("{:?}", err);
                eprintln!("Unable to read file");
                process::exit(1);
            }
        };
        let warp_theme: WarpTheme = iterm_theme.into();
        let yaml = serde_yaml::to_string(&warp_theme);
        let yaml = match yaml {
            Ok(yaml) => yaml,
            Err(_) => {
                eprintln!("Unable to convert to yaml");
                process::exit(1);
            }
        };

        println!("{}", yaml);
    } else {
        eprintln!("Missing input file");
        process::exit(1);
    }
}
