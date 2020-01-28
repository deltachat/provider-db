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
before_login_hint: For this provider to work, you need to allow IMAP/SMTP in the web settings.
after_login_hint: |
    With your provider there can be problems with receiving messages. 
    If this bothers you, consider picking another provider from https://providers.delta.chat :)
last_checked: 2019-12
website: https://ovh.it
---

For this provider to work, you need to allow IMAP/SMTP in the web settings.

