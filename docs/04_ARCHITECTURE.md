# Architecture Documentation

## System Overview

The BMI Calculator is a full-stack web application with a Rust backend and vanilla JavaScript frontend.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENT (Browser)                     │
│                                                             │
│  ┌────────────────────────────────────────────────────┐     │
│  │              HTML + CSS + JavaScript               │     │
│  │                                                    │     │
│  │  • Form inputs (weight, height)                    │     │
│  │  • Calculate button                                │     │
│  │  • Result display                                  │     │
│  │  • Error handling                                  │     │
│  └────────────────────────────────────────────────────┘     │
│                            │                                │
│                            │ HTTP/JSON                      │
└────────────────────────────┼────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                    SERVER (Rust + Axum)                     │
│                                                             │
│  ┌────────────────────────────────────────────────────┐     │
│  │                  Axum Router                       │     │
│  │                                                    │     │
│  │  GET  /          ──>  root_handler()               │     │
│  │  POST /api/calculate ──> calculate_bmi_handler()   │     │
│  └────────────────────────────────────────────────────┘     │
│                            │                                │
│                            ▼                                │
│  ┌────────────────────────────────────────────────────┐     │
│  │              Business Logic Layer                  │     │
│  │                                                    │     │
│  │  • validate_input()                                │     │
│  │  • calculate_bmi()                                 │     │
│  │  • categorize_bmi()                                │     │
│  └────────────────────────────────────────────────────┘     │
│                            │                                │
│                            ▼                                │
│  ┌────────────────────────────────────────────────────┐     │
│  │              Middleware Stack                      │     │
│  │                                                    │     │
│  │  • Tower CORS                                      │     │
│  │  • Tracing (structured logging)                    │     │
│  └────────────────────────────────────────────────────┘     │
│                            │                                │
│                            ▼                                │
│  ┌────────────────────────────────────────────────────┐     │
│  │              Tokio Async Runtime                   │     │
│  │                                                    │     │
│  │  • TCP listener (0.0.0.0:PORT)                     │     │
│  │  • Async task scheduling                           │     │
│  │  • Concurrent request handling                     │     │
│  └────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### Frontend Components

```
┌─────────────────────────────────────┐
│         index.html (embedded)       │
│                                     │
│  ┌───────────────────────────────┐  │
│  │        <head>                 │  │
│  │  • Meta tags                  │  │
│  │  • <style> (CSS)              │  │
│  └───────────────────────────────┘  │
│                                     │
│  ┌───────────────────────────────┐  │
│  │        <body>                 │  │
│  │                               │  │
│  │  ┌─────────────────────────┐  │  │
│  │  │   Container             │  │  │
│  │  │  • Title                │  │  │
│  │  │  • Subtitle             │  │  │
│  │  │  • Form                 │  │  │
│  │  │    - Weight input       │  │  │
│  │  │    - Height input       │  │  │
│  │  │    - Submit button      │  │  │
│  │  │  • Error display        │  │  │
│  │  │  • Result display       │  │  │
│  │  │    - BMI value          │  │  │
│  │  │    - Category           │  │  │
│  │  │    - Reference guide    │  │  │
│  │  └─────────────────────────┘  │  │
│  └───────────────────────────────┘  │
│                                     │
│  ┌───────────────────────────────┐  │
│  │       <script>                │  │
│  │  • Form submit handler        │  │
│  │  • Fetch API call             │  │
│  │  • Response rendering         │  │
│  │  • Error handling             │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

### Backend Components

```
┌─────────────────────────────────────────────────────┐
│                    main.rs                          │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │  Global Allocator (mimalloc)                  │  │
│  │  #[global_allocator]                          │  │
│  │  static GLOBAL: MiMalloc = MiMalloc;          │  │
│  └───────────────────────────────────────────────┘  │
│                      │                              │
│  ┌───────────────────────────────────────────────┐  │
│  │  Data Structures                              │  │
│  │  • struct BmiRequest { weight_kg, height_m }  │  │
│  │  • struct BmiResponse { bmi, category }       │  │
│  └───────────────────────────────────────────────┘  │
│                      │                              │
│  ┌───────────────────────────────────────────────┐  │
│  │  Core Functions                               │  │
│  │  • calculate_bmi(weight, height) -> f64       │  │
│  │  • categorize_bmi(bmi) -> &str                │  │
│  └───────────────────────────────────────────────┘  │
│                      │                              │
│  ┌───────────────────────────────────────────────┐  │
│  │  HTTP Handlers (async)                        │  │
│  │  • root_handler() -> Html                     │  │
│  │  • calculate_bmi_handler(Json) -> Result      │  │
│  └───────────────────────────────────────────────┘  │
│                      │                              │
│  ┌───────────────────────────────────────────────┐  │
│  │  Main Function                                │  │
│  │  • Initialize tracing                         │  │
│  │  • Build router                               │  │
│  │  • Configure middleware                       │  │
│  │  • Start TCP listener                         │  │
│  │  • Serve app                                  │  │
│  └───────────────────────────────────────────────┘  │
│                      │                              │
│  ┌───────────────────────────────────────────────┐  │
│  │  Tests Module                                 │  │
│  │  • test_calculate_bmi()                       │  │
│  │  • test_categorize_bmi()                      │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

