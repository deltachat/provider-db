// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


extern crate glob;
use glob::glob;
use std::io::Read;
extern crate regex;

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
        println!("{}", contents);
    }
}


/*

Todo/plan/idea: 
- [X] get all file paths
- [X] read file content
- [ ] seperate yaml fontmatter from the markdown content
- [ ] parse yaml
- [ ] strip markdown? convert to only text?
- [ ] save data in code to create struct

*/