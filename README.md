# kierand.dev

Personal website powered by Rust, Axum, and Tera.

## Stack

- Rust + Axum HTTP server
- Tera templates for SSR
- Static assets from `static/`
- Projects rendered from markdown in `content/projects.md` on every request

## Development

Run locally:

```bash
cargo run
```

Default address is `http://127.0.0.1:4173`.

Set a custom port:

```bash
PORT=4181 cargo run
```

## Build

```bash
cargo build --release
```

## Docker

Build and run:

```bash
docker build -t kierand-dev .
docker run --rm -p 4173:4173 kierand-dev
```
