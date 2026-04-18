# Rust Axum AWS Deployment Demo

A robust demo project showcasing the deployment of a high-performance Rust backend application on AWS infrastructure. This project leverages the [Axum](https://github.com/tokio-rs/axum) framework and the [Tokio](https://tokio.rs/) runtime to provide a scalable and efficient API template.

> The deployment of this project is documented as a tutorial on my blog. Follow this link to [read the tutorial](https://okpainmo.github.io/blog/categories/cloud-and-devops/end-to-end-rust-back-end-binary-deployment-on-aws-ec2).

![Blog Screenshot](/public/img-1.png)

## Project Overview

Despite being only a demo project, the codebase is packed with best practices that you can apply on production-grade rust codebases. It demonstrates:

- Standard environment-based configuration management.

- Structured logging and observability.

- Asynchronous request handling with Axum.

- Cloud-ready architectural patterns.

- More...

![Blog Screenshot](/public/img-2.png)

## Live API

The API server/binary is deployed and accessible at: **[https://api.xentoprotocol.xyz](https://api.xentoprotocol.xyz)**.

> This project is only meant to serve as a tutorial/guide. Hence, the live deployment via the above URL can become unavailable at any time.

## Getting Started

> Before anything, ensure that your machine is ready to run Rust programs. If not, follow the instructions on [Rust's official website](https://www.rust-lang.org/tools/install) to make the necessary installations first.

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

2. **Environment Variables Files Setup:**

    - Base Environment File

    ```bash
    cp .env.sample .env
    ```

    - Development Environment File

    ```bash
    cp .env.development.sample .env.development
    ```

    - Production Environment File

    ```bash
    cp .env.production.sample .env.production
    ```

3. **Run the Server:**

    For development with hot-reloading:

    ```bash
    cargo dev
    ```

    > `cargo dev` is an alias of the `cargo run` command. As defined in `.cargo/config.toml`, it integrates `cargo-watch` to provide hot-reloading while you're in dev mode.

    The API will be available at `http://localhost:8080`.

### Build & Deployment  

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

4. **Proceed to finish the binary deployment [following the steps in the tutorial](/blog/categories/cloud-and-devops/end-to-end-rust-back-end-binary-deployment-on-aws-ec2)**.

![Blog Screenshot](/public/img-3.png)
![Blog Screenshot](/public/img-4.png)
![Blog Screenshot](/public/img-5.png)

## API Documentation

To keep things straightforward and uncomplicated, the API exposes only three simple end-points.

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
