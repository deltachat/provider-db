---
name: iCloud Mail
status: PREPARATION
domains:
- icloud.com
- me.com
- mac.com
server:
  - type: imap
    socket: SSL
    hostname: imap.mail.me.com
    port: 993
    username_pattern: emaillocalpart
  - type: smtp
    socket: STARTTLS
    hostname: smtp.mail.me.com
    port: 587
before_login_hint: "You must create an app-specific password for Delta Chat before login."
last_checked: 2024-10
website: https://www.icloud.com/mail
---

To use Delta Chat with your iCloud email address [it is required to generate a specific password for it](https://support.apple.com/en-us/HT202304). Afterwards you can use Delta Chat with your iCloud email address and the newly created password.

When using a Custom Email Domain hosted on iCloud+ you need to: 
- fill in the address you want to use for Delta Chat in the "E-Mail Adress" field,
- fill in your iCloud login email in "Advanced > IMAP Login Name" and "Advanced > SMTP Login Name", and
- fill "Advanced > IMAP Server" with "imap.mail.me.com" and "Advanced > SMTP Server" with "smtp.mail.me.com".
