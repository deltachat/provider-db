---
name: Add Provider
about: Your Provider isn't on providers.delta.chat yet? Add it here.

---

```
---
name: Example
website: https://example.org
domains: 
 - example.org 
 - example.com
credentials: emailPass | emailAppPass | OAuth2   <!-- delete those that do not fit :) -->
status:
 state: OK | PREP | BROKEN  <!-- does it work out of the box, do you need preparations, or doesn't it work at all with Delta? -->
 date: 2017-06
registration:
 inviteOnly: true | false
 price: FREE | freemium | ≥ 5.00€ / month
 PersonalDataRequired: true | false  <!-- do you need to give an address at registration? -->
 PhoneAuthRequired: true | false  <!-- do you need to give a phone number at registration? -->
 
---

## Comments

Anything else you need to know about this provider?

## Preparations

If you don't any preparations for this provider, delete this. Otherwise, please explain it for the others :)

```
