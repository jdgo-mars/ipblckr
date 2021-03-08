# IP Blocker

## Block annoying ip's spaming your server with login attempts

## How does it work?

By watching changes on `/var/log/auth.log` and probing for new failed login attempts.
Whenever a threshold `t` is reached for a determined IP it will be blocked using IPTables.
