---
name: Mailbox.org

status: PREPARATION
domains:
- mailbox.org
- secure.mailbox.org
server:
  - type: imap
    socket: SSL
    hostname: imap.mailbox.org
    port: 993
    username: EMAILADDRESS
  - type: smtp
    socket: SSL
    hostname: smtp.mailbox.org
    port: 465
    username: EMAILADDRESS
strict_tls: true
last_checked: 2019-03
website: https://mailbox.org
---

To use Delta Chat with your mailbox.org email address you have to connect your device with your email. You can do that in the mailbox.org web interface at "Settings" (the little wheel in the upper right corner):


1. In your browser mailbox.org email account go to "Settings" and click on "Connect your device".
2. Chose the type of device you want to configure (Windows PC, Android phone or tablet, MacOS, iPhone or iPad).
3. Chose which application do you want to use (OX Drive or Mail), chose mail.
4. Save information about Server: IMAP and SMTP
5. Open / download Delta Chat, click on “add account”
6. Open "advanced settings" (below), in section IMAP Server replace “automatic” with "imap.mailbox.org", in section SMTP Server replace “automatic” with “smtp.mailbox.org"
7. Enter your mailbox.org email address and browser account password, click on login, start chatting.

---
