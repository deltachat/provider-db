---
name: e.email
status: OK
domains:
  - e.email
server:
  - type: imap
    socket: SSL
    hostname: mail.ecloud.global
    port: 993
    username_pattern: EMAIL
  - type: smtp
    socket: STARTTLS
    hostname: mail.ecloud.global
    port: 587
    username_pattern: EMAIL
last_checked: 2021-01
website: https://e.foundation
---

