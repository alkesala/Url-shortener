# Rust URL Shortener

A minimalist URL shortener built with Rust and Actix-web. Converts long URLs into short, easy-to-share codes.

## Features

- Creates 6-character short codes
- RESTful API endpoints
- Docker support
- In-memory storage

## Setup

### Prerequisites

- Docker installed, or
- Rust and Cargo for local development

### Running with Docker

1. Pull the image:

```bash
docker pull yourusername/rust-url-shortener
```

2. Run container:

```bash
docker run -p 8080:8080 rust-url-shortener
```

### Local Development

1. Clone repository
2. Build and run:

```bash
cargo build --release
cargo run
```

## Usage

### Shorten a URL

```bash
curl -X POST -H "Content-Type: application/json" -d '{"url":"https://example.com"}' http://localhost:8080/shorten
```

Response:

```json
{
  "short_code": "abc123"
}
```

### Access Original URL

Visit in browser or use curl:

```bash
curl -L http://localhost:8080/abc123
```

## Docker Build

Build your own image:

```bash
docker build -t rust-url-shortener .
```

## API Reference

- `POST /shorten` - Create short URL
- `GET /{code}` - Redirect to original URL
