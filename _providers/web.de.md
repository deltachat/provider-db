---
name: WEB.DE
status: PREPARATION
domains:
  - web.de
  - email.de
  - flirt.ms
  - hallo.ms
  - kuss.ms
  - love.ms
  - magic.ms
  - singles.ms
  - cool.ms
  - kanzler.ms
  - okay.ms
  - party.ms
  - pop.ms
  - stars.ms
  - techno.ms
  - clever.ms
  - deutschland.ms
  - genial.ms
  - ich.ms
  - online.ms
  - smart.ms
  - wichtig.ms
  - action.ms
  - fussball.ms
  - joker.ms
  - planet.ms
  - power.ms
server:
  - type: imap
    socket: SSL
    hostname: imap.web.de
    port: 993
    username_pattern: emaillocalpart
  - type: imap
    socket: STARTTLS
    hostname: imap.web.de
    port: 143
    username_pattern: emaillocalpart
  - type: smtp
    socket: STARTTLS
    hostname: smtp.web.de
    port: 587
    username_pattern: emaillocalpart
before_login_hint: |
  You must allow IMAP access to your account before you can login.
last_checked: 2017-06
website: https://web.de
---

To use your web.de email address with Delta Chat you have to enable IMAP access. Please see [web.de's own article on how to do that](https://hilfe.web.de/pop-imap/einschalten.html).

Afterwards you can use Delta Chat with your web.de email address and the newly created password.
