---
name: mehl.store
status: OK
domains:
  - mehl.store
  - ende.in.net
  - l2i.top
  - szh.homes
  - sls.post.in
  - ente.quest
  - ente.cfd
  - nein.jetzt
server:
  - type: imap
    socket: SSL
    hostname: mail.ende.in.net
    port: 993
    username_pattern: EMAIL
  - type: smtp
    socket: STARTTLS
    hostname: mail.ende.in.net
    port: 587
    username_pattern: EMAIL
before_login_hint: 
after_login_hint: |
    This account provides 3GB storage for eMails and the possibility to access a NEXTCLOUD-instance by using the email-credits!
last_checked: 2024-02
skip_auto_test: true
website: https://mehl.store
---

# If yout want some more...

This account provides 3GB storage for eMails and the possibility to access 
a NEXTCLOUD-instance by using the email-credits! You can't register yourself yet!

For special cases, the domain `mehl.store`offers invite-qr-codes for Delta.Chat 
[Please see here for details](https://mailadm.readthedocs.io/en/latest/#)!
