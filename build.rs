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
use yaml_rust::{YamlLoader};

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
        let _md_part = &cap[2];
        //println!("{} -> {}", yaml_part, md_part);
        let yaml = &YamlLoader::load_from_str(yaml_part).unwrap()[0];

        let p_name = yaml["name"].as_str().unwrap();
        println!("{}", p_name);
        let p_domains = parse_yml_string_array(yaml["domains"].clone());
        // todo status.state -> merge the pr first (https://github.com/deltachat/provider-overview/pull/15)
        let p_status_date = yaml["status"]["date"].as_str().unwrap();
        println!("{}; {:?}", p_status_date, p_domains);
    }
}


fn parse_yml_string_array(array:yaml_rust::yaml::Yaml) -> Vec<String>{
    //? could be one string or an array of strings -> eitherway please convert to vector?
    if !array.is_array() {
            return vec![array.as_str().unwrap().to_string()]
    } else {
        let a:Vec<String> = 
        array
            .into_vec().unwrap()
            .into_iter()
            .map(|x| x.as_str().unwrap().to_string()).collect::<Vec<String>>();
        return a;
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
idea:
- [ ] Error on missing yml/invalid value? / ci test to run on pull requests?
*/