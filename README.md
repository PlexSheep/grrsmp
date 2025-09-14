<div align="center">

# 🦀 GRRSMP

**Great Routed Rust Secure Messaging Protocol**

_A peer-to-peer chat application with end-to-end encryption_

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust CI](https://github.com/PlexSheep/grrsmp/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/grrsmp/actions/workflows/cargo.yaml)

<!-- [![GitHub release](https://img.shields.io/github/v/release/PlexSheep/grrsmp)](https://github.com/PlexSheep/grrsmp/releases) -->

| Component                                              | API-Documentation                               | Description                          | Version                                                   |
| ------------------------------------------------------ | ----------------------------------------------- | ------------------------------------ | --------------------------------------------------------- |
| [📦 **`core`** ](https://crates.io/crates/grrsmp-core) | [📖 Documentation](https://docs.rs/grrsmp-core) | backend, networking and cryptography | ![Crates.io](https://img.shields.io/crates/v/grrsmp-core) |
| [📦 **`gtk`**](https://crates.io/crates/grrsmp-gtk)    | [📖 Documentation](https://docs.rs/grrsmp-gtk)  | GTK4-based desktop frontend          | ![Crates.io](https://img.shields.io/crates/v/grrsmp-gtk)  |

</div>

---

## 🌟 Introduction

GRRSMP is a modern, secure messaging protocol and chat-application written in
Rust. It prioritizes privacy, security,
and decentralization by implementing peer-to-peer connections with robust
end-to-end encryption. GRRSMP allows users to communicate directly with
each other, or alternatively over a hosted service that introduces users
and proxies messages in case a direct connection is not possible.

The protocol combines the security of Ed25519 cryptographic signatures for
identity verification with the noise protocol framework for transport security, ensuring that your
conversations remain private and authentic. All messages are end-to-end
encrypted with rotating keys.

GRRSMP is currently in development, and it's details are subject to
change. I started GRRSMP because i wanted to understand better how
cryptographically state-of-the-art secured messaging works.

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

### Core Architecture

- **Network Layer**: Plain old TCP
- **Transport Security**: [Noise Protocol Framework](https://noiseprotocol.org/): Authenticated Encryption tied to your cryptographic identity -> Perfect Forward Secrecy during Transport
- **Identity**: Ed25519 cryptographic signatures for user authentication, the only long-term secret
- **Encryption**: ChaCha20-Poly1305 for message end-to-end encryption

## 🚧 Project Status

**Current Status**: Development (Alpha)

GRRSMP is currently in development. The core protocol and basic P2P messaging
functionality are still being implemented, and the project is not yet ready
for real use.

### Finished Features

- 🆘 None lol

### Currently Working on these features

- 🔜 GTK4 user interface
- 🔜 Basic Peer-to-Peer connection establishment over the Noise Protocol Framework
- 🔜 Ed25519 identity generation and management
- 🔜 Asynchronous networking with Tokio

### Planned Features

- 🈳 Message serialization and storage
- 🈳 Identity exchange and trust verification UI
- 🈳 Message encryption and decryption
- 🈳 Group chat support
- 🈳 File transfer capabilities
- 🈳 Rendezvous server for NAT traversal
- 🈳 QR code connection sharing
- 🈳 Contact management and persistence

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

There is neither documentation nor tests at this time. 😁

### Contributing

Contributions are welcome! Please feel free to:

1. Report bugs and request features via [GitHub Issues](https://github.com/PlexSheep/grrsmp/issues)
2. Submit pull requests for improvements (please make an issue first if it's something larger, for coordination)
3. Help with documentation and testing

## 🌐 Network Documentation

### Protocol Overview

GRRSMP uses a layered security approach:

```
   Application Messages
      ↓
   Identity Layer (Ed25519 signatures)
      ↓
   E2EE Layer (ChaCha20-Poly1305)
      ↓
   Transport Layer (Noise Protocol Framework: Noise_XX_25519_ChaChaPoly_Blake2s)
      ↓
   Network Layer (TCP/IP)
```

### Connection Flow

1. **TCP Connection**: Standard TCP connection establishment
2. **Noise Handshake**: Ed25519 public keys are exchanged
3. **Trust Decision**: User decides whether to trust the remote identity
4. **Message Exchange**: Encrypted messages are sent over the secure channel with the ephemeral session key

### Port Configuration

- **Port Range**: Any available port
- **Protocol**: TCP with TLS 1.3

### NAT Traversal

Currently, GRRSMP requires manual port forwarding for connections across NATs. Future versions will include:

- Rendezvous server for connection brokering

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0** (AGPL-3.0).

This copyleft license ensures that:

- You can freely use, modify, and distribute this software
- Any derivative works must also be licensed under AGPL-3.0
- **Network services using this code must provide source code to users**
- The community benefits from all improvements

See the [LICENSE](LICENSE) file for the full license text.
