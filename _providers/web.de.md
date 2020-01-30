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
after_login_hint: |
  Note: if you have your web.de spam settings too strict, you won't receive contact requests from new people.
  If you want to receive contact requests, you should disable the "3-Wege-Spamschutz" in the web.de settings. 
  Read how: https://hilfe.web.de/email/spam-und-viren/spamschutz-einstellungen.html
last_checked: 2020-01
website: https://web.de
---

To use your web.de email address with Delta Chat you have to enable IMAP access. Please see [web.de's own article on how to do that](https://hilfe.web.de/pop-imap/einschalten.html).

Afterwards you can use Delta Chat with your web.de email address and the newly created password.

## Disable 3-Way Spam Protection

If the 3-Way Spam Protection is enabled in the web.de settings, Delta Chat
won't work as expected, as all emails by unknown contacts will be moved to the
"Unbekannt" folder instead of the Inbox. 

If you want to receive contact requests, you should downgrade it to 2-Way Spam
Protection, as described here:
[https://hilfe.web.de/email/spam-und-viren/spamschutz-einstellungen.html](https://hilfe.web.de/email/spam-und-viren/spamschutz-einstellungen.html)

![3-Wege-Spamschutz in den web.de-Einstellungen](../assets/img/web.de-spam-protection.png)

One more tip: In case you find that Delta Chat messages are not moved into the designated "DeltaChat"-folder of your email account, please make sure that you have less than 40 folders altogether. Apparently web.de allows only for 40 folders. Note that also folders in the trash do count.
