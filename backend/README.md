🦀 Web3 Authentication Backend with Rust
<div align="center">

Rust
Axum
JWT
Ethereum



High-performance Web3 authentication server with Rust and Axum


Cryptographic signature verification. JWT authorization. Top-tier security.
</div>

🌟 Overview
This backend handles Web3-based authentication using Ethereum wallet signatures, providing a secure and passwordless authentication solution.
🛠️ Technology
Rust ⚡ Fast and reliable programming language
Axum 🌐 Modern Rust web framework with high performance
ethers-rs 🔐 Ethereum library for signature verification
jsonwebtoken 🎫 JWT implementation for authorization
tokio ⚙️ Asynchronous runtime for concurrency
✨ Features
🔑 API endpoint for obtaining a unique nonce
✅ Secure Ethereum signature verification
🎟️ JWT system for authorization after login
🛡️ Authentication middleware for protected routes
🔒 Anti-replay attack protection using a nonce system
📝 Informative error handling
🌍 CORS handling for frontend
📂 Project Structure
src/
├── config/       # ⚙️ Application configuration (JWT, port, etc.)
├── middleware/   # 🔗 Middleware (auth)
├── models/       # 📊 Data models (User, Claims, etc.)
├── routes/       # 🌐 API endpoint handlers
├── services/     # 🧩 Business logic (auth service, etc.)
├── utils/        # 🔧 Error handling and utility functions
└── main.rs       # 🚪 Application entry point
🚀 Running the Application
Ensure Rust and Cargo are installed:
bash
rustc --version
cargo --version
Create a .env file in the root directory:
PORT=8080
RUST_LOG=info
JWT_SECRET=super_secure_long_jwt_secret_for_web3auth_microservice
Run the server:
bash
cargo run
For a release build:
bash
cargo build --release
./target/release/backend
📡 API Endpoints
📝 Nonce Request
http
POST /api/auth/nonce
Content-Type: application/json

{
  "wallet_address": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F"
}
Response (200 OK):
json
{
  "nonce": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Please sign this message for authentication: 550e8400-e29b-41d4-a716-446655440000"
}
✅ Verify Signature
http
POST /api/auth/verify
Content-Type: application/json

{
  "wallet_address": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",
  "signature": "0x...",
  "nonce": "550e8400-e29b-41d4-a716-446655440000"
}
Response (200 OK):
json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "wallet_address": "0x71c7656ec7ab88b098defb751b7401b5f6d8976f",
    "last_login": "2023-05-20T14:56:29.000Z",
    "created_at": "2023-05-18T09:12:43.000Z"
  }
}
👤 Get Profile (Authenticated)
http
GET /api/profile
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Response (200 OK):
json
{
  "wallet_address": "0x71c7656ec7ab88b098defb751b7401b5f6d8976f",
  "last_login": "2023-05-20T14:56:29.000Z",
  "created_at": "2023-05-18T09:12:43.000Z"
}
🔧 Deployment
🐳 Using Docker
Create a Dockerfile:
Dockerfile
FROM rust:1.75 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/backend .
COPY .env .
EXPOSE 8080
CMD ["./backend"]
Build and run the Docker image:
bash
docker build -t web3auth-backend .
docker run -p 8080:8080 web3auth-backend
<div align="center">
<img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust">


Part of the Web3Auth-Rust project
</div>

