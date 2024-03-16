---
name: Mail.ru
status: PREPARATION
domains:
- mail.ru
- inbox.ru
- internet.ru
- bk.ru
- list.ru
server:
  - type: imap
    socket: SSL
    hostname: imap.mail.ru
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.mail.ru
    port: 465
before_login_hint: Вам необходимо сгенерировать "пароль для внешнего приложения" в веб-интерфейсе mail.ru, чтобы mail.ru работал с Delta Chat.
last_checked: 2021-07
website: https://mail.ru/
---

См. <https://help.mail.ru/mail/security/protection/external> для получения
дополнительной информации о том, как сгенерировать такой внешний пароль для
Delta Chat.

