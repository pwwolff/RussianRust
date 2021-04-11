mod settings_factory;
mod wiktionary_parser;

use std::collections::HashMap;

fn main() {
    let settings: HashMap<String, String> = settings_factory::get_settings();
    let wiki_path = settings.get("wiktionary_path").unwrap().clone();
    let ru_wiktionary = wiktionary_parser::WikiParser{file_path: wiki_path, ..Default::default()};
    ru_wiktionary.parse();
}
