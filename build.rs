use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use glob::glob;
use serde::de::{self, value, Deserializer, SeqAccess, Visitor};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Provider {
    name: String,
    website: Option<String>,
    #[serde(deserialize_with = "string_or_vec")]
    domains: Vec<String>,
    #[serde(deserialize_with = "string_or_vec", default = "Vec::new")]
    credentials: Vec<String>,
    status: Status,
    #[serde(default = "Registration::default")]
    registration: Registration,
}

#[derive(Debug, Deserialize, Default)]
struct Registration {
    #[serde(rename(deserialize = "inviteOnly"))]
    invite_only: bool,
}

#[derive(Debug, Deserialize)]
struct Status {
    state: State,
    date: String,
}
#[derive(Debug, Deserialize)]
enum State {
    #[serde(rename(deserialize = "OK"))]
    Ok,
    #[serde(rename(deserialize = "PREP"))]
    Prep,
    #[serde(rename(deserialize = "BROKEN"))]
    Broken,
}

// Based on https://github.com/serde-rs/serde/issues/889

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![s.to_owned()])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

fn main() {
    let out_dir = "./"; //std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("providers.rs");
    let mut dest = File::create(dest_path).unwrap();

    for (i, e) in glob("./_providers/*.md")
        .expect("Failed to read glob pattern")
        .enumerate()
    {
        let pathbuf = e.unwrap();
        let path = pathbuf.as_path();
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let details = contents.split("---").nth(1).unwrap().trim();
        println!("\n\nparsing: {:?}", &details);

        let provider: Provider = serde_yaml::from_str(&details).expect("invalid provider");

        dest.write_all(
            format!(
                r#"static ref PROVIDER_{}: Provider = Provider {{
    name: "{}",
    website: Some(
        "https://aktivix.org/",
    ),
    domains: vec![
        "aktivix.org",
    ],
    credentials: vec![
        "emailPass",
    ],
    status: Status {{
        state: Prep,
        date: "2018-10",
    }},
    registration: Registration {{
        invite_only: {},
    }},
}}
"#,
                i, provider.name, provider.registration.invite_only,
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
