---
name: rambler.ru
status: PREPARATION
domains:
# From the registration page
- rambler.ru
- autorambler.ru
- myrambler.ru
- rambler.ua
# From https://help.rambler.ru/mail/mail-pochtovye-klienty/1275
- lenta.ru
# https://help.rambler.ru/mail/mail-legal/1329
- ro.ru
- r0.ru
server:
  # Source: https://help.rambler.ru/mail/mail-nastrojki-pochtovogo-yashika/2129/
  - type: imap
    socket: SSL
    hostname: imap.rambler.ru
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.rambler.ru
    port: 465
  # Let's put `STARTTLS` below all the `SSL` ones.
  - type: imap
    socket: STARTTLS
    hostname: imap.rambler.ru
    port: 143
  # They also say "if the above ones don't work, try these":
  # https://help.rambler.ru/mail/mail-pochtovye-klienty/1275
  # But many of those don't appear to actually work right now,
  # our CI fails.
  #
  # They also have TLS, but README in this repo here doesn't allow it.
before_login_hint: |
  Чтобы войти в Рамблер/почта через Delta Chat, необходимо предварительно включить доступ с помощью почтовых клиентов на сайте mail.rambler.ru
last_checked: 2024-03
website: https://mail.rambler.ru/
---

Чтобы войти в Рамблер/почта через Delta Chat, необходимо [включить доступ с помощью почтовых клиентов на сайте mail.rambler.ru](https://help.rambler.ru/mail/mail-nastrojki-pochtovogo-yashika/2129/)
