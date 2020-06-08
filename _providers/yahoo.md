---
name: Yahoo
status: PREPARATION
domains: 
  - yahoo.com
  - yahoo.de
  - yahoo.it
  - yahoo.fr
  - yahoo.es
  - yahoo.se
  - yahoo.co.uk
  - yahoo.co.nz
  - yahoo.com.au
  - yahoo.com.ar
  - yahoo.com.br
  - yahoo.com.mx
  - ymail.com
  - rocketmail.com
  - yahoodns.net
server:
  - type: imap
    socket: SSL
    hostname: imap.mail.yahoo.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.mail.yahoo.com
    port: 465
before_login_hint: To use Delta Chat with your Yahoo email address you have to create an "App-Password" in the account security screen.
last_checked: 2020-06
website: https://yahoo.com
---

To use Delta Chat with your Yahoo email address you have to create an "App-Password" in the account security screen and use it in Delta Chat:

In your [account security screen](https://login.yahoo.com/account/security) you will find a setting titled "Generate app password". Pleaso use it to generate an app password. Then enter the generated password in Delta Chat.
