---
name: yahoo
state: PREPARATION
domains: 
  - yahoo.com
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
last_checked: 2017-12
website: https://yahoo.com
---

To use Delta Chat with your yahoo email address you have to allow "less secure apps":

In your [account security screen](https://login.yahoo.com/account/security) you will find a setting titled "Allow apps that use less secure sign in". Please switch that on to allow Delta Chat to log in to your account.

Don't be afraid: "less secure sign in" sounds bad, but it isn't. The wording deliberately tries to scare you away because Yahoo wants you to only use their own apps.
