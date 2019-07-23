#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum StatusState {
    OK,
    PREPARATION,
    BROKEN,
}
#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Status {
    state: StatusState,
    date: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Provider {
    overview_page: &'static str, // for providers.delta.chat/{overview_page}
    name: &'static str,
    status: Status,
    markdown: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
struct DomainDBEntry {
    domain: &'static str,
    list_index: u32,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

pub fn get_domain_from_email(valid_email_address: &str) -> &str {
    valid_email_address.split("@").last().unwrap()
}

pub fn get_provider_info(domain: &str) -> Option<(&Provider, Vec<&'static str>)> {
    let domain_search_res: Option<&DomainDBEntry> = DOMAIN_DB.iter().find(|e| e.domain == domain);
    let provider_id: u32 = domain_search_res?.list_index;
    
    Some((
        &DATABASE[provider_id as usize],
        get_domains_by_provider(provider_id),
    ))
}

fn get_domains_by_provider(provider_id: u32) -> Vec<&'static str> {
    return DOMAIN_DB
        .iter()
        .filter(|entry| entry.list_index == provider_id)
        .map(|e| e.domain)
        .collect();
}


#[test]
fn main() {
    println!("{:#?}", get_provider_info("mailbox.org"));
}

#[test]
fn test_example_domain() {
    assert_eq!(Some(
    (
        &Provider {
            overview_page: "example.com",
            name: "Example",
            status: Status {
                state: StatusState::PREPARATION,
                date: "2018-09",
            },
            markdown: "\n\n## Comments\n\n...\n\n## Preparations\n\n...",
        },
        vec![
            "example.com",
            "example.org",
        ],
    ),
), get_provider_info("example.org"));
}

#[test]
fn test_get_domain_from_email() {
    assert_eq!("secure.mailbox.org", get_domain_from_email("testacc.test@secure.mailbox.org"));
    assert_eq!("t.d", get_domain_from_email("0.!#$%&'*+-/=?^_`{|}~@t.d"));
    assert_eq!("b-b", get_domain_from_email("d@b-b"))
}