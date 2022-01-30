---
name: Fastmail
status: PREPARATION
domains: 
  - fastmail.com
# Servers list from https://www.fastmail.help/hc/en-us/articles/1500000278342
server:
  - type: imap
    socket: SSL
    hostname: imap.fastmail.com
    port: 993
  - type: smtp
    socket: SSL
    hostname: smtp.fastmail.com
    port: 465
  - type: smtp
    socket: STARTTLS
    hostname: smtp.fastmail.com
    port: 587
before_login_hint: "You must create an app-specific password for Delta Chat before you can log in."
last_checked: 2020-01
website: https://fastmail.com
---

To use Delta Chat with your Fastmail email address
you have to generate a specific password for it.

You can do that in the Fastmail web interface
at "Settings / Account: Password & Security / Third-Party-Apps: Add":

1. Enter your Password to be able to make changes. Then click "New App Password".
2. Choose a new name for the password.
3. At access, choose "Mail (IMAP/POP/SMTP)".
4. Click "Generate password".
5. Enter the 16-digit password shown there into the Delta Chat password field.
   **Careful** - leave out the spaces, it's just 16 digits. Better copy-paste
   it, which will always paste without the spaces.

Fastmail supports [many different domains](https://www.fastmail.com/about/ourdomains/) for your webmail email address when you register an account with them, which confuses auto-detection of server settings.  If the webmail address you selected when registering with them is an @fastmail.com address, you can simply use your webmail login and the 16-digit Fastmail assigned password.  If you selected any other domain during registration or have a custom domain, you will need to use "Advanced" during setup.

# Setup using Advanced

The email address you're using with Delta.chat should be entered as the "email" address, while your Fastmail webmail login email address should be used as the "IMAP Login Name" and "SMTP Login Name".  Your "password" and "SMTP Password" should be the 16-digit password generated for you by Fastmail.  

For security, the SSL/TLS settings should be explicitly selected and used for both IMAP and SMTP, and "Strict" for the Certificate Checks.  STARTTLS is technically supported by Fastmail, but not recommended or as secure.  Allowing Delta.chat to use "Automatic" risks an automatic security downgrade if it encounters errors (which it shouldn't).
