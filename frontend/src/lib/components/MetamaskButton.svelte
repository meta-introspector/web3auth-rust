<script lang="ts">
    import { isMetamaskAvailable, getCurrentWalletAddress, signMessage } from '$lib/services/web3.js';
    import { getNonce, verifySignature } from '$lib/services/api.js';
    import { authStore, loginWithMetamask } from '$lib/stores/auth.js';
    
    let isLoading = false;
    let error: string | null = null;
    
    async function handleLogin() {
        if (!isMetamaskAvailable()) {
            error = 'Metamask tidak terinstal. Mohon install Metamask terlebih dahulu.';
            return;
        }
        
        error = null;
        isLoading = true;
        
        try {
            const walletAddress = await getCurrentWalletAddress();
            if (!walletAddress) {
                throw new Error('Gagal mendapatkan alamat wallet');
            }
            
            const nonceResponse = await getNonce(walletAddress);
            
            const signature = await signMessage(nonceResponse.message);
            
            await loginWithMetamask(walletAddress, nonceResponse.nonce, signature);
            
        } catch (err) {
            if (err instanceof Error) {
                error = err.message;
            } else {
                error = 'Terjadi kesalahan saat login';
            }
            console.error('Login error:', err);
        } finally {
            isLoading = false;
        }
    }
</script>

<button 
    class="metamask-button"
    on:click={handleLogin}
    disabled={isLoading || $authStore.isAuthenticated}
>
    {#if isLoading}
        <span class="loading-spinner"></span>
        <span>Menghubungkan...</span>
    {:else if $authStore.isAuthenticated}
        <span>Terhubung</span>
    {:else}
        <span>Login dengan Metamask</span>
    {/if}
</button>

{#if error}
    <div class="error-message">
        {error}
    </div>
{/if}

<style>
    .metamask-button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        background-color: #f6851b;
        color: white;
        font-weight: bold;
        border: none;
        border-radius: 4px;
        padding: 10px 16px;
        cursor: pointer;
        transition: background-color 0.2s;
    }
    
    .metamask-button:hover:not(:disabled) {
        background-color: #e2761b;
    }
    
    .metamask-button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }
    
    .loading-spinner {
        display: inline-block;
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255,255,255,0.3);
        border-radius: 50%;
        border-top-color: white;
        animation: spin 1s ease-in-out infinite;
    }
    
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
    
    .error-message {
        color: #dc3545;
        margin-top: 10px;
        font-size: 14px;
    }
</style> 