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
before_login_hint: For now, you can't use Delta Chat to login to AOL. Stay tuned, we're working on it.
last_checked: 2017-12
website: https://aol.com
---

## Notes

This provider needs an OAuth2 implementation in core, which is to be done.

You might have to send at least one email from the web interface before logging in works.
Someone mentioned that you also have to solve a captcha somewhere (?), but maybe both has changed since 2017.

