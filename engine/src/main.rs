use std::io;
use std::fs;
use std::fs::File;
use std::process::exit;
use std::path::Path;
use std::collections::HashMap;
use xml::reader::{XmlEvent, EventReader};

#[derived(Debug)]
struct Lexer<'a> {
    content: &'a[char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a[char]) -> Self {
        Self {content}
    }

    fn trim_left(&mut self) {
        while.self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn next_token(&mut self) -> Option<&'a[char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a[char];

    fn next (&mut self) -> Options<Self::Item> {
        self.next_token();
    }
}
fn index_doc(_doc_content: &str) -> HashMap<String, usize> {
    todo!("not implemented");

}

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);

    let mut content = String::new();

    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("") {
            content.push_str(&text);
        }
    }
    Ok(content)

}

fn main() -> io::Result<()> {
    let content = read_entire_xml_file("docs.gl/gl4/glVertexAttribDivisor.xhhtml")?.chars().collect::<Vec<_>>();
    for token in Lexer::new(&content) {
        println!("{token:?}");


    }
    //map to a path to a file and the frequency of terms within a file
    // let all_docs = HashMap::<Path, HashMap<String, usize>>::new();    
    // let dir_path = "docs.,gl/gl4";
    // let dir = fs::read_dir(dir_path)?;
    // for file in dir {
    //     let file_path = file?.path();
    //     let content = read_entire_xml_file(&file_path)?;
    //     println!("{file_path:?} => {size}", size = &content.len());
    // }
    
   //println!("{content}"), content = read_entire_xml_file(file_path).expect("");
   Ok(())
}

// To do: implement a tokenizer
