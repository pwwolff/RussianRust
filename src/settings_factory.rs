extern crate glob;
extern crate config;

use std::collections::HashMap;
use config::*;
use glob::glob;

pub fn get_settings() -> HashMap<String, String>{
    let mut settings = Config::default();
    settings
        .merge(glob("config/*")
            .unwrap()
            .map(|path| File::from(path.unwrap()))
            .collect::<Vec<_>>())
        .unwrap();

    // Print out our settings (as a HashMap)
    settings.try_into::<HashMap<String, String>>().unwrap()
}