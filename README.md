# Minicargoft

[**English**](README.md) | [**Español**](README_es.md)

Aron Galdon Gines, 2026

Minicargoft is a basic Minecraft clone implemented with the purpose of comparing the performance and architecture of two different ecosystems: a web version using **JavaScript** and a native desktop version using **Rust**.

Both versions have the same Minimum Viable Product (MVP) features:

- Procedural terrain generation using Simplex noise.
- First-person camera and movement with collisions.
- Block destruction and placement using Raycasting.
- Different block types (Dirt, Grass, Stone).

---

## 🎮 Controls

| Action | Key / Button |
| :--- | :--- |
| Walk | `W` `A` `S` `D` |
| Sprint | Hold `Shift` |
| Jump | `Space` |
| Look | Mouse |
| Destroy Block | Left Click |
| Place Block | Right Click |
| Select Block | Keys `1`, `2`, `3` |

---

## 🌐 Web Version (JavaScript / Three.js)

This version runs directly in the browser, offering a zero-installation experience, ideal for quick play on any device.

### How to run JS version

1. Make sure you have `Node.js` installed.
2. Enter the JavaScript folder:

   ```bash
   cd js
   ```

3. Install dependencies and start the development server:

   ```bash
   npm install
   npm run dev
   ```

4. Open `http://localhost:5173` in your browser.

---

## 🦀 Native Version (Rust / Bevy Engine)

This version compiles to a native executable, taking full advantage of your graphics card. Written in Rust using the Bevy ECS engine.

### How to run Rust version

1. Make sure you have `Cargo` and `Rust` installed.
2. Enter the Rust folder:

   ```bash
   cd rust
   ```

3. Run the game in optimized (release) mode for maximum performance:

   ```bash
   cargo run --release
   ```

*(The first time you run it, it may take a while to compile the Bevy engine dependencies).*
