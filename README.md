# Style Editor

A web-based style editor for finding, accessing, modifying and creating citation styles, designed to work in lockstep with the CSL Next ecosystem.

## Project Structure

- `server/`: Rust backend API server (using Axum) that interfaces with `csln_core` and `csln_processor`.
- `client/`: React + Vite frontend application with Tailwind CSS v4.

## Client (Frontend)

![Citation Style Editor Preview](client/public/screenshot.png)

The frontend is a modern React application designed for an intuitive, premium academic experience.

### Features
- **Style Hub**: Find and manage CSL styles.
- **Visual Wizard**: Step-by-step creation of new styles with live document preview.
- **Detailed Previews**: Comprehensive citation and bibliography verification.
- **Design Tokens**: Powered by Tailwind CSS v4 for a sleek, responsive UI.

### Running the Client
Prerequisites: Node.js (v18+).

```bash
cd client
npm install
npm run dev
```
Proxying is configured to forward `/api` requests to the local Rust server.


## Server

The server provides API endpoints to preview citations and bibliographies using the CSL Next processor.

### Running the Server

Prerequisites: Rust toolchain.

```bash
cd server
cargo run
```

The server listens on `127.0.0.1:3000`.

### API Endpoints

#### `POST /preview/citation`

Preview how a citation looks with a given style and references.

**Input:** JSON object with `style` and `references`.

```bash
curl -H "Content-Type: application/json" \
     -d @payload.json \
     http://127.0.0.1:3000/preview/citation
```

#### `POST /preview/bibliography`

Preview the bibliography output.

**Input:** JSON object with `style` and `references`.

```bash
curl -H "Content-Type: application/json" \
     -d @payload.json \
     http://127.0.0.1:3000/preview/bibliography
```

### Example Payload

See [server/payload.json](server/payload.json) for a complete example of the input format.

```json
{
  "style": { ... },
  "references": [ ... ]
}
```

Note: Currently, the API expects references in CSL-JSON format (legacy `Reference` type).
