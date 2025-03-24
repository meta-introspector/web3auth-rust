import type { NonceResponse, AuthResponse } from '$lib/types/auth.js';

const API_URL = 'http://localhost:8080/api';

async function fetchApi<T>(
    endpoint: string,
    options: RequestInit = {}
): Promise<T> {
    const url = `${API_URL}${endpoint}`;
    
    const token = isBrowser() ? localStorage.getItem('auth_token') : null;

    const headers = {
        'Content-Type': 'application/json',
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...options.headers
    };

    const response = await fetch(url, {
        ...options,
        headers
    });

    if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Terjadi kesalahan pada request');
    }

    return response.json() as Promise<T>;
}

function isBrowser(): boolean {
    return typeof window !== 'undefined';
}

export async function getNonce(walletAddress: string): Promise<NonceResponse> {
    return fetchApi<NonceResponse>('/auth/nonce', {
        method: 'POST',
        body: JSON.stringify({ wallet_address: walletAddress })
    });
}

export async function verifySignature(
    walletAddress: string,
    signature: string,
    nonce: string
): Promise<AuthResponse> {
    return fetchApi<AuthResponse>('/auth/verify', {
        method: 'POST',
        body: JSON.stringify({
            wallet_address: walletAddress,
            signature,
            nonce
        })
    });
}

export async function getUserProfile() {
    return fetchApi('/profile', {
        method: 'GET'
    });
}

export function isAuthenticated(): boolean {
    return isBrowser() ? !!localStorage.getItem('auth_token') : false;
}

export function setAuthToken(token: string): void {
    if (isBrowser()) {
        localStorage.setItem('auth_token', token);
    }
}

export function getAuthToken(): string | null {
    return isBrowser() ? localStorage.getItem('auth_token') : null;
}

export function removeAuthToken(): void {
    if (isBrowser()) {
        localStorage.removeItem('auth_token');
    }
} 