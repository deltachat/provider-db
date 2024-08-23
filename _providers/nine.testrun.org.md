---
name: nine.testrun.org
status: OK
domains: 
  - nine.testrun.org
server:
  - type: imap
    socket: SSL
    hostname: nine.testrun.org
    port: 443
    username_pattern: EMAIL
  - type: smtp
    socket: SSL
    hostname: nine.testrun.org
    port: 443
    username_pattern: EMAIL
  - type: imap
    socket: SSL
    hostname: nine.testrun.org
    port: 993
    username_pattern: EMAIL
  - type: smtp
    socket: SSL
    hostname: nine.testrun.org
    port: 465
    username_pattern: EMAIL
  - type: imap
    socket: STARTTLS
    hostname: nine.testrun.org
    port: 143
    username_pattern: EMAIL
  - type: smtp
    socket: STARTTLS
    hostname: nine.testrun.org
    port: 587
    username_pattern: EMAIL
last_checked: 2024-06
config_defaults:
  mvbox_move: 0
website: https://nine.testrun.org/
---
