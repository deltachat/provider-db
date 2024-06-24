---
name: testrun.org
status: OK
domains:
- testrun.org
server:
- type: imap
  hostname: testrun.org
  port: 993
  socket: SSL
- type: smtp
  hostname: testrun.org
  port: 465
  socket: SSL
- type: imap
  hostname: testrun.org
  port: 143
  socket: STARTTLS
- type: smtp
  hostname: testrun.org
  port: 587
  socket: STARTTLS
config_defaults:
  bcc_self: 1
  sentbox_watch: 0
  mvbox_move: 0
website: https://testrun.org
last_checked: 2020-06
---
