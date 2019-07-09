---
name: Mailbox.org
website: 
- mailbox.org
- secure.mailbox.org
- (custom)
credentials: emailPass
needsPreperation: false
status:
 works: true
 date: 2019-03

limits:
    maxRecipients: Unlimited
    messagesPerDay: Unlimited

registration:
 inviteOnly: false
 PhoneAuthRequired: false
 PersonalDataRequired: false
 price: "&ge; 1.00€/mo"
---

## Comments:
- If you should reach some limits contact their support and they solve it for you [source](https://userforum.mailbox.org/topic/the-limits-for-your-account-are-exceeded#comment-14091)


## Using with Custom Domain

### DeltaChat Account settings
```
mailaddress - your extensions address
SMTP and IMAP login - main adress (not extentions address)
Server automated
Credentials: main address+password
```

### Seperate Mail from Chat
[Set Sieve rules for separate mail from chat - Tutorial](http://blog.lenzg.net/2019/02/using-delta-chat-with-email-sub-addresses/)