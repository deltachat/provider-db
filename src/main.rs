#[allow(dead_code)]
#[derive(Debug)]
enum StatusState {
    OK,
    PREPARATION,
    BROKEN,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Status {
    state: StatusState,
    date: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Provider {
    name: &'static str,
    status: Status,
    markdown: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
struct DomainDBEntry {
    domain: &'static str,
    list_index: u32,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

fn main() {
    println!("{:?}", DATABASE[0]);
}