## Request Flow

### GET / (Serve Frontend)

```
Client Browser
    │
    │ GET /
    ▼
Axum Router
    │
    │ Route match
    ▼
root_handler()
    │
    │ Return Html<String>
    ▼
Client Browser
    │
    │ Render HTML
    │ Load CSS
    │ Execute JavaScript
    ▼
Display Form
```

### POST /api/calculate (Calculate BMI)

```
Client Browser
    │
    │ Form submit
    │ Collect weight & height
    ▼
JavaScript fetch()
    │
    │ POST /api/calculate
    │ Content-Type: application/json
    │ Body: {"weight_kg": 70.0, "height_m": 1.75}
    ▼
Axum Router
    │
    │ Route match
    │ Deserialize JSON with Serde
    ▼
calculate_bmi_handler(Json<BmiRequest>)
    │
    ├─▶ Validate input
    │   └─▶ weight > 0 && height > 0
    │       ├─▶ Invalid: return Err(String)
    │       └─▶ Valid: continue
    │
    ├─▶ calculate_bmi(weight, height)
    │   └─▶ weight / (height * height)
    │
    ├─▶ categorize_bmi(bmi)
    │   └─▶ Match bmi range to category
    │
    ├─▶ Log event (structured)
    │
    └─▶ Return Ok(Json<BmiResponse>)
        │
        │ Serialize JSON with Serde
        ▼
Client Browser
    │
    │ Response received
    │ Parse JSON
    ▼
JavaScript updates DOM
    │
    ├─▶ Set BMI value
    ├─▶ Set category text
    └─▶ Show result div
```

## Data Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   User       │────>│   Browser    │────>│   Network    │
│   Input      │     │   Form       │     │   Request    │
└──────────────┘     └──────────────┘     └──────────────┘
  weight: 70kg                                    │
  height: 1.75m                                   │
                                                  ▼
                                        ┌──────────────────┐
                                        │  Axum Handler    │
                                        │  Deserialize     │
                                        └──────────────────┘
                                                  │
                                                  ▼
                                        ┌──────────────────┐
                                        │  BmiRequest      │
                                        │  {               │
                                        │    weight: 70.0  │
                                        │    height: 1.75  │
                                        │  }               │
                                        └──────────────────┘
                                                  │
                                                  ▼
                                        ┌──────────────────┐
                                        │  calculate_bmi() │
                                        │  70 / (1.75²)    │
                                        │  = 22.857        │
                                        └──────────────────┘
                                                  │
                                                  ▼
                                        ┌──────────────────┐
                                        │ categorize_bmi() │
                                        │  22.857 ∈        │
                                        │  [18.5, 25)      │
                                        │  = "Normal"      │
                                        └──────────────────┘
                                                  │
                                                  ▼
                                        ┌──────────────────┐
                                        │  BmiResponse     │
                                        │  {               │
                                        │    bmi: 22.857   │
                                        │    cat: "Normal" │
                                        │  }               │
                                        └──────────────────┘
                                                  │
                                                  ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   User       │<────│   Browser    │<────│   Network    │
│   Sees       │     │   Updates    │     │   Response   │
└──────────────┘     └──────────────┘     └──────────────┘
  BMI: 22.9                                  JSON payload
  Category: Normal
```

## Error Handling Flow

```
┌─────────────────────────────────────────┐
│           Invalid Input                 │
│  (weight ≤ 0 or height ≤ 0)             │
└─────────────────┬───────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  Log warning event  │
        │  (structured)       │
        └─────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  Return             │
        │  Err(String)        │
        └─────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  HTTP 400           │
        │  Bad Request        │
        └─────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  Client receives    │
        │  error message      │
        └─────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  Display error div  │
        │  with red styling   │
        └─────────────────────┘
```

## Middleware Stack

```
Request
  │
  ▼
┌─────────────────────────────┐
│     Tower CORS Layer        │
│  • Allow all origins (dev)  │
│  • Allow all methods        │
│  • Allow all headers        │
└─────────────┬───────────────┘
              │
              ▼
┌─────────────────────────────┐
│   Tower HTTP Trace Layer    │
│  (optional, for debugging)  │
└─────────────┬───────────────┘
              │
              ▼
┌─────────────────────────────┐
│      Axum Router            │
│  • Match route              │
│  • Deserialize body         │
└─────────────┬───────────────┘
              │
              ▼
┌─────────────────────────────┐
│     Handler Function        │
│  • Validate                 │
│  • Process                  │
│  • Log                      │
│  • Respond                  │
└─────────────┬───────────────┘
              │
              ▼
