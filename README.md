# Zync

This is the simplest WebDAV server possible to sync Zotero collections.
I made it because writing a server from scratch is easier than setting up NextCloud.

Currently it works on my machine, within my local network, and syncs between my laptop and tablet.
I would be surprised if it worked for you.

## How to use it

- Clone repo
- `cargo run --release` inside repo.
- On each Zotero instance, point your WebDAV URL to `http://<IP ADDRESS>:4918`
  - Note: `http` not `https`

## Future Plans

If I (or you!) ever had time, here is a list of things I plan to do:

- [x] Basic WebDAV Support: `PROPFIND`, `GET`, `PUT`, `DELETE`.
- Add logging
  - To TTY
  - To log file
- Add username/password authentication.
- Add a test suite:
  - Integration tests
  - performance regression tests:
    - throughput
    - Memory
- Dynamic port binding
- SSL/TLS
- argument parsing:
  - `--data-dir`
  - `--port`
- Sync Status Monitoring
- File Versioning and backup
- Systemd service
- Interactive mode
  - `l`: Toggle logging
  - `q`: Quit
  - `?`: Show help
  - etc
- Configuration file
- Docker container

## Regrets

Maybe Rust was an overkill.
