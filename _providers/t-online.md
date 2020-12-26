---
name: T-Online
status: PREPARATION
domains: 
  - t-online.de
  - magenta.de
server:
  - type: imap
    socket: SSL
    hostname: secureimap.t-online.de
    port: 993
  - type: smtp
    socket: SSL
    hostname: securesmtp.t-online.de
    port: 465
before_login_hint: To use Delta Chat with a T-Online email address, you need to create an app password in the web interface.
last_checked: 2020-02
website: https://www.t-online.de/
---

To use Delta Chat with a T-Online email address, you need an app password. A
(german) guide how to set this up is here:
[https://www.telekom.de/hilfe/festnetz-internet-tv/e-mail/e-mail-adresse-passwoerter-und-sicherheit/passwort-fuer-e-mail-programme-einrichten](https://www.telekom.de/hilfe/festnetz-internet-tv/e-mail/e-mail-adresse-passwoerter-und-sicherheit/passwort-fuer-e-mail-programme-einrichten)

You need to enter an additional password for external access in the [web
interface settings](https://email.t-online.de/pr?a=globalsettings.passwords).
To get to the settings, go to the T-Online webmail interface, click on the
wheel in the top right, and then click "Passwörter" on the left. You can now
set a password at "Passwort für E-Mail-Programme ändern".

Enter your password twice to set it, and confirm it. Now you can login with
this password in Delta Chat.

