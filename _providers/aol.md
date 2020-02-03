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
before_login_hint: For now, you can't use Delta Chat to login to AOL.
last_checked: 2017-12
website: https://aol.com
---

This provider needs an extra OAuth2 implementation in core, which is [not
planned right now](https://github.com/deltachat/provider-db/issues/91).

