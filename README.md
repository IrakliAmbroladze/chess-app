# Real-Time Multiplayer Chess

A full-stack chess application built entirely in Rust, featuring real-time multiplayer gameplay with WebSockets, timers, and full chess rules validation.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Features

- **Real-time Multiplayer** - Play chess with anyone using WebSockets
- **Private Game Rooms** - Create or join games with unique room codes
- **Full Chess Rules** - Complete move validation including castling, en passant, and pawn promotion
- **Live Timers** - 10-minute countdown timer per player
- **Move History** - Track all moves in Standard Algebraic Notation (SAN)
- **Modern UI** - Responsive design with smooth animations
- **Board Rotation** - Black player automatically sees a flipped board
- **Pawn Promotion** - Interactive dialog for choosing promotion piece

## Technology Stack

### Frontend

- **[Leptos](https://leptos.dev/)** (v0.8)

### Backend

- **[Axum](https://github.com/tokio-rs/axum)** (v0.8)
- **[Tokio](https://tokio.rs/)** - Async runtime

### Deployment

- **[fly.io](https://fly.io)**
- **Docker**

## Quick Start

### 1. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### 2. Clone and Setup

```bash
git clone https://github.com/IrakliAmbroladze/chess-app.git
cd chess-app
```

### 3. Run Locally

**Terminal 1 - Frontend:**

```bash
trunk serve
```

**Terminal 2 - Backend:**

```bash
cargo run --bin server --features ssr
```

**Open your browser:**

```
http://localhost:8080
```

## Project Structure

```
chess-app/
├── src/
│   ├── main.rs              # Axum server & WebSocket handler
│   ├── lib.rs               # Leptos app entry point
│   ├── shared.rs            # Shared types (Client/Server messages)
│   ├── game.rs              # Chess game state & move validation
│   └── components/
│       ├── mod.rs           # Component exports
│       ├── home.rs          # Home page (create/join)
│       ├── game.rs          # Game page with WebSocket
│       └── board.rs         # Chess board component
├── Cargo.toml               # Rust dependencies
├── index.html               # HTML entry point
├── style.css                # Styling
├── Trunk.toml               # Trunk configuration
├── Dockerfile               # Production deployment
├── fly.toml                 # Fly.io configuration
└── README.md
```

## Contact

Created by [@IrakliAmbroladze](https://github.com/IrakliAmbroladze)

Project Link: [https://github.com/IrakliAmbroladze/chess-app](https://github.com/IrakliAmbroladze/chess-app)

Live Demo: [https://chess-app-irakli.fly.dev](https://chess-app-irakli.fly.dev/)

---

**Built with ❤️ and 🦀 Rust**
