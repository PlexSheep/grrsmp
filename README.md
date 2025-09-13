<div align="center">

# 🦀 GRRSMP

**Great Routed Rust Secure Messaging Protocol**

_A peer-to-peer chat application with end-to-end encryption_

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![GitHub release](https://img.shields.io/github/v/release/PlexSheep/grrsmp)](https://github.com/PlexSheep/grrsmp/releases)
[![Rust CI](https://github.com/PlexSheep/grrsmp/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/grrsmp/actions/workflows/cargo.yaml)
[![Crates.io](https://img.shields.io/crates/v/grrsmp)](https://crates.io/crates/grrsmp)

[📖 Documentation](https://docs.rs/grrsmp-gtk) • [🐙 GitHub](https://github.com/PlexSheep/grrsmp) • [📦 Crates.io](https://crates.io/crates/grrsmp-gtk)

</div>

---

## 🌟 Introduction

GRRSMP is a modern, secure messaging application written in Rust. It
prioritizes privacy, security, and decentralization by implementing
peer-to-peer connections with robust end-to-end encryption. GRRSMP allows
users to communicate directly with each other, or alternatively over a hosted
service that introduces users and proxies messages in case a direct connection
is not possible.

The protocol combines the security of Ed25519 cryptographic signatures for
identity verification with TLS for transport security, ensuring that your
conversations remain private and authentic. All messages are end-to-end
encrypted with rotating keys.

## ✨ Features

- 🔐 **End-to-End Encryption**: All messages are encrypted with rotating keys
- 🌐 **Peer-to-Peer Architecture**: Direct connections between users, no central server required
- 🔑 **Cryptographic Identity**: Ed25519-based identity system with trust-on-first-use (TOFU), like SSH
- 🔒 **Forward Secrecy**: Message keys are rotated to protect past conversations
- 🚀 **Modern Asynchronous Runtime**: Built on Tokio for high-performance networking
- 🖥️ **Native GTK4 Interface**: Clean, responsive desktop application
- 🌍 **Not Cross-Platform**: Runs only on Linux, LOL

## 📦 Installation

### From Crates.io (Recommended)

```bash
cargo install --locked grrsmp-gtk
```

### From Source

```bash
git clone https://github.com/PlexSheep/grrsmp.git
cd grrsmp
cargo build --release --locked
./target/release/grrsmp-gtk
```

Technically, installation from crates.io is also from source, but you know what
I mean.

### Dependencies

#### Linux (Debian/Ubuntu)

```bash
sudo apt-get update
sudo apt-get install libgtk-4-dev build-essential pkg-config
```

## 📁 Crates

This project is organized as a multi-crate workspace:

| Crate             | Description                                                | Version                                                   |
| ----------------- | ---------------------------------------------------------- | --------------------------------------------------------- |
| **`grrsmp-core`** | Core protocol implementation, networking, and cryptography | ![Crates.io](https://img.shields.io/crates/v/grrsmp-core) |
| **`grrsmp-gtk`**  | GTK4-based desktop application interface                   | ![Crates.io](https://img.shields.io/crates/v/grrsmp-gtk)  |

### Core Architecture

- **Networking**: TLS 1.3 transport security over TCP with self-signed certificates
- **Identity**: Ed25519 cryptographic signatures for user authentication
- **Encryption**: AES-GCM for message end-to-end encryption

## 🚧 Project Status

**Current Status**: Development (Alpha)

GRRSMP is currently in development. The core protocol and basic P2P messaging
functionality are still being implemented, and the project is not yet ready
for real use.

### Implemented Features

- ✅ GTK4 user interface
- ✅ Basic Peer-to-Peer connection establishment
- ✅ Asynchronous networking with Tokio

### Planned Features

- 🔄 TLS transport security with custom certificate verification
- 🔄 Ed25519 identity generation and management
- 🔄 Message serialization and storage
- 🔄 Identity exchange and trust verification UI
- 🔄 Message encryption and decryption
- 🔄 Group chat support
- 🔄 File transfer capabilities
- 🔄 Rendezvous server for NAT traversal
- 🔄 QR code connection sharing
- 🔄 Contact management and persistence

## 🛠️ Development

### Prerequisites

- **Rust**: 1.85.1 or later (MSRV)
- **GTK4**: 4.10 or later
- **Platform**: Linux, windows and mac may work if you get the dependencies (mainly GTK4) to work

### Development Tools

```bash
# run unit tests
cargo test

# Generate documentation
cargo doc --open
```

### Contributing

Contributions are welcome! Please feel free to:

1. Report bugs and request features via [GitHub Issues](https://github.com/PlexSheep/grrsmp/issues)
2. Submit pull requests for improvements
3. Help with documentation and testing

## 🚀 Usage

### First Time Setup

1. **Launch the application**:

   ```bash
   grrsmp-gtk
   ```

2. **Create your identity**: On first launch, generate a unique Ed25519 key pair for your identity

3. **Connect to a peer**: Use the "Connection" menu to connect to another GRRSMP user via IP address and port

4. **Listen for incoming connections**: Use the "Connection" menu start the listener so that another user can connect to you

### Connecting to Others

**Direct Connection**: If you're on the same local network or have port forwarding configured:

```
Connection → Connect → Enter IP:Port (e.g., 192.168.1.100:51673)
```

**Trust Verification**: When connecting to someone new, you'll be shown their identity public key. Verify this through an alternative channel (phone, in person, etc.) before accepting.

## 🌐 Network Documentation

### Protocol Overview

GRRSMP uses a layered security approach:

```
   Application Messages
      ↓
   E2EE Layer (AES-256-GCM)
      ↓
   Identity Layer (Ed25519 signatures)
      ↓
   Transport Layer (TLS 1.3)
      ↓
   Network Layer (TCP/IP)
```

### Connection Flow

1. **TCP Connection**: Standard TCP connection establishment
2. **TLS Handshake**: TLS 1.3 with self-signed certificates containing identity keys
3. **Identity Exchange**: Ed25519 public keys are exchanged and verified
4. **Trust Decision**: User decides whether to trust the remote identity
5. **Message Exchange**: Encrypted messages are sent over the secure channel

### Port Configuration

- **Port Range**: Any available port
- **Protocol**: TCP with TLS 1.3

### NAT Traversal

Currently, GRRSMP requires manual port forwarding for connections across NATs. Future versions will include:

- UPnP automatic port mapping
- Rendezvous server for connection brokering

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0** (AGPL-3.0).

This copyleft license ensures that:

- You can freely use, modify, and distribute this software
- Any derivative works must also be licensed under AGPL-3.0
- **Network services using this code must provide source code to users**
- The community benefits from all improvements

See the [LICENSE](LICENSE) file for the full license text.
