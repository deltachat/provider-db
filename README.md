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
  # If no servers are defined, autoconfig, autodiscover or guessing is used;
  # this will lead to the same server-configuration as if there is no provider-entry at all.
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
config_defaults:
  # optional, see below for details
  key: value
  other_key: other_value
last_checked: [optional: date when the information was last checked: YYYY-MM]
website: [optional: website of the provider]
---
[Markdown-formatted content that gets displayed as provider-page on the web, linked from the apps (if status is not OK)]
```

## Status options

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


## Configuration Defaults

Beside the server-configuration, Delta Chat has several other options
that can typically be set by the user at runtime.
In most cases the global-default for these options are fine for most providers,
however, if not, you have the possibility to define provider-specific-defaults
with the `config_defaults` section.

The api for that is a bit low-level: you have to define key-value-pairs
where the keys have to match the names used in the API, the values have to be
plain numeric values, see
[Delta Chat API](https://c.delta.chat/classdc__context__t.html#aff3b894f6cfca46cab5248fdffdf083d)
for details.

The provider-specific-defaults are applied _once_
after the first successful configuration,
they are not applied later on re-configures or on updates -
reason for that is to respect user-choice of changing these values.


## OAuth2

With the top-level option `oauth2=AUTHORIZER` you can specify,
that emails on the given domains support OAuth2 with the given authorizers.
Supported authorizers are `yandex` and `gmail`.

In contrast to other authorization methods, you cannot use oauth2
only because the server may support it.
New Oauth2 authorizers require adaptions in deltachat-core
and typically also bureaucratic effort.

### Use OAuth2 together with other options

If for an entered address, OAuth2 is supported,
and the used client supports OAuth2,
the user will be asked if he wants to continue with that.

Only if that is _cancelled_, the `before_login_hint` is shown;
so it is not needed to say sth. about OAuth2 before login.

All other options are applied as usual.
