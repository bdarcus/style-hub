# CSL Style Editor

A modern, intent-based editor for Citation Style Language (CSL) files.

## Project Structure

- `server/`: Rust backend using Axum and a custom intent-based decision engine.
- `client/`: SvelteKit 5 frontend with Tailwind CSS 4.

## Features

- **Style Discovery**: Find and browse existing citation styles from the CSL repository.
- **Intent-Based Wizard**: Create new styles by answering simple questions about how you want your citations to look, rather than editing XML directly.
- **Live Preview**: Real-time rendering of citations and bibliographies as you make decisions.
- **CSL Export**: Download your finished style as a valid CSL XML file.

## Design Philosophy

The editor prioritizes **Visual Discovery**. Most users are looking to tweak an existing style. The Landing Page focuses on search and trending styles, with the **Creation Wizard** serving as a "Start from Scratch" option for advanced needs.

The interface uses a clean, premium "Paper" aesthetic for previews, providing an academic context for the design decisions.

## Development

### Running the Project

From the root directory:

```bash
# Start both client and server concurrently
npm run dev
```

### Backend (Rust)

```bash
cd server
cargo run
```

### Frontend (SvelteKit)

```bash
cd client
npm install
npm run dev
```

## Technology Stack

- **Backend**: Rust, Axum, Serde, Specta (for TS bindings).
- **Frontend**: Svelte 5, SvelteKit, Tailwind CSS 4, Lucide Svelte.
- **Deployment**: Optimized for standard SvelteKit adapters (Vercel, Node, etc.).
