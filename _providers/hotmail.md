---
name: Hotmail (Outlook, Office365)
status: BROKEN
domains:
  - hotmail.com
  - outlook.com
  - office365.com
  - outlook.com.tr
  - live.com
server:
  - type: imap
    socket: SSL
    hostname: imap-mail.outlook.com
    port: 993
  - type: smtp
    socket: STARTTLS
    hostname: smtp-mail.outlook.com
    port: 587
before_login_hint: |
  Outlook email addresses will not work as expected as these servers remove some important transport information.
  Hopefully sooner or later there will be a fix, for now we suggest to use another email address.
after_login_hint: |
  Outlook email addresses will not work as expected as these servers remove some important transport information.
  Unencrypted 1-on-1 chats kind of work, but groups and encryption don't.
  Hopefully sooner or later there will be a fix, for now we suggest to use another email address.
  You may check in advance if your newly chosen provider works well with Delta Chat at https://providers.delta.chat/
last_checked: 2019-02
website: https://outlook.live.com/mail
---

Unfortunately outlook.com breaks using Delta Chat by manipulating messages in a way that we can't work around. We're very sorry indeed, but it's not our fault.

If this situation ever changes we will update this page, but for now please use a different email provider with Delta Chat.

