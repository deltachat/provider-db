---
name: example.com
status: BROKEN
domains:
  - example.com
  - example.org
  - example.net
server:
  - type: imap
    socket: SSL
    hostname: imap.example.com
    port: 1337
    username: EMAILLOCALPART
  - type: smtp
    socket: STARTTLS
    hostname: smtp.example.com
    port: 1337
    username: EMAILADDRESS
before_login_hint: Hush this provider doesn't exist!
after_login_hint: |
    This provider doesn't really exist, so you can't use it :/
    If you need an email provider for Delta Chat, take a look at providers.delta.chat!
last_checked: 2020-01
website: https://example.com
---

# example.com doesn't exist :/

This provider doesn't really exist, so you can't use it. But if you could, you
could find steps on this page on how to configure it for Delta Chat.

If you need an email provider, maybe take a look at [this list](https://providers.delta.chat)!

