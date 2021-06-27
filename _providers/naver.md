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
    socket: SSL/TLS
    hostname: smtp.naver.com
    port: 587
before_login_hint: "You do not create an app-specific password for Delta Chat, but manually enabling IMAP/SMTP is required, like using any other mail clients with Naver Mail."
last_checked: 2021-06
website: https://mail.naver.com/
---

Please follow [the document provided by
Naver](https://help.naver.com/support/contents/contents.help?serviceNo=2342&categoryNo=2288) to enable IMAP/SMTP.
