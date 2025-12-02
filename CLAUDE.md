# CLAUDE.md - BMI Calculator Project

Project-specific guidance for Claude Code when working on this BMI Calculator application.

## Project Overview

**Name**: BMI Calculator
**Type**: Web Application (Rust + Axum + Tokio)
**Deployment**: Heroku
**Language**: All code/docs in US English

## Architecture Decisions

### Tech Stack
- **Backend**: Axum 0.7 + Tokio (async runtime)
- **Frontend**: Vanilla JS/HTML/CSS (embedded in main.rs)
- **Error Handling**: Anyhow (application-level)
- **Logging**: Tracing with structured events
- **Allocator**: Mimalloc (15-25% perf boost)

### Why No Leptos?
Initially planned but simplified to vanilla JS for:
- Smaller binary size (2.3MB vs 8MB+)
- Faster cold starts on Heroku
- Simpler deployment
- Educational clarity

## Microsoft Rust Guidelines Applied

This project strictly follows:

### M-APP-ERROR
- Uses `anyhow::Result` for application errors
- No custom error types (application-level)
- Proper error propagation with `?`

### M-MIMALLOC-APPS
```rust
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

### M-LOG-STRUCTURED
```rust
event!(
    name: "bmi.calculation.success",
    Level::INFO,
    bmi = bmi,
    category = category,
    "BMI calculated: {{bmi}}, category: {{category}}"
);
```

### M-CANONICAL-DOCS
- All public items documented
- Summary sentences < 15 words
- Examples, Errors, Panics sections
- Module-level docs present

### M-CONCISE-NAMES
- No Service/Manager/Factory suffixes
- Clear, descriptive names
- `calculate_bmi()` not `BmiCalculationService.calculate()`

## Code Organization

### Single File Structure (main.rs)
```
1. Module docs (lines 1-21)
2. Imports (lines 23-33)
3. Global allocator (lines 35-37)
4. Data structures (lines 39-81)
5. Business logic (lines 83-127)
6. HTTP handlers (lines 129-418)
7. Main function (lines 420-471)
8. Tests (lines 473-490)
```

**Why single file?**
- Simple project (< 500 LOC)
- Easy to navigate
- No cross-module complexity
- Educational clarity

## Build Configuration

### Cargo.toml Profile
```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Single codegen unit
strip = true           # Remove debug symbols
panic = "abort"        # Reduce runtime size
```

### .cargo/config.toml
```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

**Result**: Binary reduced from 2.96MB → 2.34MB

## Deployment Pipeline

### Heroku Setup
```bash
heroku create rust-bmi-app-3 --buildpack emk/rust
heroku git:remote -a rust-bmi-app-3
git push heroku main
```

### Key Files
- `Procfile`: `web: ./target/release/bmi_calculator`
- `RustConfig`: `VERSION=stable`
- `rust-toolchain.toml`: Rust stable channel
- `.slugignore`: Exclude docs/dev files

### Environment
- `PORT`: Dynamically assigned by Heroku
- `RUST_LOG`: Set to `bmi_calculator=info`

## API Design

### Endpoint: POST /api/calculate

**Request:**
```json
{
  "weight_kg": 70.0,
  "height_m": 1.75
}
```

**Response (Success):**
```json
{
  "bmi": 22.857142857142858,
  "category": "Normal weight"
}
```

**Response (Error):**
```
Status: 400 Bad Request
Body: "Weight and height must be positive numbers"
```

### Validation Rules
- `weight_kg > 0.0`
- `height_m > 0.0`
- Both must be finite f64 values

### BMI Categories (WHO)
- Underweight: < 18.5
- Normal: 18.5 - 24.9
- Overweight: 25.0 - 29.9
- Obese: ≥ 30.0

## Code Style

### Naming Conventions
- Functions: `snake_case` (e.g., `calculate_bmi`)
- Structs: `PascalCase` (e.g., `BmiRequest`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `GLOBAL`)
- Variables: `snake_case`

### Documentation Format
```rust
/// Brief summary < 15 words.
///
/// Extended description with context.
///
/// # Examples
///
/// ```
/// let bmi = calculate_bmi(70.0, 1.75);
/// assert_eq!(bmi, 22.857142857142858);
/// ```
///
/// # Errors
///
/// Returns error if...
///
/// # Panics
///
/// Panics if height is zero.
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
```

### Logging Format
```rust
// Good: structured with named event
event!(
    name: "module.operation.state",
    Level::INFO,
    property = value,
    "Message with {{property}} template"
);

