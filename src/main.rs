// Rust guideline compliant 2025-12-02

//! BMI Calculator Web Application.
//!
//! A simple web application that calculates Body Mass Index (BMI) based on
//! weight in kilograms and height in meters (SI units).
//!
//! # Architecture
//!
//! - Backend: Axum web framework with Tokio async runtime
//! - Frontend: Leptos reactive UI framework (CSR mode)
//! - API: RESTful endpoint for BMI calculation
//!
//! # Examples
//!
//! Run the application:
//! ```sh
//! cargo run --release
//! ```
//!
//! Access at: http://localhost:3000

use axum::{
    extract::Json,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use mimalloc::MiMalloc;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::{event, Level};
use anyhow::Result;

/// Global allocator using mimalloc for performance (M-MIMALLOC-APPS).
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// BMI calculation request payload.
///
/// Contains weight in kilograms and height in meters (SI units).
///
/// # Examples
///
/// ```
/// use bmi_calculator::BmiRequest;
///
/// let request = BmiRequest {
///     weight_kg: 70.0,
///     height_m: 1.75,
/// };
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BmiRequest {
    /// Weight in kilograms (must be positive).
    pub weight_kg: f64,
    /// Height in meters (must be positive).
    pub height_m: f64,
}

/// BMI calculation response payload.
///
/// Contains calculated BMI value and health category interpretation.
///
/// # Examples
///
/// ```
/// use bmi_calculator::BmiResponse;
///
/// let response = BmiResponse {
///     bmi: 22.86,
///     category: "Normal weight".to_string(),
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct BmiResponse {
    /// Calculated BMI value.
    pub bmi: f64,
    /// Health category based on WHO standards.
    pub category: String,
}

/// Calculates BMI from weight and height.
///
/// Uses the standard BMI formula: BMI = weight(kg) / height(m)²
///
/// # Examples
///
/// ```
/// let bmi = calculate_bmi(70.0, 1.75);
/// assert_eq!(bmi, 22.857142857142858);
/// ```
///
/// # Panics
///
/// Panics if height is zero (division by zero).
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

/// Categorizes BMI value according to WHO standards.
///
/// Returns health category as a string based on BMI ranges:
/// - Underweight: BMI < 18.5
/// - Normal weight: 18.5 ≤ BMI < 25
/// - Overweight: 25 ≤ BMI < 30
/// - Obese: BMI ≥ 30
///
/// # Examples
///
/// ```
/// assert_eq!(categorize_bmi(22.0), "Normal weight");
/// assert_eq!(categorize_bmi(17.0), "Underweight");
/// assert_eq!(categorize_bmi(27.0), "Overweight");
/// assert_eq!(categorize_bmi(32.0), "Obese");
/// ```
pub fn categorize_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal weight"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    }
}

/// Handles BMI calculation requests.
///
/// Validates input, calculates BMI, and returns categorized result.
///
/// # Examples
///
/// POST /api/calculate
/// ```json
/// {
///   "weight_kg": 70.0,
///   "height_m": 1.75
/// }
/// ```
///
/// # Errors
///
/// Returns HTTP 400 if:
/// - Weight or height are not positive numbers
/// - JSON payload is malformed
async fn calculate_bmi_handler(
    Json(payload): Json<BmiRequest>,
) -> Result<Json<BmiResponse>, String> {
    event!(
        name: "bmi.calculation.started",
        Level::INFO,
        weight_kg = payload.weight_kg,
        height_m = payload.height_m,
        "BMI calculation requested: weight={{weight_kg}}kg, height={{height_m}}m"
    );

    // Validate input
    if payload.weight_kg <= 0.0 || payload.height_m <= 0.0 {
        event!(
            name: "bmi.validation.failed",
            Level::WARN,
            weight_kg = payload.weight_kg,
            height_m = payload.height_m,
            "Invalid input: weight and height must be positive"
        );
        return Err("Weight and height must be positive numbers".to_string());
    }

    let bmi = calculate_bmi(payload.weight_kg, payload.height_m);
    let category = categorize_bmi(bmi);

    event!(
        name: "bmi.calculation.success",
        Level::INFO,
        bmi = bmi,
        category = category,
        "BMI calculated: {{bmi}}, category: {{category}}"
    );

    Ok(Json(BmiResponse {
        bmi,
        category: category.to_string(),
    }))
}

