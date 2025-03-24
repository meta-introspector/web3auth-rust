# 🔐 Web3Auth-Rust: Autentikasi Web3 dengan Rust & Svelte

<div align="center">

![Web3Auth-Rust](https://img.shields.io/badge/Web3Auth-Rust-orange?style=for-the-badge&logo=rust)
![Ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-Framework-black?style=for-the-badge)

<br>

**Sistem autentikasi dan otorisasi Web3 modern menggunakan Rust dan Svelte**
<br>
***Login dengan Metamask. Verifikasi tanda tangan kriptografi. Aman dan full-stack.***

[🚀 Demo](#demo) •
[📋 Fitur](#fitur-utama) •
[🛠️ Teknologi](#teknologi) •
[🏁 Mulai](#cara-memulai) •
[👥 Kontribusi](#kontribusi)

</div>

## 🌟 Ringkasan

**Web3Auth-Rust** adalah implementasi lengkap microservice autentikasi Web3 yang memungkinkan pengguna melakukan autentikasi menggunakan dompet Web3 (Ethereum) tanpa perlu menyimpan kata sandi.

> 🔥 **Dapatkan otentikasi modern untuk aplikasi Anda dengan teknologi blockchain**

## ✨ Fitur Utama

- 🔒 **Aman**: Autentikasi berbasis tanda tangan kriptografi Ethereum
- 🛡️ **Nonce Challenge**: Mencegah serangan replay dan menjamin keamanan
- 🎫 **JWT**: Token berbasis stateless untuk otorisasi
- ⚡ **Performa Tinggi**: Backend Rust yang cepat dan efisien
- 🖥️ **UI Modern**: Frontend Svelte dengan UX yang intuitif
- 📱 **Responsif**: Desain yang bekerja pada semua ukuran layar

## 🛠️ Teknologi

### Backend
- **[Rust](https://www.rust-lang.org/)**: Bahasa pemrograman yang cepat dan andal
- **[Axum](https://github.com/tokio-rs/axum)**: Framework web Rust modern dan efisien 
- **[ethers-rs](https://github.com/gakonst/ethers-rs)**: Implementasi Ethereum di Rust
- **[tokio](https://tokio.rs/)**: Runtime asinkron untuk Rust
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)**: Implementasi JWT untuk Rust

### Frontend
- **[Svelte](https://svelte.dev/)**: Framework UI reaktif yang cepat
- **[SvelteKit](https://kit.svelte.dev/)**: Framework meta untuk membangun aplikasi Svelte
- **[TypeScript](https://www.typescriptlang.org/)**: JavaScript dengan tipe statis untuk keamanan kode
- **[ethers.js](https://docs.ethers.io/)**: Library lengkap untuk interaksi dengan Ethereum

## 🏗️ Struktur Proyek

```
.
├── backend/            # 🦀 Server Rust dengan Axum
│   ├── src/
│   │   ├── config/     # ⚙️ Konfigurasi aplikasi
│   │   ├── middleware/ # 🔗 Middleware JWT
│   │   ├── models/     # 📊 Model data
│   │   ├── routes/     # 🌐 API routes
│   │   ├── services/   # 🧩 Business logic
│   │   ├── utils/      # 🔧 Utility functions
│   │   └── main.rs     # 🚪 Entry point
│   └── Cargo.toml      # 📦 Rust dependencies
│
├── frontend/           # 🎨 UI Svelte
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/ # 🧱 UI Components
│   │   │   ├── services/   # 🔌 API dan Web3 services
│   │   │   ├── stores/     # 🗄️ State management
│   │   │   └── types/      # 📝 TypeScript types
│   │   ├── routes/         # 📑 Halaman aplikasi
│   │   └── app.css         # 🎨 Global styles
│   └── package.json        # 📦 NPM dependencies
│
└── README.md           # 📃 Anda berada di sini
```

## 🚀 Cara Memulai

### Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (>= 1.75.0)
- [Node.js](https://nodejs.org/) (>= 18)
- [Metamask](https://metamask.io/) extension pada browser

### 🏃‍♂️ Menjalankan Backend

1. Clone repository:
   ```bash
   git clone https://github.com/badruzbby/web3auth-rust.git
   cd web3auth-rust
   ```

2. Masuk ke direktori backend:
   ```bash
   cd backend
   ```

3. Buat file `.env` (atau gunakan yang sudah ada):
   ```
   PORT=8080
   RUST_LOG=info
   JWT_SECRET=rahasia_jwt_web3auth_microservice_sangat_aman_dan_panjang
   ```

4. Jalankan server:
   ```bash
   cargo run
   ```

Backend akan berjalan di `http://localhost:8080` 🎉

### 🖌️ Menjalankan Frontend

1. Masuk ke direktori frontend:
   ```bash
   cd frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Jalankan server development:
   ```bash
   npm run dev
   ```

Frontend akan berjalan di `http://localhost:5173` 🎉

## 📡 API Endpoints

| Method | Endpoint | Deskripsi | Auth |
|--------|----------|-----------|------|
| `POST` | `/api/auth/nonce` | Mendapatkan nonce unik | 🚫 |
| `POST` | `/api/auth/verify` | Verifikasi tanda tangan | 🚫 |
| `GET`  | `/api/profile` | Mendapatkan profil pengguna | ✅ |

## 🔒 Alur Autentikasi

1. **📝 Request Nonce**: Frontend meminta nonce dari backend
2. **👛 Tanda Tangan**: User menandatangani pesan (termasuk nonce) dengan Metamask
3. **✅ Verifikasi**: Backend memverifikasi tanda tangan dan identitas pengguna
4. **🎫 Token JWT**: Backend menghasilkan dan mengirim token JWT ke frontend
5. **🔐 Otorisasi**: Frontend menggunakan JWT untuk API requests yang memerlukan auth

## 🔧 Deployment

### 🐳 Menggunakan Docker

Cara tercepat untuk deploy:

```bash
docker-compose up -d
```

Docker Compose file included in the repo!

### ☁️ Cloud Deployment

Proyek ini kompatibel dengan:

- **AWS**: ECS dengan Fargate atau EC2
- **GCP**: Cloud Run atau Kubernetes Engine
- **Azure**: Container Instances atau Kubernetes Service
- **Heroku**: Buildpacks tersedia untuk Rust dan Node.js

## 🤝 Kontribusi

Kontribusi sangat disambut! Berikut adalah beberapa cara untuk berkontribusi:

- 🐛 Laporkan bug atau masalah
- 💡 Usulkan fitur atau peningkatan baru
- 📝 Perbaiki atau perbarui dokumentasi
- 🧩 Kirimkan pull request

**Langkah-langkah untuk kontribusi:**

1. Fork repository ini
2. Buat branch fitur baru (`git checkout -b feature/amazing-feature`)
3. Commit perubahan Anda (`git commit -m 'Menambahkan fitur yang luar biasa'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buka Pull Request

## 📄 Lisensi

Proyek ini dilisensikan di bawah MIT License. Lihat file `LICENSE` untuk lebih detail.

---

<div align="center">
Dibuat dengan ❤️ oleh Muhammad Badruz Zaman, untuk Developer
<br>
<br>
<img src="https://img.shields.io/badge/Rust-FTW-orange?style=for-the-badge&logo=rust">
</div>

## 🧪 Fitur Teknis untuk Developer

- **Zero Dependencies**: Backend hanya menggunakan library yang sangat diperlukan
- **Memory Safe**: Keamanan memori bawaan dari Rust mencegah bug keamanan umum
- **Async I/O**: Performa tinggi dengan model konkurensi asinkron di backend
- **Strongly Typed**: TypeScript di frontend mencegah banyak bug dalam pengembangan
- **Compositional UI**: Komponen Svelte yang dapat digunakan kembali dan termodulasi
- **Web3 Ready**: Siap diintegrasikan dengan kontrak pintar dan dApp
