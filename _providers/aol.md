---
name: Aol. (America Online)
domains: 
  - aol.com
status: BROKEN
servers:
  - type: imap
    socket: SSL
    hostname: imap.aol.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.aol.com
    port: 465
before_login_hint: You can't use Delta Chat to login to AOL.
last_checked: 2020-02
website: https://aol.com
---

To enable using AOL, Delta Chat would have to provide an extra authentication
mechanism (OAuth2). There are [no plans for
this](https://github.com/deltachat/provider-db/issues/91) in the foreseeable
future, so for now you can't use Delta Chat to log into your AOL account.

