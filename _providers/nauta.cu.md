---
name: nauta.cu
domains:
  - nauta.cu
status: OK
max_smtp_rcpt_to: 20
strict_tls: false
server:
  - type: imap
    socket: STARTTLS
    hostname: imap.nauta.cu
    port: 143
  - type: smtp
    socket: STARTTLS
    hostname: smtp.nauta.cu
    port: 25
config_defaults:
  delete_server_after: 1
  bcc_self: 0
  sentbox_watch: 0
  mvbox_watch: 0
  mvbox_move: 0
  e2ee_enabled: 0
  media_quality: 1
  fetch_existing_msgs: 0
last_checked: 2021-01
website: https://webmail.nauta.cu
---

