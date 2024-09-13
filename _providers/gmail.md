---
name: Gmail
status: PREPARATION
domains:
  - gmail.com
  - googlemail.com
  - google.com
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
  For Gmail accounts, you need to have "2-Step Verification" enabled and create an app-password.
last_checked: 2024-08
website: https://gmail.com
---

To use Delta Chat with your Gmail email address,
you need to enable **2-Step Verification** in your Google Account
and then **Create an app-specific password**.

This means to create an additional password which you use only for one device or app.
Please [follow Google's instructions](https://support.google.com/accounts/answer/185833),
create an app-password at <https://myaccount.google.com/apppasswords>
and then use the newly generated password to log in with Delta Chat.

If you still have login problems,
make sure IMAP is enabled in the Gmail web interface
at **Settings / See all settings / Forwarding and POP/IMAP / IMAP access / Enable IMAP**.

## Further information

Gmail places some limits on the [amount of emails you may send and receive per day](https://support.google.com/mail/answer/22839).

In the past, we supported Google's Oauth as well.
Due Google adding more and more bureaucracy in that area,
we dropped support for that.
This allows our small open source project
using our limited resources
in areas with larger impact.
