# Zync

Zync is a lightweight and straightforward WebDAV server
designed specifically for syncing Zotero collections within a local network.

If setting up NextCloud is a hassle, Zync is the solution.

## Installation

Follow these steps to install and set up Zync on your machine:

1. Clone the repository.

```bash
git clone https://github.com/she3o/zync.git
cd zync
```

2. Build the project:

```bash
cargo build --release
```

3. Run the Server

```bash
./target/release/zync --host <HOST> --port <PORT> --directory <DIRECTORY>
```

By default, Zync chooses `--host 0.0.0.0`, `--port 4918` and `--directory data`.

## Usage

1. Configure Zotero WebDAV:

  - Open Zotero and navigate to `Preferences` > `Sync`.

  - Select WebDAV as the synchronization method.

  - Enter the WebDAV URL in the format: `http://<IP_ADDRESS>:<PORT>`.

    - Replace `<IP_ADDRESS>` with the server's IP address within your local network.

    - Replace `<PORT>` with the port number you specified (default is `4918`).

2. Ensure Network Accessibility:

  - Make sure that the server machine is reachable from other devices within the local network.

  - If using `127.0.0.1` as the host, it will restrict access to the local machine only. Use the server's LAN IP address (or `0.0.0.0`) to allow other devices to connect.

## Future Plans

Zync is in its infancy, but here are some of the features I wish to implement:

- [x] Basic WebDAV Support
- [x] Argument parsing
- [x] Logging to TTY
- [ ] Logging To file
- [ ] Username/password authentication.
- [ ] Test suite
- [ ] Dynamic port binding
- [ ] SSL/TLS Support
- [ ] Sync Status Monitoring
- [ ] File Versioning and backup
- [ ] Systemd service
- [ ] Interactive mode
- [ ] Configuration file
- [ ] Docker container

## Contributing

All contributions are welcome! Please feel free to submit pull requests or open issues.

## License

Zync is licensed under the AGPL-3.0 License.
