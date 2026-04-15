# Rust Axum AWS Deployment Demo

A robust demonstration project showcasing the deployment of a high-performance Rust backend application on AWS infrastructure. This project leverages the [Axum](https://github.com/tokio-rs/axum) framework and the [Tokio](https://tokio.rs/) runtime to provide a scalable and efficient API template.

## Live URL

The project is deployed and accessible at: **[https://api.xentoprotocol.xyz](https://api.xentoprotocol.xyz)**

## Project Overview

This repository serves as a blueprint for deploying Rust services to AWS. It demonstrates best practices for:

- Asynchronous request handling with Axum.

- Environment-based configuration management.

- Structured logging and observability.

- Cloud-ready architectural patterns.

## Getting Started

> Before anything, ensure that you machine is setup to run Rust programs. If not, follow the instructions on the [Rust's official website](https://www.rust-lang.org/tools/install) to install Rust and get set.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)(latest stable)

- [cargo-watch](https://github.com/watchexec/cargo-watch)(recommended for development)

```bash
cargo install cargo-watch
```

### Local Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/Okpainmo/rust-axum-aws-deployment-demo.git
   cd rust-axum-aws-deployment-demo
   ```

2. **Configure Environment:**

    **Base Environment File**

    ```bash
    cp .env.sample .env
    ```

    **Development Environment File**

    ```bash
    cp .env.development.sample .env.development
    ```

3. **Run the Server:**

   For development with hot-reloading:

   ```bash
   cargo dev
   ```

   For a standard run:

   ```bash
   cargo run
   ```

   The API will be available at `http://127.0.0.1:8000`.

### Build For Deployment  

1. **Create Optimized Binary:**

   ```bash
   cargo build --release
   ```

2. **Run the Optimized Binary - Test Locally:**

   ```bash
   ./target/release/rust_axum_aws_deployment_demo
   ```

3. **Copy the binary and config setup to your server**

From your local machine:

E.g. 

```bash
scp -r config target/release/rust_axum_aws_deployment_demo <your-vm-username>@<your-vm-ip>:<desired-path-to-push-to>

# If you use a key:

scp -i ~/.ssh/your_key.pem -r config target/release/rust_axum_aws_deployment_demo <your-vm-username>@<your-vm-ip>:<desired-path-to-push-to>
```

4. Proceed to finish the binary deployment.

## API Documentation

> Postman collection for the API [is available here](/rust-axum-aws-deployment-demo.postman_collection.json)

### 1. Base API Status

- **Endpoint:** `/`

- **Method:** `GET`

- **Description:** Checks if the API is operational.

- **Success Response:**

  ```json
  {
    "message": "API is running"
  }
  ```

### 2. Health Check

- **Endpoint:** `/health`

- **Method:** `GET`

- **Description:** Can be used by load balancers or monitoring tools to verify the service's health.

- **Success Response:**

  ```json
  {
    "message": "healthy"
  }
  ```

### 3. Developer Information

- **Endpoint:** `/me`

- **Method:** `GET`

- **Description:** Returns information developer information.

- **Success Response:**

  ```json
  {
    "name": "Andrew James Okpainmo",
    "email": "okpainmoandrew@gmail.com",
    "github": "https://github.com/Okpainmo"
  }
  ```

## Tech Stack

- **Core:** `axum`, `tokio`

- **Serialization:** `serde`

- **Logging:** `tracing`, `tracing-subscriber`

- **Extras:** `config`, `dotenvy`, `anyhow`, `chrono`

- **Deployment Target:** AWS