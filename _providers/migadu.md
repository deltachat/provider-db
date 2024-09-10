---
name: Migadu
status: OK
domains:
  - migadu.com
server:
  - type: imap
    socket: SSL
    hostname: imap.migadu.com
    port: 993
    username_pattern: EMAIL
  - type: smtp
    socket: SSL
    hostname: smtp.migadu.com
    port: 465
    username_pattern: EMAIL
  - type: smtp
    socket: STARTTLS
    hostname: smtp.migadu.com
    port: 587
    username_pattern: EMAIL
last_checked: 2024-09
website: https://www.migadu.com/
---
