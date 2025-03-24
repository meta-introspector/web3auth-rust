import { BrowserProvider } from 'ethers';
import type { Eip1193Provider } from 'ethers';

declare global {
    interface Window {
        ethereum?: Eip1193Provider;
    }
}

export function isMetamaskAvailable(): boolean {
    return typeof window !== 'undefined' && !!window.ethereum;
}

export function getProvider(): BrowserProvider | null {
    if (!isMetamaskAvailable() || !window.ethereum) {
        return null;
    }
    return new BrowserProvider(window.ethereum);
}

export async function requestAccounts(): Promise<string[]> {
    const provider = getProvider();
    if (!provider) {
        throw new Error('Metamask tidak terdeteksi');
    }
    
    try {
        const accounts = await provider.send('eth_requestAccounts', []);
        return accounts as string[];
    } catch (error) {
        console.error('Gagal mendapatkan akun:', error);
        throw new Error('Gagal mengkoneksikan ke Metamask');
    }
}

export async function getCurrentWalletAddress(): Promise<string | null> {
    try {
        const accounts = await requestAccounts();
        return accounts[0] || null;
    } catch (error) {
        console.error('Gagal mendapatkan alamat wallet:', error);
        return null;
    }
}

export async function signMessage(message: string): Promise<string> {
    const provider = getProvider();
    if (!provider) {
        throw new Error('Metamask tidak terdeteksi');
    }
    
    try {
        const signer = await provider.getSigner();
        return await signer.signMessage(message);
    } catch (error) {
        console.error('Gagal menandatangani pesan:', error);
        throw new Error('Pengguna membatalkan penandatanganan pesan');
    }
} 