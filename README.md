# Delta Chat Provider Info


## Add an Entry
First check wether the file for the provider already exists, if it does skip to the **Edit an Entry** section.

Add [provider name].md to the `_providers` folder in the following format:

```
---
name: Example Provider
state: [OK | PREPARATION | BROKEN]
domains:
- example.org
servers:
  - type: imap
    socket: SSL
    hostname: imap.example.org
    port: 123
    username: defaults to email address
  - type: imap
    socket: STARTTLS
    hostname: imap.example.org
    port: 456
    username: defaults to email address
  - type: smtp
    socket: SSL
    hostname: smtp.example.org
    port: 789
    username: defaults to email address
before_login_hint: 
    optional text that is be displayed before the user logs in
    typically in combination with state=PREPARATION or state=BROKEN
after_login_hint: optional,
    text that is added to the device chat after login
date: [optional, YYYY-MM]
website: [optional, website of the provider]
---                                                                             
[optional markdown that descripes the preperation steps, this gets displayed on the website]
```

### Status options:

State | Meaning
---|---
OK | works right out of the box, no additional steps needed (exception to this are custom domain email addresses where you might need to specify the smtp and imap server manualy)
PREPARATION | preparation steps are needed - (a few steps are required then it works - for example enabling imap/smtp on provider page)
BROKEN | not working - Does not work. (too unstable to use falls also in this category)

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
        println!("{}", provider.markdown);
    }
}
```

See `cargo doc --open` for more information.