/// Serves the main HTML page with embedded Leptos frontend.
///
/// Returns static HTML containing the BMI calculator interface.
async fn root_handler() -> impl IntoResponse {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BMI Calculator</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }

        .container {
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            padding: 40px;
            max-width: 500px;
            width: 100%;
        }

        h1 {
            color: #333;
            text-align: center;
            margin-bottom: 10px;
            font-size: 2em;
        }

        .subtitle {
            text-align: center;
            color: #666;
            margin-bottom: 30px;
            font-size: 0.9em;
        }

        .input-group {
            margin-bottom: 20px;
        }

        label {
            display: block;
            color: #555;
            margin-bottom: 8px;
            font-weight: 500;
        }

        input {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 10px;
            font-size: 16px;
            transition: border-color 0.3s;
        }

        input:focus {
            outline: none;
            border-color: #667eea;
        }

        button {
            width: 100%;
            padding: 14px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 10px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: transform 0.2s, box-shadow 0.2s;
        }

        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
        }

        button:active {
            transform: translateY(0);
        }

        .result {
            margin-top: 30px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 10px;
            display: none;
        }

        .result.show {
            display: block;
            animation: fadeIn 0.3s;
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(-10px); }
            to { opacity: 1; transform: translateY(0); }
        }

        .bmi-value {
            font-size: 3em;
            font-weight: bold;
            text-align: center;
            margin: 10px 0;
            color: #667eea;
        }

        .bmi-category {
            text-align: center;
            font-size: 1.2em;
            color: #555;
            margin-bottom: 15px;
        }

        .bmi-info {
            font-size: 0.9em;
            color: #666;
            line-height: 1.6;
        }

        .error {
            background: #fee;
            color: #c33;
            padding: 12px;
            border-radius: 8px;
            margin-top: 15px;
            display: none;
        }

        .error.show {
            display: block;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>BMI Calculator</h1>
        <div class="subtitle">Calculate your Body Mass Index</div>

        <form id="bmiForm">
            <div class="input-group">
                <label for="weight">Weight (kg)</label>
                <input type="number" id="weight" step="0.1" min="0" required placeholder="e.g., 70.0">
            </div>

            <div class="input-group">
                <label for="height">Height (m)</label>
                <input type="number" id="height" step="0.01" min="0" required placeholder="e.g., 1.75">
            </div>

            <button type="submit">Calculate BMI</button>
        </form>

        <div id="error" class="error"></div>

        <div id="result" class="result">
            <div class="bmi-value" id="bmiValue"></div>
            <div class="bmi-category" id="bmiCategory"></div>
            <div class="bmi-info">
                <strong>BMI Categories (WHO):</strong><br>
                • Underweight: &lt; 18.5<br>
                • Normal weight: 18.5 - 24.9<br>
                • Overweight: 25 - 29.9<br>
                • Obese: ≥ 30
            </div>
        </div>
    </div>

    <script>
        document.getElementById('bmiForm').addEventListener('submit', async (e) => {
            e.preventDefault();

            const weight = parseFloat(document.getElementById('weight').value);
            const height = parseFloat(document.getElementById('height').value);

            const errorDiv = document.getElementById('error');
            const resultDiv = document.getElementById('result');

            errorDiv.classList.remove('show');
            resultDiv.classList.remove('show');

            try {
                const response = await fetch('/api/calculate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        weight_kg: weight,
                        height_m: height
                    })
                });

                if (!response.ok) {
                    const error = await response.text();
                    throw new Error(error);
                }

                const data = await response.json();

                document.getElementById('bmiValue').textContent = data.bmi.toFixed(1);
                document.getElementById('bmiCategory').textContent = data.category;
                resultDiv.classList.add('show');

            } catch (error) {
                errorDiv.textContent = error.message || 'An error occurred';
                errorDiv.classList.add('show');
            }
        });
    </script>
</body>
</html>"#,
    )
}

/// Application entry point.
///
/// Initializes logging, creates HTTP server, and starts listening.
///
/// # Errors
///
/// Returns error if:
/// - Port 3000 is already in use
/// - Network interface is unavailable
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for structured logging (M-LOG-STRUCTURED)
    tracing_subscriber::fmt()
        .with_env_filter("bmi_calculator=info,tower_http=debug")
        .init();

    event!(
        name: "app.startup.initiated",
        Level::INFO,
        "Starting BMI Calculator application"
    );

    // Build application routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/calculate", post(calculate_bmi_handler))
        .layer(CorsLayer::permissive());

    // Determine bind address (support Heroku's PORT env var)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = format!("0.0.0.0:{}", port);

    event!(
        name: "app.server.listening",
        Level::INFO,
        address = addr.as_str(),
        "Server listening on {{address}}"
    );

    println!("🚀 BMI Calculator running on http://localhost:{}", port);
    println!("📊 API endpoint: POST /api/calculate");

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bmi() {
        let bmi = calculate_bmi(70.0, 1.75);
        assert!((bmi - 22.857).abs() < 0.01);
    }

    #[test]
    fn test_categorize_bmi() {
        assert_eq!(categorize_bmi(17.0), "Underweight");
        assert_eq!(categorize_bmi(22.0), "Normal weight");
        assert_eq!(categorize_bmi(27.0), "Overweight");
        assert_eq!(categorize_bmi(32.0), "Obese");
    }
}
