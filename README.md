# Delta Chat Provider Info


## Add an Entry
First check wether the file for the provider already exists, if it does skip to the **Edit an Entry** section.

Add [provider name].md to the _providers folder in the following format:
~~~
---
name: [provider name]
website: [website of the provider]
domains: [email domains of the provider / must be an yaml array]
before_login_hint: [~ or a string that should be displayed before the user logs in]
after_login_hint: [~ or a string that should be displayed after the user logged in]
status:
  state: [OK | PREPARATION | BROKEN] - "PREPARATION" stands for Preparation Steps needed
  date: [YYYY-MM]
---
[markdown that descripes the preperation steps, this gets displayed on the website]
~~~

Note: if the state is `PREPARATION` or `BROKEN`, you need to write something about that into the `before_login_hint` field.

### Status options:

State | Meaning
---|---
OK | works right out of the box, no additional steps needed (exception to this are custom domain email addresses where you might need to specify the smtp and imap server manualy)
PREPARATION | preparation step/s is/are needed - (a few steps are required then it works - for example enabling imap/smtp on provider page)
BROKEN | not working - Does not work. (too unstable to use falls also in this category)

## Edit an Entry

## Example
~~~
---
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
---

### Advanced Login Settings
```
imap mail.example.com:993
smtp mail.example.com:465
```
~~~

## Use as cargo Package

https://crates.io/crates/deltachat-provider-database

Usage:
```rust
extern crate deltachat_provider_database;

use deltachat_provider_database::get_provider_info;


fn main() {
    let (provider, _domains) = get_provider_info("example.org").unwrap();

    println!("{}", provider.name);

    if provider.status.state == deltachat_provider_database::StatusState::PREPARATION {
        println!("{}", provider.before_login_hint);
    }
}
```

See `cargo doc --open` for more information.
