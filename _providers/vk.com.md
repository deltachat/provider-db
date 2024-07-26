---
name: VK Mail
status: PREPARATION
domains:
- vk.com
# Source: https://vk.com/wall-104054395_14970?w=wall-104054395_14970_r26667
# But their FAQ still states that vk.com can't work with other email clients:
# https://help.vk.mail.ru/vkmail/questions/client
# Maybe they just haven't updated it yet?
# Or maybe they're just testing and might disable it again, as it once happened:
# https://vk.com/wall-104054395_14970?w=wall-104054395_14970_r15160
#
server:
  - type: imap
    socket: SSL
    # The vk.com service is ruled by the same company as mail.ru.
    # For mail.ru see ./mail.ru.md
    hostname: imap.mail.ru
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.mail.ru
    port: 465
# I have tried to find a way to get to this link through the UI, but couldn't.
before_login_hint: |
  Вам необходимо сгенерировать "пароль для внешнего приложения" в веб-интерфейсе mail.ru
  https://account.mail.ru/user/2-step-auth/passwords/
  чтобы vk.com работал с Delta Chat.
last_checked: 2024-07
website: https://vk.mail.ru/
# Also apparently OAuth is supported as well:
# https://support.delta.chat/t/oauth2-vk-com/1854?u=wofwca
---

Вам необходимо сгенерировать "пароль для внешнего приложения" в веб-интерфейсе mail.ru
<https://account.mail.ru/user/2-step-auth/passwords/>
чтобы vk.com работал с Delta Chat.