Response
```

## Logging Architecture

```
┌──────────────────────────────────────────┐
│          Tracing Subscriber              │
│  • Filter: bmi_calculator=info           │
│  • Format: JSON structured               │
└──────────────┬───────────────────────────┘
               │
               │ Collects events
               │
    ┌──────────┼──────────┐
    │          │          │
    ▼          ▼          ▼
┌────────┐ ┌────────┐ ┌────────┐
│ Startup│ │Calc OK │ │ Error  │
│ Event  │ │Event   │ │ Event  │
└────────┘ └────────┘ └────────┘

Event format:
{
  "name": "bmi.calculation.success",
  "level": "INFO",
  "fields": {
    "weight_kg": 70.0,
    "height_m": 1.75,
    "bmi": 22.857,
    "category": "Normal weight"
  },
  "message": "BMI calculated: {bmi}, category: {category}"
}
```

## Deployment Architecture (Heroku)

```
┌─────────────────────────────────────────────────────┐
│                  Internet                           │
└─────────────────────┬───────────────────────────────┘
                      │
                      │ HTTPS
                      ▼
┌─────────────────────────────────────────────────────┐
│              Heroku Router                          │
│  • Load balancing                                   │
│  • SSL termination                                  │
│  • Request routing                                  │
└─────────────────────┬───────────────────────────────┘
                      │
                      │ HTTP
                      ▼
┌─────────────────────────────────────────────────────┐
│               Dyno (Container)                      │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │         bmi_calculator binary                 │  │
│  │  • Binds to 0.0.0.0:$PORT                     │  │
│  │  • Handles requests                           │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  Environment:                                       │
│  • PORT=5432 (or dynamic)                           │
│  • RUST_LOG=info                                    │
└─────────────────────────────────────────────────────┘
```

## Performance Considerations

### Memory Layout

```
┌─────────────────────────────────────┐
│       Process Memory (~10MB)        │
│                                     │
│  ┌───────────────────────────────┐  │
│  │    Binary Code (~8MB)         │  │
│  │  • Rust runtime               │  │
│  │  • Tokio runtime              │  │
│  │  • Axum framework             │  │
│  └───────────────────────────────┘  │
│                                     │
│  ┌───────────────────────────────┐  │
│  │    Heap (~1MB)                │  │
│  │  • Request buffers            │  │
│  │  • JSON parsing               │  │
│  │  • Managed by mimalloc        │  │
│  └───────────────────────────────┘  │
│                                     │
│  ┌───────────────────────────────┐  │
│  │    Stack (~1MB)               │  │
│  │  • Tokio tasks                │  │
│  │  • Function frames            │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

### Async Task Execution

```
Tokio Runtime (multi-threaded)

Thread Pool:
┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐
│Thread│  │Thread│  │Thread│  │Thread│
│  1   │  │  2   │  │  3   │  │  4   │
└───┬──┘  └───┬──┘  └───┬──┘  └───┬──┘
    │         │         │         │
    └─────────┴────┬────┴─────────┘
                   │
                   ▼
         ┌─────────────────┐
         │   Task Queue    │
         │                 │
         │  Task 1: Req A  │
         │  Task 2: Req B  │
         │  Task 3: Req C  │
         │  Task 4: Req D  │
         └─────────────────┘
```

## Security Architecture

```
┌─────────────────────────────────────────┐
│           Security Layers               │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  Transport Security               │  │
│  │  • HTTPS (Heroku provides)        │  │
│  │  • TLS 1.2+                       │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  Input Validation                 │  │
│  │  • Type checking (Serde)          │  │
│  │  • Range validation               │  │
│  │  • Positive number checks         │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  CORS Policy                      │  │
│  │  • Configurable origins           │  │
│  │  • Permissive in dev              │  │
│  │  • Restrictive in prod            │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  Error Handling                   │  │
│  │  • No stack traces exposed        │  │
│  │  • User-friendly messages         │  │
│  │  • Structured logging             │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## Future Architecture (Planned)

```
┌─────────────────────────────────────────────────────┐
│                  Load Balancer                      │
└────────────────────┬────────────────────────────────┘
                     │
         ┌───────────┼───────────┐
         │           │           │
         ▼           ▼           ▼
    ┌────────┐  ┌────────┐  ┌────────┐
    │ Server │  │ Server │  │ Server │
    │   1    │  │   2    │  │   3    │
    └───┬────┘  └───┬────┘  └───┬────┘
        │           │           │
        └───────────┼───────────┘
                    │
                    ▼
            ┌───────────────┐
            │   PostgreSQL  │
            │   Database    │
            │  • User data  │
            │  • History    │
            └───────────────┘
```

---

**Architecture Version**: 1.0
**Last Updated**: 2025-12-02
**Status**: Production-ready
