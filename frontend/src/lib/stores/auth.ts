import { writable, derived } from 'svelte/store';
import type { User } from '$lib/types/auth.js';
import { isAuthenticated, getAuthToken, setAuthToken, removeAuthToken, getUserProfile } from '$lib/services/api.js';
import { browser } from '$app/environment';

interface AuthState {
    isLoading: boolean;
    isAuthenticated: boolean;
    user: User | null;
    error: string | null;
}

const initialState: AuthState = {
    isLoading: false,
    isAuthenticated: false,
    user: null,
    error: null
};

const authStore = writable<AuthState>(initialState);

export async function loadUserProfile(): Promise<void> {
    if (!isAuthenticated()) return;
    
    try {
        authStore.update(state => ({ ...state, isLoading: true }));
        const userData = await getUserProfile() as User;
        
        authStore.update(state => ({
            ...state,
            isLoading: false,
            isAuthenticated: true,
            user: userData
        }));
    } catch (error) {
        console.error('Gagal memuat profil user:', error);
        
        removeAuthToken();
        
        authStore.update(state => ({
            ...state,
            isLoading: false,
            isAuthenticated: false,
            user: null,
            error: 'Sesi login telah kedaluwarsa. Silakan login kembali.'
        }));
    }
}

if (browser) {
    authStore.update(state => ({
        ...state,
        isAuthenticated: isAuthenticated()
    }));
    
    if (isAuthenticated()) {
        loadUserProfile();
    }
}

export async function loginWithMetamask(
    walletAddress: string,
    nonce: string,
    signature: string
): Promise<void> {
    try {
        authStore.update((state) => ({ ...state, isLoading: true, error: null }));

        const response = await fetch('http://localhost:8080/api/auth/verify', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                wallet_address: walletAddress,
                signature,
                nonce
            })
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.error || 'Autentikasi gagal');
        }

        const data = await response.json();
        
        setAuthToken(data.token);
        
        authStore.update((state) => ({
            ...state,
            isLoading: false,
            isAuthenticated: true,
            user: data.user,
            error: null
        }));
    } catch (error) {
        let errorMessage = 'Terjadi kesalahan saat login';
        if (error instanceof Error) {
            errorMessage = error.message;
        }
        
        authStore.update((state) => ({
            ...state,
            isLoading: false,
            error: errorMessage
        }));
        
        throw error;
    }
}

export function logout(): void {
    removeAuthToken();
    authStore.set({
        isLoading: false,
        isAuthenticated: false,
        user: null,
        error: null
    });
}

export const isAuthLoading = derived(authStore, ($auth) => $auth.isLoading);
export const isLoggedIn = derived(authStore, ($auth) => $auth.isAuthenticated);
export const user = derived(authStore, ($auth) => $auth.user);
export const authError = derived(authStore, ($auth) => $auth.error);

export { authStore }; 