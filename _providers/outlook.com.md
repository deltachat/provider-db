---
name: Outlook.com
status: BROKEN
before_login_hint: |
    Unfortunately, Outlook does not allow using passwords anymore,
    per-app-passwords are currently not working.
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
per-app-passwords are reported to not being working in November 2024.
Please let us know if you succeed using Outlook, so we can adapt these hints.
