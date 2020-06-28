---
name: sdf.org
status: PREPARATION
domains:
  - sdf.org
server:
  - type: imap
    socket: SSL
    hostname: mx.sdf.org
    port: 993
    username: EMAILLOCALPART
  - type: smtp
    socket: STARTTLS
    hostname: smtp.example.com
    port: 587
    username: EMAILADDRESS
before_login_hint: You need to create an SMTP AUTH secret at the shell.
last_checked: 2020-06
website: https://sdf.org
---

For more information about how to configure SMTP authentication, see the
[sdf.org docs](https://sdf.org/?tutorials/smtpauth).

