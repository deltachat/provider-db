---
name: mail2tor
status: PREPARATION
domains:
- mail2tor.com
strict_tls: true
server:
- type: imap
  hostname: g77kjrad6bafzzyldqvffq6kxlsgphcygptxhnn4xlnktfgaqshilmyd.onion
  port: 143
  socket: PLAIN
- type: smtp
  hostname: xc7tgk2c5onxni2wsy76jslfsitxjbbptejnqhw6gy2ft7khpevhc7ad.onion
  port: 25
  socket: PLAIN
before_login_hint: |
  Tor is needed to connect to the email servers.
website: http://mail2torjgmxgexntbrmhvgluavhj7ouul5yar6ylbvjkxwqf6ixkwyd.onion/
# website: http://mail2tor.com/
last_checked: 2021-08-15
---

Email servers are only reachable over [Tor](https://www.torproject.org/).
