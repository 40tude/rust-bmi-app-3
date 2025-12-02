# Rust Web Technologies Overview

This guide clarifies the different Rust tools and their specific purposes.



## **Tokio** - Async Runtime (The Foundation)

**What it does:**
- **Async runtime** for Rust that enables concurrent code execution
- Manages async/await tasks, timers, network I/O, etc.
- **Foundation** upon which Axum and Actix Web are built

**When to use it:**
- Almost always in production for async code
- Whenever you do networking, HTTP requests, databases

**Example:**
```rust
// In Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting...");

    // Run multiple tasks concurrently
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 done");
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 2 done");
    });

    // Wait for both
    let _ = tokio::join!(task1, task2);
}
```

---

## **Axum** - Modern Web Framework (Backend API)

**What it does:**
- Create **REST APIs**, GraphQL, WebSocket
- **Modern** and ergonomic web framework
- Built on Tokio + Tower (middleware)

**When to use it:**
- To create a **modern backend API**
- When you want strong type safety
- New projects (most modern option)

**Example:**
```rust
// In Cargo.toml: axum = "0.7", tokio = { version = "1", features = ["full"] }

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

// Simple GET handler
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        name: format!("User {}", id),
    })
}

// POST handler with JSON body
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        name: payload.name,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---
<!--
## **Actix Web** - High-Performance Web Framework (Backend API)

**What it does:**
- Also creates **backend APIs**
- One of the first mature Rust frameworks
- Architecture based on Actor pattern

**When to use it:**
- Existing projects already using it
- When you need extreme performance
- If you prefer its approach (more "traditional")

**Example:**
```rust
// In Cargo.toml: actix-web = "4"

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

// Handler functions
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let user = User {
        id: *path,
        name: format!("User {}", path),
    };
    HttpResponse::Ok().json(user)
}

async fn create_user(user: web::Json<CreateUser>) -> impl Responder {
    let new_user = User {
        id: 1,
        name: user.name.clone(),
    };
    HttpResponse::Created().json(new_user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Axum vs Actix Web:**
- Axum: more modern, better integration with Tower ecosystem
- Actix: more mature, slightly faster in benchmarks
- For a new project → **Axum** is recommended -->

---

## **Leptos** - Frontend Framework (like React/Vue)

**What it does:**
- Create **web user interfaces** (frontend)
- Compiles to **WebAssembly** to run in the browser
- Reactive framework with a signals system

**When to use it:**
- To create the **frontend** of your app
- Alternative to React/Vue but in Rust
- Can do SSR (Server-Side Rendering)

**Example:**
```rust
// In Cargo.toml: leptos = "0.6"

use leptos::*;

#[component]
fn App() -> impl IntoView {
    // Create a reactive signal
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <h1>"Counter App"</h1>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Increment"
            </button>
            <p>"Count: " {count}</p>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
```

---

## **Burn** - Deep Learning Framework (ML/AI)

**What it does:**
- Create and train **machine learning models**
- Alternative to PyTorch/TensorFlow but in Rust
- Supports CPU, CUDA, WebGPU, etc.

**When to use it:**
- To do **deep learning** in Rust
- Train neural networks
- Create ML models

**Example:**
```rust
// In Cargo.toml: burn = "0.14"

use burn::prelude::*;
use burn::nn;

#[derive(Module, Debug)]
pub struct SimpleModel<B: Backend> {
    linear1: nn::LinearConfig,
    linear2: nn::LinearConfig,
    activation: nn::Relu,
}

impl<B: Backend> SimpleModel<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(input);
        let x = self.activation.forward(x);
        self.linear2.forward(x)
    }
}

// Training code would go here...
```

---

## **Typical Full-Stack App Architecture**

Here's how these tools work together:

```
┌─────────────────────────────────────────┐
│         FRONTEND (browser)              │
│              LEPTOS                     │
│         (compiles to WASM)              │
└─────────────┬───────────────────────────┘
              │ HTTP/REST API
              ↓
┌─────────────────────────────────────────┐
│            BACKEND API                  │
│         AXUM or ACTIX WEB               │
│        (runs on TOKIO)                  │
└─────────────┬───────────────────────────┘
              │ (optional)
              ↓
┌─────────────────────────────────────────┐
│         MACHINE LEARNING                │
│              BURN                       │
│    (ML models for predictions)          │
└─────────────────────────────────────────┘
```

## **Summary**

| Tool | Category | Usage |
|------|----------|-------|
| **Tokio** | Async runtime | Foundation for all async code |
| **Axum** | Backend web | Modern REST API (recommended) |
| **Actix Web** | Backend web | Mature and fast REST API |
| **Leptos** | Frontend web | Browser UI (like React) |
| **Burn** | Machine Learning | Create ML/DL models |

**Common confusion:**
- Axum and Actix both create backend APIs (they are competitors)
- Tokio is used by Axum and Actix (low-level layer)
- Leptos is for frontend (browser side)
- Burn is completely separate for ML

For a typical fullstack Rust web project:
- **Backend**: Tokio + Axum
- **Frontend**: Leptos
- **(Optional) ML**: Burn

