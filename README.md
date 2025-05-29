🔐 Web3Auth-Rust: Web3 Authentication with Rust & Svelte
<div align="center">

Web3Auth-Rust
Ethereum
Svelte
TypeScript
Axum



Modern Web3 authentication and authorization system using Rust and Svelte


Login with Metamask. Cryptographic signature verification. Secure and full-stack.
🚀 Demo (#demo) •
📋 Features (#main-features) •
🛠️ Technology (#technology) •
🏁 Getting Started (#getting-started) •
👥 Contribution (#contribution)
</div>

🌟 Overview
Web3Auth-Rust is a complete Web3 authentication microservice implementation that allows users to authenticate using a Web3 wallet (Ethereum) without the need to store passwords.
🔥 Get modern authentication for your application with blockchain technology
✨ Main Features
🔒 Secure: Ethereum cryptographic signature-based authentication
🛡️ Nonce Challenge: Prevents replay attacks and ensures security
🎫 JWT: Stateless token-based authorization
⚡ High Performance: Fast and efficient Rust backend
🖥️ Modern UI: Svelte frontend with intuitive UX
📱 Responsive: Design works on all screen sizes
🛠️ Technology
Backend
Rust: Fast and reliable programming language
Axum: Modern and efficient Rust web framework
ethers-rs: Ethereum implementation in Rust
tokio: Asynchronous runtime for Rust
jsonwebtoken: JWT implementation for Rust
Frontend
Svelte: Fast and reactive UI framework
SvelteKit: Meta-framework for building Svelte applications
TypeScript: JavaScript with static typing for code safety
ethers.js: Comprehensive library for Ethereum interaction
🏗️ Project Structure
.
├── backend/            # 🦀 Rust server with Axum
│   ├── src/
│   │   ├── config/     # ⚙️ Application configuration
│   │   ├── middleware/ # 🔗 JWT middleware
│   │   ├── models/     # 📊 Data models
│   │   ├── routes/     # 🌐 API routes
│   │   ├── services/   # 🧩 Business logic
│   │   ├── utils/      # 🔧 Utility functions
│   │   └── main.rs     # 🚪 Entry point
│   └── Cargo.toml      # 📦 Rust dependencies
│
├── frontend/           # 🎨 Svelte UI
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/ # 🧱 UI Components
│   │   │   ├── services/   # 🔌 API and Web3 services
│   │   │   ├── stores/     # 🗄️ State management
│   │   │   └── types/      # 📝 TypeScript types
│   │   ├── routes/         # 📑 Application pages
│   │   └── app.css         # 🎨 Global styles
│   └── package.json        # 📦 NPM dependencies
│
└── README.md           # 📃 You're here
🚀 Getting Started
Prerequisites
Rust (>= 1.75.0)
Node.js (>= 18)
Metamask browser extension
🏃‍♂️ Running the Backend
Clone the repository:
bash
git clone https://github.com/badruzbby/web3auth-rust.git
cd web3auth-rust
Navigate to the backend directory:
bash
cd backend
Create a .env file (or use the existing one):
PORT=8080
RUST_LOG=info
JWT_SECRET=super_secure_long_jwt_secret_for_web3auth_microservice
Run the server:
bash
cargo run
The backend will run at http://localhost:8080 🎉
🖌️ Running the Frontend
Navigate to the frontend directory:
bash
cd frontend
Install dependencies:
bash
npm install
Run the development server:
bash
npm run dev
The frontend will run at http://localhost:5173 🎉
📡 API Endpoints
Method
Endpoint
Description
Auth
POST
/api/auth/nonce
Get a unique nonce
🚫
POST
/api/auth/verify
Verify signature
🚫
GET
/api/profile
Get user profile
✅
🔒 Authentication Flow
📝 Request Nonce: Frontend requests a nonce from the backend
👛 Signature: User signs a message (including nonce) with Metamask
✅ Verification: Backend verifies the signature and user identity
🎫 JWT Token: Backend generates and sends a JWT token to the frontend
🔐 Authorization: Frontend uses the JWT for authenticated API requests
🔧 Deployment
🐳 Using Docker
The fastest way to deploy:
bash
docker-compose up -d
Docker Compose file included in the repo!
☁️ Cloud Deployment
This project is compatible with:
AWS: ECS with Fargate or EC2
GCP: Cloud Run or Kubernetes Engine
Azure: Container Instances or Kubernetes Service
Heroku: Buildpacks available for Rust and Node.js
🤝 Contribution
Contributions are warmly welcomed! Here are some ways to contribute:
🐛 Report bugs or issues
💡 Suggest new features or improvements
📝 Fix or update documentation
🧩 Submit pull requests
Steps to contribute:
Fork this repository
Create a new feature branch (git checkout -b feature/amazing-feature)
Commit your changes (git commit -m 'Add an amazing feature')
Push to the branch (git push origin feature/amazing-feature)
Open a Pull Request
📄 License
This project is licensed under the MIT License. See the LICENSE file for details.
<div align="center">
Built with ❤️ by Muhammad Badruz Zaman, for Developers




<img src="https://img.shields.io/badge/Rust-FTW-orange?style=for-the-badge&logo=rust">
</div>

🧪 Technical Features for Developers
Zero Dependencies: Backend uses only essential libraries
Memory Safe: Rust's built-in memory safety prevents common security bugs
Async I/O: High performance with asynchronous concurrency model in the backend
Strongly Typed: TypeScript in the frontend prevents many development bugs
Compositional UI: Reusable and modular Svelte components
Web3 Ready: Ready to integrate with smart contracts and dApps
