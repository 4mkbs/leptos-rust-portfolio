# 🦀 Rust Leptos Developer Portfolio

A modern, blazing fast, reactive full-stack portfolio website built purely in **Rust**, **Leptos** (CSR), and **WebAssembly**, compiled with **Trunk**.

---

## ⚡ Features

- **Near-Zero JS Overhead**: Entire UI logic and reactivity rendered by Rust running as WebAssembly.
- **Ultra-Lightweight**: Production WASM binary is only ~229 KB after `wasm-opt` optimization.
- **Reactive State**: Built with Leptos 0.7 fine-grained reactive signals (`RwSignal`).
- **Interactive WASM Benchmark**: Built-in in-browser benchmark demo calculating Fibonacci/wrapping kernel directly in WebAssembly.
- **Tailwind CSS Styling**: Dark mode by default, modern glassmorphic accents, glowing badges, responsive grid.
- **Filterable Projects**: Dynamic project filtering by category (Rust & WASM, Go Systems, Python & Django).
- **Interactive Contact Form**: Pure Leptos reactive form with instant feedback state.

---

## 🚀 Quick Start & Development

### 1. Run Development Server (with Auto-Reload)
From the project directory:
```bash
cd /home/khairul/portfolio-leptos
trunk serve --open
```
This starts the local web server at `http://127.0.0.1:8080` with automatic hot-reloading on code edits.

### 2. Build for Production
```bash
cd /home/khairul/portfolio-leptos
trunk build --release
```
The optimized production output will be generated inside the `dist/` directory, ready to be deployed to GitHub Pages, Cloudflare Pages, Vercel, or any static host.

---

## 📁 Project Structure

```
portfolio-leptos/
├── Cargo.toml          # Rust crate configuration & dependencies
├── index.html          # HTML entry point with Tailwind CDN & Trunk hook
├── src/
│   └── main.rs         # Leptos App components, signals & UI logic
├── dist/               # Production WebAssembly & HTML distribution assets
└── README.md
```
