[bots against war](https://github.com/bots-against-war), a tech-activist collective I'm part of,
uses [Redis](https://redis.io/) as a primary DB for all the web services and Telegram bots we
make. consequently, at some point we started thinking about data backup, given that our cloud
providers ask a bit too much money for this feature. so, we decided to implement
a makeshift alternative.

turns out, Redis' data format is quite simple and intuitive to work with, so, inspired by 
[a similar `go` library](https://github.com/yannh/redis-dump-go), i made a simple
script. i added a couple of quality-of-life features like error reporting to a dedicated
Telegram channel, metadata to keep track of how much data and backup script performance
we have, and an ability to run as a systemd service to not deal with `cron`.
