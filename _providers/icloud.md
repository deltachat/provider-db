---
name: iCloud Mail
website: https://www.icloud.com/mail
domains: 
- icloud.com
- me.com
- mac.com
credentials: emailAppPass
before_login_hint: "You must create a specific password for Delta Chat before you can login. Please see PAGE_URL for details."
after_login_hint: |
  An example hint that contains newlines.
  Sometimes we might have a lot to say.
status:
 state: PREP
 date: 2017-05
connection:
  imap:
    host: FIXME
    port: FIXME
    sec: FIXME
  smtp:
    host: FIXME
    port: FIXME
    sec: FIXME
---

To use Delta Chat with your iCloud email address you have to generate a specific password for it.

Please follow [these instructions](https://support.apple.com/en-us/HT202304) to do that.

Afterwards you can login to Delta Chat using your iCloud email address and the newly created password.
