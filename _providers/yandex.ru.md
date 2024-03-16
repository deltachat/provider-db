---
name: yandex.ru
status: PREPARATION
domains:
- yandex.com
- yandex.by
- yandex.kz
- yandex.ru
- yandex.ua
- ya.ru
- narod.ru
oauth2: yandex
server:
  - type: imap
    socket: SSL
    hostname: imap.yandex.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.yandex.com
    port: 465
before_login_hint: |
  For Yandex accounts, you have to set IMAP protocol option turned on.
last_checked: 2020-03
website: https://yandex.ru/
---

To use Delta Chat with your Yandex accounts you have to set IMAP protocol option *turned on* in the Yandex web interface. Please visit <https://yandex.com/support/mail/mail-clients/others.html> for more details.

Russian help: <https://yandex.ru/support/mail/mail-clients/others.html>
