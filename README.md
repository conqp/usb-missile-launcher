# usb-missile-launcher

Rust library and terminal controllers for the **c-enter USB Missile Launcher**. The
workspace supports direct USB control from the computer attached to the launcher,
or remote control over a simple HTTP proxy.

> **Safety:** Keep people, animals, fragile objects, and the launcher’s firing path
> clear. Start with the launcher aimed at a safe area. The remote proxy has no
> authentication or access control; only expose it on a trusted network.

## Requirements

- A supported c-enter USB Missile Launcher (USB vendor ID `0416`, product ID
  `9391`) connected to the machine that runs the direct controller or server.
- A recent stable Rust toolchain (the workspace uses Rust edition 2024).
- Permission to access USB devices. Platform-specific driver or permission setup
  may be required by `nusb` on the host operating system.

Build every package:

```shell
cargo build --workspace
```

Run the available checks:

```shell
cargo fmt --all -- --check
cargo test --workspace
```

## Controllers

All terminal controllers use the same keys:

| Key | Action |
| --- | --- |
| Arrow keys | Move the launcher in that direction while held |
| Enter | Fire while held |
| Esc | Stop and quit |

### Direct USB control

Run the local terminal UI on the computer with the launcher attached:

```shell
cargo run -p uml-tui
```

The application opens the first USB device matching the supported vendor and
product IDs and sends control commands directly to interface `0`.

### Remote control

Remote control has three processes:

```text
terminal client  --HTTP POST-->  proxy  <--HTTP GET--  USB server  --> launcher
```

1. On the machine reachable by the client, start the proxy:

   ```shell
   cargo run -p remotectl-proxy
   ```

   Rocket’s default configuration listens on `127.0.0.1:8000`. Configure Rocket
   as needed when the proxy must be reachable from another host.

2. On the machine with the launcher attached, start the USB server and give it
   the proxy address as a positional argument:

   ```shell
   cargo run -p remotectl-srv -- 127.0.0.1:8000
   ```

   The server polls the proxy every 100 ms. If a request fails, it sends `Stop`
   to the launcher and retries after one second.

3. Start the remote terminal client, again with the proxy address:

   ```shell
   cargo run -p remotectl-clt -- 127.0.0.1:8000
   ```

   Pressing a control key stores the corresponding command in the proxy;
   releasing it stores `Stop`. Exiting with `Esc` also sends `Stop`.

## HTTP API

The proxy exposes a single resource at `/`:

| Request | Purpose |
| --- | --- |
| `POST /` with a JSON command | Set the current command |
| `GET /` | Read the current command as JSON |

Commands are JSON enum names: `"Stop"`, `"Left"`, `"Right"`, `"Up"`,
`"Down"`, and `"Fire"`.

For example, stop a launcher through a proxy on the local machine:

```shell
curl -X POST http://127.0.0.1:8000/ -H "Content-Type: application/json" -d '"Stop"'
```

The proxy initializes to `Stop`, but it retains its last command until another
command is posted. Ensure clients send `Stop` when finished.

## Library usage

The `uml` crate exposes the `MissileLauncher` trait for direct control. It
provides `left`, `right`, `up`, `down`, `fire`, and `stop` convenience methods.

```rust
use uml::{Device, MissileLauncher};

fn main() -> std::io::Result<()> {
    let mut launcher = <Device as MissileLauncher>::open()?;
    launcher.left()?;
    launcher.stop()?;
    Ok(())
}
```

`Command` can also be sent explicitly with `send_command` or
`send_command_with_timeout`.

## Workspace layout

| Package | Purpose |
| --- | --- |
| `uml` | USB device discovery and command transport library |
| `uml-tui` (`tui/`) | Direct local terminal controller |
| `remotectl-common` | Shared remote command type and proxy-address argument |
| `remotectl-proxy` | In-memory Rocket HTTP proxy |
| `remotectl-srv` | USB-side process that polls the proxy and controls the launcher |
| `remotectl-clt` | Remote terminal controller that posts commands to the proxy |

## License

Licensed under the [MIT License](LICENSE).
