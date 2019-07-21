---
name: Google Mail
domains:
 - gmail.com
 - googlemail.com
credentials: 
 - emailPass
 - emailAppPass
 - Oauth
status:
 state: PREP
 date: 2018-05
limits:
 maxRecipients: 500
 messagesPerDay: 500

registration:
 inviteOnly: false
 PhoneAuthRequired: true
 PersonalDataRequired: true
 price: FREE
---

## Comments

- Additional information to the email sending/recieving limits can be found on https://support.google.com/mail/answer/22839?hl=en

## Preparations

### Use OAuth (recomended)

When Deltachat asks you to use Oauth, accept and login in the google login that pops up.


### Without 2FA

Enable "less-secure-apps" to allow non google programms to connect to your email account. (It is recomended to use Outh instead)

### With 2FA

Create an "App Specific Passwort" for DeltaChat.