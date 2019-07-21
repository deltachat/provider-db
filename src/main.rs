#[allow(dead_code)]
enum StatusState {
   OK,
   PREPARATION,
   BROKEN
}

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());
}

