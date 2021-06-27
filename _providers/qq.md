---
name: QQ Mail
status: OK
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
    socket: SSL/TLS
    hostname: smtp.qq.com
    port: 465
before_login_hint: "You must create an app-specific password for Delta Chat, like as any other mail clients."
last_checked: 2021-06
website: https://mail.qq.com/
---

To use Delta Chat with your iCloud email address you have to generate a specific password for it.

Please follow [these instructions by Apple](https://support.apple.com/en-us/HT202304) to do that.

Afterwards you can use Delta Chat with your iCloud email address and the newly created password.
