---
name: mailo.com
status: OK
domains:
  - mailo.com
server:
  - type: imap
    socket: SSL
    hostname: imap.mailo.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.mailo.com
    port: 465
last_checked: 2020-02
website: mailo.com
---
