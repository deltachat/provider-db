---
name: Fastmail
status: PREPARATION
# Domain list from https://www.fastmail.com/about/ourdomains/
domains: 
  - 123mail.org
  - 150mail.com
  - 150ml.com
  - 16mail.com
  - 2-mail.com
  - 4email.net
  - 50mail.com
  - airpost.net
  - allmail.net
  - bestmail.us
  - cluemail.com
  - elitemail.org
  - emailcorner.net
  - emailengine.net
  - emailengine.org
  - emailgroups.net
  - emailplus.org
  - emailuser.net
  - eml.cc
  - f-m.fm
  - fast-email.com
  - fast-mail.org
  - fastem.com
  - fastemail.us
  - fastemailer.com
  - fastest.cc
  - fastimap.com
  - fastmail.cn
  - fastmail.co.uk
  - fastmail.com
  - fastmail.com.au
  - fastmail.de
  - fastmail.es
  - fastmail.fm
  - fastmail.fr
  - fastmail.im
  - fastmail.in
  - fastmail.jp
  - fastmail.mx
  - fastmail.net
  - fastmail.nl
  - fastmail.org
  - fastmail.se
  - fastmail.to
  - fastmail.tw
  - fastmail.uk
  - fastmail.us
  - fastmailbox.net
  - fastmessaging.com
  - fea.st
  - fmail.co.uk
  - fmailbox.com
  - fmgirl.com
  - fmguy.com
  - ftml.net
  - h-mail.us
  - hailmail.net
  - imap-mail.com
  - imap.cc
  - imapmail.org
  - inoutbox.com
  - internet-e-mail.com
  - internet-mail.org
  - internetemails.net
  - internetmailing.net
  - jetemail.net
  - justemail.net
  - letterboxes.org
  - mail-central.com
  - mail-page.com
  - mailandftp.com
  - mailas.com
  - mailbolt.com
  - mailc.net
  - mailcan.com
  - mailforce.net
  - mailftp.com
  - mailhaven.com
  - mailingaddress.org
  - mailite.com
  - mailmight.com
  - mailnew.com
  - mailsent.net
  - mailservice.ms
  - mailup.net
  - mailworks.org
  - ml1.net
  - mm.st
  - myfastmail.com
  - mymacmail.com
  - nospammail.net
  - ownmail.net
  - petml.com
  - postinbox.com
  - postpro.net
  - proinbox.com
  - promessage.com
  - realemail.net
  - reallyfast.biz
  - reallyfast.info
  - rushpost.com
  - sent.as
  - sent.at
  - sent.com
  - speedpost.net
  - speedymail.org
  - ssl-mail.com
  - swift-mail.com
  - the-fastest.net
  - the-quickest.com
  - theinternetemail.com
  - veryfast.biz
  - veryspeedy.net
  - warpmail.net
  - xsmail.com
  - yepmail.net
  - your-mail.com
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
# Technically supported, but very strongly discouraged by Fastmail    
#  - type: smtp
#    socket: STARTTLS
#    hostname: smtp.fastmail.com
#    port: 587
before_login_hint: "You must create an app-specific password for Delta Chat before you can log in."
last_checked: 2022-01
website: https://fastmail.com
---

To use Delta Chat with your Fastmail email address you have to generate a specific password for it.

You can do that in the Fastmail web interface at "Settings / Account: Password & Security / Third-Party-Apps: Add":

1. Enter your Password to be able to make changes. Then click "New App Password".
2. Choose a new name for the password.
3. At access, choose "Mail (IMAP/POP/SMTP)".
4. Click "Generate password".
5. Enter the 16-digit password shown there into the Delta Chat password field.
   **Careful** - leave out the spaces, it's just 16 digits. Better to copy-paste it, which will always paste without the spaces.

Fastmail supports [any of its domains](https://www.fastmail.com/about/ourdomains/) being used as an alias regardless of which you registered with, however the email and domain you picked during registration is the one they use for your "username".  If you are setting up an alias other than the email you originally registered at Fastmail with, you willl need to use "Advanced" during setup.

# Aliases and Custom Domains

The email address you're using with Delta.chat should be entered as the "email" address, while your Fastmail webmail login email address (the one you registered with originally) should be used as the "IMAP Login Name" and "SMTP Login Name".  Your "password" and "SMTP Password" should be the 16-digit password generated for you by Fastmail.  

For security, the SSL/SSL settings should be explicitly selected and used for both IMAP and SMTP, and "Strict" for the Certificate Checks.  Allowing Delta.chat to use "Automatic" for any of these risks an automatic security downgrade in the unlikely event an error is encountered.
