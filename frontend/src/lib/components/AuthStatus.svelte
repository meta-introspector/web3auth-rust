<script lang="ts">
    import { isLoggedIn, user, authError } from '$lib/stores/auth.js';
</script>

<div class="auth-status">
    {#if $isLoggedIn && $user}
        <div class="status logged-in">
            <span class="dot green"></span>
            <span>Terhubung sebagai: {$user.wallet_address.substring(0, 6)}...{$user.wallet_address.substring($user.wallet_address.length - 4)}</span>
        </div>
    {:else}
        <div class="status logged-out">
            <span class="dot red"></span>
            <span>Tidak terhubung</span>
        </div>
    {/if}
    
    {#if $authError}
        <div class="auth-error">
            {$authError}
        </div>
    {/if}
</div>

<style>
    .auth-status {
        display: flex;
        flex-direction: column;
        margin-bottom: 16px;
    }
    
    .status {
        display: flex;
        align-items: center;
        font-size: 14px;
        margin-bottom: 4px;
    }
    
    .dot {
        width: 10px;
        height: 10px;
        border-radius: 50%;
        margin-right: 8px;
    }
    
    .green {
        background-color: #28a745;
    }
    
    .red {
        background-color: #dc3545;
    }
    
    .auth-error {
        color: #dc3545;
        font-size: 14px;
        margin-top: 4px;
    }
</style> 