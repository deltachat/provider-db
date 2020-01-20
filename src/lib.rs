pub mod provider;
use provider::*;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// Get the domain part of an valid email address
pub fn get_domain_from_email(valid_email_address: &str) -> &str {
    valid_email_address.split('@').last().unwrap()
}

/// Get provider info for an email domain
pub fn get_provider_info(domain: &str) -> Option<&Provider> {
    let domain_search_res: Option<&DomainDBEntry> = DOMAIN_DB.iter().find(|e| e.domain == domain);
    let provider_id: u32 = domain_search_res?.list_index;
    Some(&DATABASE[provider_id as usize])
    // A list of the domains could be retrieved by
    // get_domains_by_provider(provider_id) (commented out below)
    // See https://github.com/deltachat/provider-db/pull/20
}

/*
fn get_domains_by_provider(provider_id: u32) -> Vec<&'static str> {
    DOMAIN_DB
        .iter()
        .filter(|entry| entry.list_index == provider_id)
        .map(|e| e.domain)
        .collect()
}*/

mod tests {
    #[allow(unused_imports)]
    use super::*; // This import is NOT unused

    #[test]
    fn main() {
        println!("{:#?}", get_provider_info("mailbox.org"));
    }

    #[test]
    fn test_example_domain() {
        assert_eq!(
            Some(&Provider {
                overview_page: "https://providers.delta.chat/example.com",
                name: "example.com",
                before_login_hint: None,
                after_login_hint: Some("hush this provider doesn\'t exist"),
                status: Status {
                    state: StatusState::PREPARATION,
                    date: "2018-09"
                }
            }),
            get_provider_info("example.org")
        );
    }

    #[test]
    fn to_json() {
        let j = serde_json::to_string(&get_provider_info("example.org").unwrap());
        println!("{:?}", j)
    }

    #[test]
    fn test_get_domain_from_email() {
        assert_eq!(
            "secure.mailbox.org",
            get_domain_from_email("testacc.test@secure.mailbox.org")
        );
        assert_eq!("t.d", get_domain_from_email("0.!#$%&'*+-/=?^_`{|}~@t.d"));
        assert_eq!("b-b", get_domain_from_email("d@b-b"))
    }
}
