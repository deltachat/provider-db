# Delta Chat Provider Info


## Add an Entry
First check wether the file for the provider already exists, if it does skip to the **Edit an Entry** section.

Add [provider name].md to the _providers folder in the following format:
```
---
name: [provider name]
website: [ website of the provider / can also be yaml array when there are multiple ones]
credentials: [emailPass | emailAppPass | Oauth] - can also be an array, if multiple are supported availible
needsPreperation: [boolean, wether a preparation step is needed - for example enabling imap/smtp on provider page]
status:
   works: [boolean]
   date: [YYYY-MM]
---
## Comments
[If there are any]

## Preparations

[if the provider requires additional steps, describe them here, you may also include images or a video tutorial of the process here]

## status history
[if you change the status, copy the previous here that we can allow for a history]
[has a table format? that we can color code the states]
[just a link to github changelog/history of this file?]
```

## Edit an Entry

## Entries

Optional yaml font matter fields:
```
limits:
    maxRecipients: [number | NaN]

?registration?: [  inviteOnly | noPersonalData | PhoneAuth | PersonalData]

```
?idea?

## Example
```
---
name: example
website: example.com
credentials: emailPass |  emailAppPass | Oauth
needsPreperation: boolean
status:
 works: boolean
 date: YYYY-MM
---

## Comments
This is an example provider

## Preperations
```


##Ideas:

- optional registration type property
- repurpose websites into email domains? or include email domains otherwise? (remove protocol and www subdomain again? and make an extra property for the website)