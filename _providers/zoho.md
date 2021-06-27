---
name: Zoho Mail
domains: 
  - zohomail.eu
  - zoho.com 
status: PREPARATION
server:
  - type: imap
    socket: SSL
    hostname: imap.zoho.eu
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.zoho.eu
    port: 465
before_login_hint: To use Zoho Mail, you have to turn on IMAP in the Zoho Mail backend.
last_checked: 2021-06
website: https://www.zoho.com/
---

To enable using Zoho Mail, you have to turn on IMAP Access:

1. Log into Zoho Mail on the web and navigate to [Settings > Mail Accounts](https://mail.zoho.eu/zm/#settings/all/mailaccounts)
2. There should be check box for imap access so tick that

These instructions are also on [this Zoho Mail help page](https://www.zoho.com/mail/help/imap-access.html#EnableIMAPAccess)

You should also make sure that you specify the mail server manually since going with the defaults may result in errors.
