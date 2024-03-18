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
  - type: imap
    socket: STARTTLS
    hostname: imap.rambler.ru
    port: 143
  - type: smtp
    socket: SSL
    hostname: smtp.rambler.ru
    port: 465
  # Source: https://help.rambler.ru/mail/mail-pochtovye-klienty/1275
  # They also have TLS, but README in this repo here doesn't allow it.

  # SSL
  # This one's already specified above.
  # - type: imap
  #   socket: SSL
  #   hostname: imap.rambler.ru
  #   port: 993
  - type: imap
    socket: SSL
    hostname: imap.rambler.ru
    port: 143

  # This one's already specified above.
  # - type: smtp
  #   socket: SSL
  #   hostname: smtp.rambler.ru
  #   port: 465
  - type: smtp
    socket: SSL
    hostname: smtp.rambler.ru
    port: 587
  - type: smtp
    socket: SSL
    hostname: smtp.rambler.ru
    port: 25
  - type: smtp
    socket: SSL
    hostname: smtp.rambler.ru
    port: 2525

  # STARTTLS
  - type: imap
    socket: STARTTLS
    hostname: imap.rambler.ru
    port: 993
  # This one's already specified above.
  # - type: imap
  #   socket: STARTTLS
  #   hostname: imap.rambler.ru
  #   port: 143

  - type: smtp
    socket: STARTTLS
    hostname: smtp.rambler.ru
    port: 465
  - type: smtp
    socket: STARTTLS
    hostname: smtp.rambler.ru
    port: 587
  - type: smtp
    socket: STARTTLS
    hostname: smtp.rambler.ru
    port: 25
  - type: smtp
    socket: STARTTLS
    hostname: smtp.rambler.ru
    port: 2525

  # https://help.rambler.ru/mail/mail-pochtovye-klienty/1275 also offers
  # unencrypted connections to all the same hosts. Let's leave it for users
  # to configure manually and not just automatically fall back to unencrypted
  # if all encrypted fail.
before_login_hint: |
  Чтобы войти в Рамблер/почта через Delta Chat, необходимо предварительно включить доступ с помощью почтовых клиентов на сайте mail.rambler.ru
last_checked: 2024-03
website: https://mail.rambler.ru/
---

Чтобы войти в Рамблер/почта через Delta Chat, необходимо [включить доступ с помощью почтовых клиентов на сайте mail.rambler.ru](https://help.rambler.ru/mail/mail-nastrojki-pochtovogo-yashika/2129/)
