---
name: hotmail / outlook / office365
website: https://outlook.live.com/mail
domains:
 - hotmail.com
 - outlook.com
 - office365.com
 - outlook.com.tr
credentials: emailPass
status:
 state: BROKEN
 date: 2019-02
---

> Detected to be not working any more, for details please check this [issue](https://github.com/deltachat/deltachat-core/issues/561)


## Comments
- **Outlook isn't compliant with the industry standards for email in header handling and is therefore incompatible with DeltaChat.**
- Owned by Microsoft

## Preparations
```
IMAP-Server: outlook.office365.com
Note: SMTP-Server can be omitted as outlook.office365.com is used (automatically by Delta Chat) and working. Other SMTP-Server that are working are smtp.office365.com 1 and smtp-mail.outlook.com 6.
```
