---
name: QQ Mail
status: PREPARATION
domains:
- qq.com
- foxmail.com
server:
  - type: imap
    socket: SSL
    hostname: imap.qq.com
    port: 993
    username_pattern: emaillocalpart
  - type: smtp
    socket: STARTTLS
    hostname: smtp.qq.com
    port: 465
before_login_hint: "Manually enabling IMAP/SMTP and creating an app-specific password for Delta Chat are required."
last_checked: 2021-06
website: https://mail.qq.com/
---

To use Delta Chat with your QQ Mail address, manually enabling IMAP/SMTP and creating an app-specific password for Delta Chat are required. See also [the guide from QQ Mail](https://service.mail.qq.com/cgi-bin/help?subtype=1&id=28&no=331).

