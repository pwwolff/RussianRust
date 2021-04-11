use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Default)]
pub struct WikiParser {
    pub(crate) file_path: String,
}

impl WikiParser {
    pub(crate) fn parse(&self) {
        let file = File::open(&self.file_path).unwrap();
        let file = BufReader::new(file);
        let mut page_count: u64 = 0;
        let mut current_node_type: String = "".to_string();
        let parser = EventReader::new(file);
        let mut current_page: WikiPage = Default::default();
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    current_node_type = name.local_name.clone();
                    if current_node_type == "page" {
                        // Reset all values, we're starting to parse a new page
                        current_page = Default::default();
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    match current_node_type.as_str() {
                        "text" => {
                            current_page.content = text;
                        },
                        "title" => {
                            current_page.name = text;
                        },
                        _ => {},
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    page_count += 1;
                    if name.local_name == "page" {
                        println!("{} pages parsed. Title: {}, wiki text is {} characters long",
                                 page_count, current_page.name, current_page.content.len());
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}

#[derive(Default)]
struct WikiPage {
    wiki_id: String,
    name: String,
    content: String,
}

impl WikiPage {
    fn is_template(&self) -> bool {
        false
    }
}