---
name: Yggmail
domains: 
  - yggmail
status: PREPARATION
strict_tls: false
server:
  - type: imap
    socket: PLAIN
    hostname: localhost
    port: 1143
  - type: smtp
    socket: PLAIN
    hostname: localhost
    port: 1025
before_login_hint: An Yggmail companion app needs to be installed on your device to access the Yggmail network.
after_login_hint: |
    Make sure, the Yggmail companion app runs whenever you want to use this account.
    Note, that you usually cannot write from @yggmail addresses to normal e-mail-addresses (as @gmx.net). However, you can create another account in the normal e-mail-network for this purpose.
last_checked: 2021-08
website: https://github.com/neilalexander/yggmail
---

Yggmail servers usually exist only on your local device
and you need to install a companion app to make them work.

[Yggmail Project Homepage](https://github.com/neilalexander/yggmail)
