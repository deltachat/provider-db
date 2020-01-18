use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StatusState {
    /// Works right out of the box without any preperation steps needed
    OK,
    /// Works, but preparation steps are needed
    PREPARATION,
    /// Doesn't work (too unstable to use falls also in this category)
    BROKEN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// The status of a provider
pub struct Status {
    pub state: StatusState,
    /// Date of when the state was last checked/updated
    pub date: &'static str,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Information about a provider
pub struct Provider {
    /// for linking to the providers page on the overview website
    /// like `providers.delta.chat/{overview_page}`
    pub overview_page: &'static str,
    pub name: &'static str,
    pub before_login_hint: Option<&'static str>,
    pub after_login_hint: Option<&'static str>,
    pub status: Status,
    // todo connection
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DomainDBEntry {
    pub domain: &'static str,
    pub list_index: u32,
}

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// /// supported login method/s (supported by this provider)
// pub enum LoginMethod {
//     EmailPassword,
//     AppPassword,
//     OAuth2
// }


#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Only used by build.rs
pub struct RawStatus {
    pub state: StatusState,
    /// Date of when the state was last checked/updated
    pub date: String,
}

/// What is parsed from the yaml font matter
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RawProviderData {
    pub name: String, // we use string here because serde_yaml doesn't support string references with lifetime yet
    pub website: String,
    pub domains: Vec<String>,
    pub before_login_hint: Option<String>,
    pub after_login_hint: Option<String>,
    pub status: RawStatus,
    // todo connection
}
// RawProviderData is build.rs exclusively, todo - don't compile it into the provider overview

/* changes to yaml:

- remove unused filds
- rename credentials to loginMethods and edit its content too

- add new fields

*/

mod tests {
    #[allow(unused_imports)]
    use super::*; // This import is NOT unused

    #[test]
    fn raw_provider_data() {
        let provider = RawProviderData {
            name: "example.com".to_owned(),
            website: "https://example.com".to_owned(),
            domains: vec!["example.com".to_owned(), "example.org".to_owned()],
            before_login_hint: None,
            after_login_hint: Some("hush this provider doesn't exist".to_owned()),
            status: RawStatus {
                state: StatusState::PREPARATION,
                date: "2018-09".to_owned(),
            },
        };

        // println!("{:#?}", serde_yaml::to_string(&provider));

        let teststr = r#"---
        name: example.com
        website: https://example.com
        domains:
            - example.com
            - example.org
        before_login_hint: ~
        after_login_hint: "hush this provider doesn't exist"
        status:
            state: PREPARATION
            date: 2018-09
        "#;

        let pr: RawProviderData = serde_yaml::from_str(teststr).unwrap();

        //println!("{:#?}", pr)
        assert_eq!(provider, pr);
    }
}
