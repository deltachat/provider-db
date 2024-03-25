---
 name: Ouvaton
 status: OK
 domains: 
   - ouvaton.org
 server:
   - type: imap
     socket: SSL
     hostname: imap.ouvaton.coop
     port: 993
     username_pattern: EMAIL
   - type: smtp
     socket: SSL
     hostname: smtp.ouvaton.coop
     port: 465
     username_pattern: EMAIL
 last_checked: 2022-12
 website: https://ouvaton.coop
---
