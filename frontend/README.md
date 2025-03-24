# 🎨 Frontend Web3 Authentication dengan Svelte

<div align="center">

![Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white)
![TailwindCSS](https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)

<br>

**Antarmuka pengguna modern untuk autentikasi Web3 dengan Svelte**
<br>
*Pengalaman login Metamask yang mulus. Desain responsif. UX yang intuitif.*

</div>

## 🌟 Ringkasan

Frontend ini menyediakan antarmuka modern untuk autentikasi berbasis Web3 menggunakan Metamask, memberikan pengalaman login yang mulus dan terpercaya tanpa kata sandi.

<div align="center">
<img src="https://i.imgur.com/4XbJ9ZL.png" alt="Frontend Screenshot" width="600px"/>
</div>

## 🛠️ Teknologi

- **[Svelte](https://svelte.dev/)** ⚡ Framework UI reaktif yang cepat dan efisien
- **[SvelteKit](https://kit.svelte.dev/)** 🧩 Framework meta untuk aplikasi Svelte
- **[TypeScript](https://www.typescriptlang.org/)** 📝 JavaScript dengan tipe statis
- **[ethers.js](https://docs.ethers.io/)** 🔐 Library untuk interaksi Ethereum/Metamask
- **[Tailwind CSS](https://tailwindcss.com/)** 🎨 Styling modern dan responsif

## ✨ Fitur

- 🦊 Login dengan Metamask secara mulus
- ✒️ Tandatangan pesan untuk verifikasi keamanan
- 🔑 Penyimpanan JWT token yang aman
- 👤 UI status autentikasi yang informatif
- 📱 Desain responsif untuk semua perangkat 
- 🚨 Handling error yang user-friendly
- 🌐 Integrasi penuh dengan backend Rust

## 📂 Struktur Proyek

```
src/
├── lib/
│   ├── components/   # 🧱 Komponen UI (MetamaskButton, UserProfile, dll)
│   ├── services/     # 🔌 Service untuk API dan Web3
│   ├── stores/       # 🗄️ State management dengan Svelte stores
│   └── types/        # 📋 TypeScript type definitions
├── routes/
│   ├── +layout.svelte  # 📐 Layout aplikasi
│   └── +page.svelte    # 📄 Halaman utama
└── app.css           # 🎭 Style global
```

## 🧩 Komponen Utama

- 🔘 **MetamaskButton**: Tombol interaktif untuk login dengan Metamask
- 🚦 **AuthStatus**: Menampilkan status koneksi dan autentikasi
- 👤 **UserProfile**: Tampilan profil pengguna setelah login

## 🚀 Menjalankan Aplikasi

1. **Pastikan Node.js dan npm/yarn terinstal:**
   ```bash
   node --version
   npm --version
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Jalankan server development:**
   ```bash
   npm run dev
   ```

4. **Untuk build versi produksi:**
   ```bash
   npm run build
   ```

## 🎮 Cara Penggunaan

1. **Buka aplikasi** di browser (pastikan Metamask terinstal)
2. **Klik tombol** "Login dengan Metamask"
3. **Izinkan permintaan koneksi** di popup Metamask
4. **Tanda tangani pesan** yang muncul di Metamask
5. **Nikmati akses** - Anda akan melihat profil dengan alamat wallet Anda

<div align="center">
<img src="https://i.imgur.com/k4zMVrQ.gif" alt="Login Flow Animation" width="500px"/>
</div>

## 🔧 Deployment

### 🌍 Menggunakan Server HTTP

1. **Build aplikasi:**
   ```bash
   npm run build
   ```

2. **Deploy folder** `build` ke server HTTP seperti Nginx atau Vercel.

### 🐳 Menggunakan Docker

1. **Buat Dockerfile:**
   ```Dockerfile
   FROM node:18-alpine as build
   WORKDIR /app
   COPY package*.json ./
   RUN npm ci
   COPY . .
   RUN npm run build
   
   FROM nginx:alpine
   COPY --from=build /app/build /usr/share/nginx/html
   COPY nginx.conf /etc/nginx/conf.d/default.conf
   EXPOSE 80
   CMD ["nginx", "-g", "daemon off;"]
   ```

2. **Buat file nginx.conf:**
   ```nginx
   server {
     listen 80;
     root /usr/share/nginx/html;
     index index.html;
     
     # SPA config
     location / {
       try_files $uri $uri/ /index.html;
     }
     
     # API proxy untuk backend
     location /api {
       proxy_pass http://backend:8080/api;
       proxy_set_header Host $host;
       proxy_set_header X-Real-IP $remote_addr;
     }
   }
   ```

3. **Build dan jalankan Docker image:**
   ```bash
   docker build -t web3auth-frontend .
   docker run -p 80:80 web3auth-frontend
   ```

## ⚙️ Konfigurasi

Untuk mengubah URL API backend, edit file `src/lib/services/api.ts` dan ubah konstanta `API_URL`.

---

<div align="center">
<img src="https://img.shields.io/badge/Made%20with-Svelte-FF3E00?style=for-the-badge&logo=svelte">
<br>
Bagian dari proyek Web3Auth-Rust
</div>
