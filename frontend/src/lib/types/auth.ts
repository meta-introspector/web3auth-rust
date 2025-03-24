export interface User {
    wallet_address: string;
    last_login: string | null;
    created_at: string;
}

export interface NonceResponse {
    nonce: string;
    message: string;
}

export interface AuthResponse {
    token: string;
    user: User;
}

export interface ErrorResponse {
    error: string;
} 