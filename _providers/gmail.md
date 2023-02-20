---
name: Gmail
status: PREPARATION
domains:
  - gmail.com
  - googlemail.com
  - google.com
oauth2: gmail
server:
  - type: imap
    socket: SSL
    hostname: imap.gmail.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.gmail.com
    port: 465
opt:
  delete_to_trash: true
before_login_hint: |
  For Gmail accounts, you need to create an app-password if you have "2-Step Verification" enabled. If this setting is not available, you need to enable "less secure apps".
last_checked: 2021-08
website: https://gmail.com
---

To use Delta Chat with your Gmail email address you have to take **one of the following actions**. We prefer the first one, because it is the easiest. If that doesn't suit you, try one of the other options.

* **Authenticate using "oauth"**: *This currently works only with Delta Chat Android.* Delta Chat automatically tries this method for recognized Gmail-accounts, it is the extra login dialog you get to see when trying to log in with Delta Chat for the first time. If you cancelled that dialog you may simply start the login process again to authenticate with "oauth".
* **Create an app-specific password**: This method is only viable if you use "2-Step Verification" for logging into Gmail. It means to create an additional password that allows access to your emails, which you use only for one device or app.  Please [follow Google's instructions](https://support.google.com/accounts/answer/185833), and then use the newly generated password to log in with Delta Chat.
* **Allow "less-secure apps"**: This sounds like a bad idea, but it actually just enables a very commonly used way to access emails, so don't let yourself be scared away. Please [switch this feature on](https://myaccount.google.com/lesssecureapps), and then log in with Delta Chat just normally.

If you still have login problems,
make sure IMAP is enabled in the Gmail web interface
at **Settings / See all settings / Forwarding and POP/IMAP / IMAP access / Enable IMAP**.

## Further information

Gmail places some limits on the [amount of emails you may send and receive per day](https://support.google.com/mail/answer/22839).
