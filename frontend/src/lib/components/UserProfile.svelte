<script lang="ts">
    import { user, logout } from '$lib/stores/auth.js';
    
    function formatDate(dateString: string | null): string {
        if (!dateString) return 'N/A';
        const date = new Date(dateString);
        return new Intl.DateTimeFormat('id-ID', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        }).format(date);
    }
    
    function formatWalletAddress(address: string): string {
        if (!address || address.length < 10) return address;
        return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
    }
    
    function handleLogout() {
        logout();
    }
</script>

{#if $user}
    <div class="profile-container">
        <h2>Profil Pengguna</h2>
        
        <div class="wallet-info">
            <div class="wallet-address-row">
                <span class="label">Alamat Wallet:</span>
                <span class="address" title={$user.wallet_address}>
                    {formatWalletAddress($user.wallet_address)}
                </span>
            </div>
            
            <div class="info-row">
                <span class="label">Login Terakhir:</span>
                <span>{formatDate($user.last_login)}</span>
            </div>
            
            <div class="info-row">
                <span class="label">Dibuat Pada:</span>
                <span>{formatDate($user.created_at)}</span>
            </div>
        </div>
        
        <button class="logout-button" on:click={handleLogout}>
            Keluar
        </button>
    </div>
{:else}
    <div class="profile-container empty">
        <p>Silakan login dengan Metamask untuk melihat profil Anda.</p>
    </div>
{/if}

<style>
    .profile-container {
        background-color: #f9f9f9;
        border-radius: 8px;
        padding: 20px;
        margin: 20px 0;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
    
    .profile-container.empty {
        text-align: center;
        color: #666;
        font-style: italic;
    }
    
    h2 {
        margin-top: 0;
        color: #333;
        font-size: 1.5em;
        margin-bottom: 16px;
    }
    
    .wallet-info {
        margin-bottom: 20px;
    }
    
    .wallet-address-row, .info-row {
        display: flex;
        margin-bottom: 8px;
        font-size: 14px;
    }
    
    .label {
        width: 120px;
        font-weight: bold;
        color: #555;
    }
    
    .address {
        font-family: monospace;
        background: #eee;
        padding: 2px 6px;
        border-radius: 4px;
        cursor: pointer;
    }
    
    .logout-button {
        background-color: #dc3545;
        color: white;
        border: none;
        border-radius: 4px;
        padding: 8px 16px;
        cursor: pointer;
        font-size: 14px;
        transition: background-color 0.2s;
    }
    
    .logout-button:hover {
        background-color: #c82333;
    }
</style> 