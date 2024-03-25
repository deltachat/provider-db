---
name: Disroot
status: OK
domains: 
  - disroot.org
server:
  - type: imap
    socket: SSL
    hostname: disroot.org
    port: 993
    username_pattern: EMAILLOCALPART
  - type: smtp
    socket: STARTTLS
    hostname: disroot.org
    port: 587
    username_pattern: EMAILLOCALPART
last_checked: 2017-06
website: https://disroot.org
---