// Bad: string formatting
tracing::info!("Message with {}", value);
```

## Testing Strategy

### Unit Tests
- Test pure functions (calculate_bmi, categorize_bmi)
- No network/IO mocking needed (stateless)
- Use `assert_eq!` for exact matches
- Use `assert!((a - b).abs() < epsilon)` for floats

### Manual Testing
```bash
# Local
cargo run --release
curl -X POST http://localhost:3000/api/calculate \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": 70.0, "height_m": 1.75}'

# Heroku
curl -X POST https://rust-bmi-app-3.herokuapp.com/api/calculate \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": 70.0, "height_m": 1.75}'
```

## Performance Targets

- Binary size: < 3MB (currently 2.34MB)
- Cold start: < 100ms
- API response: < 5ms
- Memory usage: < 20MB idle
- Concurrent requests: 1000+/sec

## Common Tasks

### Local Development
```bash
# Build
cargo build --release

# Run
cargo run --release

# Test
cargo test

# Format
cargo fmt

# Lint
cargo clippy
```

### Deployment
```bash
# Deploy
git push heroku main

# View logs
heroku logs --tail --app rust-bmi-app-3

# Restart
heroku restart --app rust-bmi-app-3

# Shell access
heroku run bash --app rust-bmi-app-3

# Check binary size
heroku run "ls -lh target/release/bmi_calculator" --app rust-bmi-app-3
```

### Debugging
```bash
# Local structured logs
RUST_LOG=bmi_calculator=debug cargo run

# Production logs
heroku logs --tail --app rust-bmi-app-3 | grep "bmi.calculation"

# Test specific endpoint
curl -v http://localhost:3000/api/calculate \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": -1, "height_m": 1.75}'
```

## Security Considerations

### Current State (Development)
- CORS: Permissive (all origins)
- HTTPS: Provided by Heroku
- Input validation: Type + range checks
- No authentication (stateless calculator)

### Production Recommendations
```rust
// Restrict CORS
let cors = CorsLayer::new()
    .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST]);

// Add rate limiting
use tower::limit::RateLimitLayer;
let rate_limit = RateLimitLayer::new(100, Duration::from_secs(60));

// Add request size limits (already default in Axum: 2MB)
```

## Known Issues & Limitations

### Current Limitations
- No BMI history tracking
- No user accounts
- Only SI units (kg/m)
- No age/gender considerations
- Single dyno (no load balancing)

### Future Enhancements
- [ ] PostgreSQL integration for history
- [ ] User authentication
- [ ] Imperial units support (lb/ft)
- [ ] BMI charts/visualizations
- [ ] Age/gender adjustments
- [ ] Export to PDF/CSV
- [ ] Mobile app (PWA)

## Troubleshooting

### Build Failures
```bash
# Clear cache
cargo clean

# Update dependencies
cargo update

# Check toolchain
rustc --version
cargo --version
```

### Heroku Deploy Fails
```bash
# Check buildpack
heroku buildpacks --app rust-bmi-app-3

# View build logs
heroku logs --tail --app rust-bmi-app-3

# Verify files
ls -la Procfile RustConfig rust-toolchain.toml
```

### Binary Too Large
- Enable `strip = true` in Cargo.toml
- Use `.slugignore` to exclude unnecessary files
- Consider `opt-level = "z"` for size optimization
- Use `target-cpu=native` in .cargo/config.toml

### Port Binding Errors
- Ensure code reads `PORT` env var
- Bind to `0.0.0.0:$PORT` not `127.0.0.1`
- Check Procfile command matches binary name

## Contact & Resources

**Heroku App**: https://rust-bmi-app-3.herokuapp.com
**Git Remote**: https://git.heroku.com/rust-bmi-app-3.git

**Documentation Files**:
- README.md - Complete documentation
- QUICKSTART.md - 2-minute getting started
- DEPLOYMENT.md - Heroku deployment guide
- PROJECT_SUMMARY.md - Technical overview
- ARCHITECTURE.md - System architecture diagrams
- CLAUDE.md - This file

**External Resources**:
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [Tokio Docs](https://docs.rs/tokio/latest/tokio/)
- [Microsoft Rust Guidelines](https://github.com/microsoft/rust-guidelines)
- [Heroku Rust Support](https://devcenter.heroku.com/articles/getting-started-with-rust)

---

**Last Updated**: 2025-12-02
**Project Status**: Production-ready
**Maintainer**: Educational project
