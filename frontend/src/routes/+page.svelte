<script lang="ts">
    import { MetamaskButton, UserProfile, AuthStatus } from '$lib/components/index.js';
    import { isLoggedIn } from '$lib/stores/auth.js';
</script>

<div class="container">
    <header>
        <h1>Web3 Authentication Demo</h1>
    </header>
    
    <div class="card">
        <h2>Status Autentikasi</h2>
        <AuthStatus />
        <MetamaskButton />
    </div>
    
    {#if $isLoggedIn}
        <div class="card">
            <UserProfile />
        </div>
    {/if}
    
    <div class="card info">
        <h2>Tentang Aplikasi</h2>
        <p>
            Aplikasi ini adalah demonstrasi dari sistem autentikasi Web3 menggunakan Metamask.
            Pengguna dapat masuk dengan menandatangani pesan menggunakan dompet Ethereum mereka,
            tanpa perlu kata sandi tradisional.
        </p>
        <h3>Bagaimana Cara Kerjanya:</h3>
        <ol>
            <li>Klik tombol <strong>Login dengan Metamask</strong> di atas.</li>
            <li>Konfirmasikan permintaan koneksi di ekstensi Metamask.</li>
            <li>Tanda tangani pesan di Metamask untuk verifikasi identitas Anda.</li>
            <li>Server akan memverifikasi tanda tangan Anda dan memberikan token JWT.</li>
            <li>Anda akan melihat informasi profil Anda setelah login berhasil.</li>
        </ol>
        <p class="note">
            <strong>Catatan:</strong> Aplikasi ini berjalan pada jaringan tes Ethereum.
            Tidak ada biaya gas fee yang diperlukan untuk menggunakannya.
        </p>
    </div>
</div>

<style>
    header {
        margin-bottom: 2rem;
        text-align: center;
    }
    
    .info {
        background-color: #f8f9fa;
    }
    
    h3 {
        margin: 1rem 0 0.5rem 0;
        font-size: 1.2rem;
    }
    
    ol {
        margin-left: 1.5rem;
        margin-bottom: 1rem;
    }
    
    li {
        margin-bottom: 0.5rem;
    }
    
    .note {
        font-size: 0.9rem;
        color: #666;
        margin-top: 1rem;
        border-left: 4px solid #f6851b;
        padding-left: 1rem;
    }
</style>
