# 🦀 Backend Web3 Authentication dengan Rust

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-Framework-black?style=for-the-badge)
![JWT](https://img.shields.io/badge/JWT-000000?style=for-the-badge&logo=JSON%20web%20tokens&logoColor=white)
![Ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white)

<br>

**Server autentikasi Web3 berperforma tinggi dengan Rust dan Axum**
<br>
*Verifikasi tanda tangan kriptografi. Otorisasi JWT. Keamanan terdepan.*

</div>

## 🌟 Ringkasan

Backend ini menangani autentikasi berbasis Web3 menggunakan tanda tangan dompet Ethereum, memberikan solusi autentikasi yang aman dan tanpa kata sandi.

<div align="center">
<img src="https://i.imgur.com/dDkxVrm.png" alt="Backend Flow Diagram" width="600px"/>
</div>

## 🛠️ Teknologi

- **[Rust](https://www.rust-lang.org/)** ⚡ Bahasa pemrograman yang cepat dan andal
- **[Axum](https://github.com/tokio-rs/axum)** 🌐 Framework web Rust modern dengan performa tinggi
- **[ethers-rs](https://github.com/gakonst/ethers-rs)** 🔐 Library Ethereum untuk verifikasi tanda tangan
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)** 🎫 Implementasi JWT untuk otorisasi
- **[tokio](https://tokio.rs/)** ⚙️ Runtime asinkron untuk konkurensi

## ✨ Fitur

- 🔑 API endpoint untuk mendapatkan nonce unik
- ✅ Verifikasi tanda tangan Ethereum yang aman
- 🎟️ Sistem JWT untuk otorisasi setelah login
- 🛡️ Middleware autentikasi untuk route yang dilindungi
- 🔒 Proteksi anti-replay attack menggunakan sistem nonce
- 📝 Error handling yang informatif
- 🌍 CORS handling untuk frontend

## 📂 Struktur Proyek

```
src/
├── config/       # ⚙️ Konfigurasi aplikasi (JWT, port, dll)
├── middleware/   # 🔗 Middleware (auth)
├── models/       # 📊 Model data (User, Claims, dll)
├── routes/       # 🌐 Endpoint handler API
├── services/     # 🧩 Logika bisnis (auth service, dll)
├── utils/        # 🔧 Error handling dan fungsi utilitas
└── main.rs       # 🚪 Entry point aplikasi
```

## 🚀 Menjalankan Aplikasi

1. **Pastikan Rust dan Cargo terinstal:**
   ```bash
   rustc --version
   cargo --version
   ```

2. **Buat file `.env` di root direktori:**
   ```
   PORT=8080
   RUST_LOG=info
   JWT_SECRET=rahasia_jwt_web3auth_microservice_sangat_aman_dan_panjang
   ```

3. **Jalankan server:**
   ```bash
   cargo run
   ```

   Untuk build versi rilis:
   ```bash
   cargo build --release
   ./target/release/backend
   ```

## 📡 API Endpoints

### 📝 Nonce Request
```http
POST /api/auth/nonce
Content-Type: application/json

{
  "wallet_address": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F"
}
```

**Response** (200 OK):
```json
{
  "nonce": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Silakan tanda tangani pesan ini untuk autentikasi: 550e8400-e29b-41d4-a716-446655440000"
}
```

### ✅ Verify Signature
```http
POST /api/auth/verify
Content-Type: application/json

{
  "wallet_address": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",
  "signature": "0x...",
  "nonce": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response** (200 OK):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "wallet_address": "0x71c7656ec7ab88b098defb751b7401b5f6d8976f",
    "last_login": "2023-05-20T14:56:29.000Z",
    "created_at": "2023-05-18T09:12:43.000Z"
  }
}
```

### 👤 Get Profile (Authenticated)
```http
GET /api/profile
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response** (200 OK):
```json
{
  "wallet_address": "0x71c7656ec7ab88b098defb751b7401b5f6d8976f",
  "last_login": "2023-05-20T14:56:29.000Z",
  "created_at": "2023-05-18T09:12:43.000Z"
}
```

## 🔧 Deployment

### 🐳 Menggunakan Docker

1. **Buat Dockerfile:**
   ```Dockerfile
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
   ```

2. **Build dan jalankan Docker image:**
   ```bash
   docker build -t web3auth-backend .
   docker run -p 8080:8080 web3auth-backend
   ```

---

<div align="center">
<img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust">
<br>
Bagian dari proyek Web3Auth-Rust
</div> 