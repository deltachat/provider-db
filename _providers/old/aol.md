---
name: Aol. (America Online)
domains: 
  - aol.com
status: BROKEN
servers:
  - type: imap
    socket: SSL
    hostname: export.imap.aol.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.aol.com
    port: 465
before_login_hint: |
  You have to send at least one email from the web interface before logging in works.
last_checked: 2017-12
website: https://aol.com
---

