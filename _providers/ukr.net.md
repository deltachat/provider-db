---
name: ukr.net
status: PREPARATION
domains:
  - ukr.net
server:
- hostname: imap.ukr.net
  port: 993
  socket: SSL
  type: imap
- hostname: smtp.ukr.net
  port: 465
  socket: SSL
  type: smtp
before_login_hint: |
  You must allow IMAP access to your account before you can login.
last_checked: 2024-08
website: https://www.ukr.net/
---

To use your ukr.net email address with Delta Chat you have to enable access for 3rd party applications through IMAP. Please see [ukr.net's own article on how to do that](https://wiki.ukr.net/ManageIMAPAccess).
