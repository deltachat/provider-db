#[macro_use] extern crate lazy_static;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

extern crate glob;
use glob::glob;
use std::io::Read;
extern crate regex;
use regex::Regex;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();

    gather_data();
    println!("done");
}

fn gather_data(){
    println!("gather data");

    for e in glob("./_providers/*.md").expect("Failed to read glob pattern") {
        let pathbuf = e.unwrap();
        let path = pathbuf.as_path();
        //println!("{}", path.display());
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        //println!("{}", contents);
        lazy_static! {
            static ref RE: Regex =  Regex::new(r"(?ims)^---\n(.+)\n---(.*)").unwrap();
        }
        let cap = RE.captures(&contents).unwrap();
        let yaml_part = &cap[1];
        let md_part = &cap[2];
        println!("{} -> {}", yaml_part, md_part);
    }
}



/*

Todo/plan/idea: 
- [X] get all file paths
- [X] read file content
- [X] seperate yaml fontmatter from the markdown content (with regex)
- [ ] parse yaml
    https://crates.io/crates/yaml-rust
- [ ] strip markdown? convert to only text?
- [ ] save data in code to create struct

*/