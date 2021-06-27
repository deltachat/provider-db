---
name: Naver Mail
status: PREPARATION
domains:
- naver.com
server:
  - type: imap
    socket: SSL
    hostname: imap.naver.com
    port: 993
    username_pattern: emaillocalpart
  - type: smtp
    socket: STARTTLS
    hostname: smtp.naver.com
    port: 587
before_login_hint: "Manually enabling IMAP/SMTP is required."
last_checked: 2021-06
website: https://mail.naver.com/
---

[Manually enabling IMAP/SMTP](https://help.naver.com/support/contents/contents.help?serviceNo=2342&categoryNo=2288) is required.
