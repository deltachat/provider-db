---
name: Google Mail
state: PREPARATION
domains:
  - gmail.com
  - googlemail.com
server:
  - type: imap
    socket: SSL
    hostname: imap.gmail.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.gmail.com
    port: 465
before_login_hint: |
  For Gmail accounts, you need to create an app-password if you have "2-Step Verification" enabled. If this setting is not available, you need to enable "less secure apps".
last_checked: 2018-05
website: https://gmail.com
---

**TODO**: Extend this text.

To use Delta Chat with your Gmail email address you have to take one of the following paths:

* Authenticate using "oauth" (works only on Android). That is the extra login dialog you get to see when trying to login into Delta Chat on your Android phone. If you cancelled that dialog you may simply start the login process again.
* Allow "less-secure apps" (if you use a second factor for logging in).
* Create an app-specific password for Delta Chat.


Additional information on limits regarding sending and receiving emails are described on https://support.google.com/mail/answer/22839?hl=en.
