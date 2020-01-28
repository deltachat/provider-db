---
name: OVH
status: PREPARATION
domains: 
  - custom
server:
  - type: imap
    socket: SSL
    hostname: ss10.ovh.net
    port: 465
  - type: smtp
    socket: SSL
    hostname: ss10.ovh.net
    port: 993
last_checked: 2019-12
website: https://ovh.it
---

For this provider to work, you need to allow IMAP/SMTP in the web settings.

## Comments

With OVH, there are problems receiving emails with Delta Chat.

