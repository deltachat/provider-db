---
name: dubby.org
status: OK
domains:
- dubby.org
strict_tls: true
server:
- type: imap
  hostname: dubby.org
  port: 993
  socket: SSL
- type: smtp
  hostname: dubby.org
  port: 587
  socket: STARTTLS
- type: smtp
  hostname: dubby.org
  port: 465
  socket: SSL
config_defaults:
  bcc_self: 1
  sentbox_watch: 0
  mvbox_move: 0
website: https://dubby.org
last_checked: 2020-06
---
