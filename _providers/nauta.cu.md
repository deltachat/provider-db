---
name: nauta.cu
domains:
  - nauta.cu
status: OK
server:
  - type: imap
    socket: STARTTLS
    hostname: imap.nauta.cu
    port: 143
  - type: smtp
    socket: STARTTLS
    hostname: smtp.nauta.cu
    port: 25
after_login_hint: |
    Atención - con nauta.cu, puede enviar mensajes sólo a un máximo de 20 personas a la vez.
    En grupos más grandes, no puede enviar mensajes o abandonar el grupo.
last_checked: 2020-01
website: https://webmail.nauta.cu/
---

