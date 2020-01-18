#[macro_use]
extern crate lazy_static;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

extern crate glob;
use glob::glob;
use std::io::Read;
extern crate regex;
use regex::Regex;

pub const PROVIDER_OVERVIEW_URL: &'static str = "https://providers.delta.chat";

include!("./src/provider.rs");

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut f = File::create(&dest_path).unwrap();
    println!("path: {:?}", dest_path);
    let (provider_count, provider_data, domain_count, domain_data) = gather_data();
    f.write_fmt(format_args!(
        "static DATABASE:[Provider;{}] = [{}];\n static DOMAIN_DB:[DomainDBEntry;{}] = [{}];",
        provider_count, provider_data, domain_count, domain_data
    ))
    .unwrap();
    println!("done");

    // panic!("nope");
}

fn parse_provider(path: &Path) -> (String, Vec<String>) {
    println!("Processing provider {}", path.display());
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    //println!("{}", contents);
    file.read_to_string(&mut contents).unwrap();

    let overview_page = format!(
        "{}/{}",
        PROVIDER_OVERVIEW_URL,
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".md")
    );

    lazy_static! {
        static ref RE_YAML_AND_MD_PART: Regex = Regex::new(r"(?ims)^---\n(.+)\n---(.*)").unwrap();
    }
    let cap = RE_YAML_AND_MD_PART.captures(&contents).unwrap();
    let yaml_part = &cap[1];
    let raw_data: RawProviderData = serde_yaml::from_str(yaml_part).unwrap();
    println!("{:#?}", raw_data);

    // Validation:
    // TODO if state == preperation there should be a before_login_hint field

    // let provider = Provider {
    //     overview_page: overview_page,
    //     name: raw_data.name,
    //     before_login_hint: raw_data.before_login_hint,
    //     after_login_hint: raw_data.after_login_hint,
    //     status: raw_data.status,
    // };

    let provider = format!(
        r#" Provider {{
        overview_page: {:?},
        name: {:?},
        before_login_hint: {:?},
        after_login_hint: {:?},
        status: Status {{
            state: StatusState::{:?},
            date: {:?}
        }},
    }}"#,
        overview_page,
        raw_data.name,
        raw_data.before_login_hint,
        raw_data.after_login_hint,
        raw_data.status.state,
        raw_data.status.date,
    );

    (provider, raw_data.domains)
}

fn gather_data() -> (u32, String, u32, String) {
    println!("gather data");
    let mut provider_data = Vec::new();
    let mut provider_count: u32 = 0;
    let mut domain_data = Vec::new();
    let mut domain_count: u32 = 0;

    for e in glob("./_providers/*.md").expect("Failed to read glob pattern") {
        let (provider, domains) = parse_provider(e.unwrap().as_path());

        provider_data.push(provider);
        provider_data.push(",".to_owned());

        for domain in domains {
            domain_data.push(format!(
                "DomainDBEntry {{ domain: \"{}\", list_index: {} }}",
                domain, provider_count
            ));

            domain_data.push(",\n".to_string());
            domain_count += 1;
        }

        provider_count += 1;
    }

    //remove last commas
    provider_data.pop();
    domain_data.pop();

    let provider_string: String = provider_data.join("");
    let domain_string: String = domain_data.join("");
    (provider_count, provider_string, domain_count, domain_string)
}
