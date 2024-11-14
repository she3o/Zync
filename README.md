# Zync

This is the simplest WebDAV server possible to sync Zotero collections.
I made it because writing a server from scratch is easier than setting up NextCloud.

Currently it works on my machine, within my local network, and syncs between my laptop and tablet.
I would be surprised if it worked for you.

## How to use it

- Clone repo
- `cargo build --release` inside repo.
- `./target/release/zync --host <HOST> --port <PORT>`
  - Without args, defaults to `0.0.0.0:4918`
  - Use `--host 127.0.0.1` to allow other devices to connect.
- On each Zotero instance, point your WebDAV URL to `http://<IP ADDRESS>:<PORT>`
  - On other devices, you don't reach the server with `127.0.0.1`.
  - Note: `http` not `https`

## Future Plans

If I (or you!) ever had time, here is a list of things I plan to do:

- [x] Basic WebDAV Support: `PROPFIND`, `GET`, `PUT`, `DELETE`.
- Add logging
  - [x] To TTY
  - To log file
- Add username/password authentication.
- Add a test suite:
  - Integration tests
  - performance regression tests:
    - throughput
    - Memory
- Dynamic port binding
- SSL/TLS
- [x] argument parsing
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
