---
name: Outlook.com
status: BROKEN
domains:
  - hotmail.com
  - outlook.com
  - office365.com
  - outlook.com.tr
  - live.com
  - outlook.de
server:
  - type: imap
    socket: SSL
    hostname: outlook.office365.com
    port: 993
  - type: smtp
    socket: STARTTLS
    hostname: smtp.office365.com
    port: 587
last_checked: 2024-11
website: https://outlook.live.com/mail
---

Unfortunately, Outlook does not allow using passwords anymore,
per-app-passwords are currently not working.
