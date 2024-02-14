---
name: Westfalen-Kolleg Paderborn
status: PREPARATION
domains: 
  - wkpb.de
server:
  - type: imap
    socket: SSL
    hostname: pimap.schulon.org
    port: 993
    username_pattern: EMAIL
  - type: smtp
    socket: SSL
    hostname: psmtp.schulon.org
    port: 465
    username_pattern: EMAIL
before_login_hint: "Dies sind die gleichen Anmeldedaten wie bei Moodle und Abitur-Online."
last_checked: 2023-12
website: https://wkpb.de/
---

Student and teacher email of the Westfalen-Kolleg in Paderborn.
