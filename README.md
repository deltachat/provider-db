# Delta Chat Provider Info

This repository collects information on email providers and their interoperability with Delta Chat.

Parts of that information (the metadata, aka front matter) is included into the deltachat-core, to provides them to the Delta Chat apps on the different platforms.

The page's content is built into a web page that shows the status of the respective provider regarding its usage with Delta Chat, and details possibly required preparation steps, or explains why the interoperability is broken.


## Format

The files build on this format:

```yaml
---
name: [name of the provider]
status: [OK or PREPARATION or BROKEN]
domains: 
  - an_array
  - of_domains
  - used_by_this_provider
server:
  # Repeat the following block for each server (usually one for imap, one for smtp).
  - type: [IMAP or SMTP]
    socket: [SSL or STARTTLS]
    hostname: [hostname to connect to]
    port: [port number]
    username_pattern: [optional: EMAIL or EMAILLOCALPART, default is EMAIL]
before_login_hint: |
  [required for status PREPARATION or BROKEN: a string that will be displayed before the user logs in.
  Multiple lines are possible (line-breaks will be honoured), but keep in mind this text appears within the login form on possibly small displays.
  ]
after_login_hint: |
  [optional: a string that will be displayed in the device chat after the user logged in.
  Multiple lines are possible (line-breaks will be honoured).
  There's more room for text in the device chat than in the login form, but please keep the text concise nonetheless.
  ]
last_checked: [optional: date when the information was last checked: YYYY-MM]
website: [optional: website of the provider]
---
[Markdown-formatted content that gets displayed as provider-page on the web, linked from the apps (if status is not OK)]
```

## Status options:

### OK

If the status is `OK`, a standard text is used as page content. You don't need to put in anything.

### PREPARATION

This status means that the user must do some preparing steps before they can use Delta Chat with their provider. For example enabling IMAP/SMTP at their provider's settings, or creating an app-specific password.

The required steps must be described as page content in a friendly, helpful howto-style.

Additionally a short, informative sentence must be written as `before_login_hint`, so tech-savy users already know what to do, and others get an idea what to expect from the linked provider page.

### BROKEN

This status means that Delta Chat will not work with this provider.

The problems blocking the usage must be summarized as page content in a friendly tone.

Additionally a short, informative sentence must be written as `before_login_hint`, so tech-savy users already know what's up, and others get an idea what to expect from the linked provider page.

**Note:** Bear in mind the `before_login_hint` is displayed by the UIs as normal text, **without** html, markdown, whatever. Therefore, links in the `before_login_hint` are *not allowed*, especially as they tend to be wider than some smartphone displays.
