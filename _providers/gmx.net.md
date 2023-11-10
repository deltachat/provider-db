---
name: GMX.net
status: PREPARATION
domains:
 - gmx.net
 - gmx.de
 - gmx.at
 - gmx.ch
 - gmx.org
 - gmx.eu
 - gmx.info
 - gmx.biz
 - gmx.com
server:
  - type: imap
    socket: SSL
    hostname: imap.gmx.net
    port: 993
  - type: smtp
    socket: SSL
    hostname: mail.gmx.net
    port: 465
  - type: smtp
    socket: STARTTLS
    hostname: mail.gmx.net
    port: 587
before_login_hint: |
  You must allow IMAP access to your account before you can login.
last_checked: 2017-06
website: https://www.gmx.net/
---

To use your GMX email address with Delta Chat you have to enable access for 3rd party applications through IMAP. Please see [GMX's own article on how to do that](https://support.gmx.com/pop-imap/toggle.html).
(Note: in the GMX settings, if the checkbox isn't visible in your mobile browser, try "Show website as desktop version".)

Afterwards you can use Delta Chat with your GMX email address and the newly created password.

One more tip: In case you find that Delta Chat messages are not moved into the designated "DeltaChat"-folder of your email account, please make sure that you have less than 20 folders altogether. Apparently GMX Freemail allows only for 20 folders. Note that also folders in the trash do count.

