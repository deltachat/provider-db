---
name: antagonist.nl
status: PREPARATION
domains:
  - antagonist.nl
server:
  - type: IMAP
    socket: SSL
    hostname: mail.antagonist.nl
    port: 993
    username_pattern: EMAIL
  - type: smtp
    socket: STARTTLS
    hostname: mail.antagonist.nl
    port: 587
    username_pattern: EMAIL
before_login_hint: You will need a valid E-mail account with Antagonist.nl first.
last_checked: 2021-08
website: https://antagonist.nl
---

You will need a valid E-mail service with the Antagonist.nl web host first,
then enter the Antagonist hostnames, e-mail address @ your domain as user name and your
password. You can leave the default ports, but they are provided for 
confirmation here.

Help page in Dutch: [How to set up e-mail clients](https://www.antagonist.nl/help/nl/email/client/overview)
