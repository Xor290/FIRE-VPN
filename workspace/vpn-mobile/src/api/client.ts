import { AuthResponse, ConnectionInfo, PeerStatus, Server } from "../types";

class ApiError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "ApiError";
    }
}

async function request<T>(url: string, options: RequestInit = {}): Promise<T> {
    const res = await fetch(url, {
        ...options,
        headers: {
            "Content-Type": "application/json",
            ...options.headers,
        },
    });

    const body = await res.json();

    if (!res.ok) {
        throw new ApiError(body.error ?? `Erreur ${res.status}`);
    }

    // L'API Go wrappe les reponses dans { "data": ... }
    return (body.data ?? body) as T;
}

function authHeaders(token: string): Record<string, string> {
    return { Authorization: `Bearer ${token}` };
}

export async function register(
    baseUrl: string,
    username: string,
    email: string,
    password: string,
): Promise<AuthResponse> {
    return request<AuthResponse>(`${baseUrl}/auth/register`, {
        method: "POST",
        body: JSON.stringify({ username, email, password }),
    });
}

export async function login(
    baseUrl: string,
    email: string,
    password: string,
): Promise<AuthResponse> {
    return request<AuthResponse>(`${baseUrl}/auth/login`, {
        method: "POST",
        body: JSON.stringify({ email, password }),
    });
}

export async function listServers(
    baseUrl: string,
    token: string,
): Promise<Server[]> {
    return request<Server[]>(`${baseUrl}/vpn/servers`, {
        headers: authHeaders(token),
    });
}

export async function connectToServer(
    baseUrl: string,
    token: string,
    serverId: number,
): Promise<ConnectionInfo> {
    return request<ConnectionInfo>(`${baseUrl}/vpn/connect`, {
        method: "POST",
        headers: authHeaders(token),
        body: JSON.stringify({ server_id: serverId }),
    });
}

export async function disconnectFromServer(
    baseUrl: string,
    token: string,
    serverId: number,
): Promise<void> {
    await request(`${baseUrl}/vpn/disconnect`, {
        method: "POST",
        headers: authHeaders(token),
        body: JSON.stringify({ server_id: serverId }),
    });
}

export async function getStatus(
    baseUrl: string,
    token: string,
): Promise<PeerStatus[]> {
    return request<PeerStatus[]>(`${baseUrl}/vpn/status`, {
        headers: authHeaders(token),
    });
}
