---
name: five.chat
status: OK
domains:
- five.chat
strict_tls: true
server:
- type: imap
  hostname: five.chat
  port: 143
  socket: STARTTLS
- type: imap
  hostname: five.chat
  port: 993
  socket: SSL
- type: smtp
  hostname: five.chat
  port: 587
  socket: STARTTLS
config_defaults:
  bcc_self: 1
  sentbox_watch: 0
  mvbox_watch: 0
  mvbox_move: 0
website: https://five.chat
last_checked: 2020-06
---
