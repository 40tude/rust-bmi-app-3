# BMI Calculator

A simple, modern web application for calculating Body Mass Index (BMI) using SI units (kilograms and meters).

## Features

- **Clean UI**: Modern, responsive interface with gradient design
- **Real-time Calculation**: Instant BMI calculation via REST API
- **WHO Standards**: BMI categorization following World Health Organization guidelines
- **Production Ready**: Built with Rust, Axum, and Tokio for performance and reliability
- **Structured Logging**: OpenTelemetry-compatible event logging
- **Optimized Performance**: Uses mimalloc allocator for improved throughput

## Architecture

- **Backend**: Axum web framework with Tokio async runtime
- **Frontend**: Vanilla JavaScript with modern CSS (no framework overhead)
- **API**: RESTful JSON endpoint for BMI calculations
- **Deployment**: Heroku-ready with Procfile and buildpack configuration

## Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo

## Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd bmi_calculator
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

4. Open your browser:
```
http://localhost:3000
```

## Usage

### Web Interface

1. Enter your weight in kilograms (e.g., 70.0)
2. Enter your height in meters (e.g., 1.75)
3. Click "Calculate BMI"
4. View your BMI value and health category

### API Endpoint

**POST** `/api/calculate`

Request body:
```json
{
  "weight_kg": 70.0,
  "height_m": 1.75
}
```

Response:
```json
{
  "bmi": 22.86,
  "category": "Normal weight"
}
```

### BMI Categories (WHO Standards)

| Category | BMI Range |
|----------|-----------|
| Underweight | < 18.5 |
| Normal weight | 18.5 - 24.9 |
| Overweight | 25.0 - 29.9 |
| Obese | ≥ 30.0 |

## Testing

Run the test suite:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## Development

### Code Structure

```
bmi_calculator/
├── src/
│   └── main.rs          # Main application with API handlers
├── Cargo.toml           # Dependencies and project metadata
├── Procfile             # Heroku process definition
├── RustConfig           # Heroku Rust version configuration
├── rust-toolchain.toml  # Rust toolchain specification
└── README.md            # This file
```

### Guidelines Compliance

This project follows Microsoft Rust Guidelines:

- **M-APP-ERROR**: Uses `anyhow` for application-level error handling
- **M-MIMALLOC-APPS**: Uses mimalloc as global allocator for performance
- **M-LOG-STRUCTURED**: Implements structured logging with named events
- **M-CANONICAL-DOCS**: Comprehensive documentation following Rust standards
- **M-CONCISE-NAMES**: Clear, descriptive naming without weasel words

### Logging

The application uses structured logging with named events. Set log level:

```bash
RUST_LOG=bmi_calculator=debug cargo run
```

## Deployment to Heroku

### Prerequisites

- Heroku CLI installed
- Heroku account

### Steps

1. Create a new Heroku app:
```bash
heroku create your-app-name
```

2. Set the buildpack:
```bash
heroku buildpacks:set emk/rust
```

3. Deploy:
```bash
git push heroku main
```

4. Open your app:
```bash
heroku open
```

### Environment Variables

The application automatically detects Heroku's `PORT` environment variable. No additional configuration needed.

## Performance

- **Allocator**: mimalloc for 15-25% performance improvement
- **Async Runtime**: Tokio for efficient concurrent request handling
- **Optimized Build**: LTO and codegen-units=1 for release builds
- **Zero-copy**: Efficient JSON serialization with serde

## License

MIT License

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Run clippy: `cargo clippy`
6. Format code: `cargo fmt`
7. Submit a pull request

## Troubleshooting

### Port Already in Use

If port 3000 is already in use, set a custom port:
```bash
PORT=8080 cargo run
```

### Build Errors

Ensure you're using Rust 2021 edition or later:
```bash
rustc --version
cargo --version
```

Update Rust if needed:
```bash
rustup update
```

## API Documentation

Generate and view API documentation:
```bash
cargo doc --open
```

## Credits

Built with:
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization
- [Tracing](https://github.com/tokio-rs/tracing) - Structured logging
- [Mimalloc](https://github.com/microsoft/mimalloc) - Performance allocator

---

**Note**: This is an educational project demonstrating modern Rust web development practices following Microsoft Rust Guidelines.
