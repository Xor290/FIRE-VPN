import React, {
    createContext,
    useContext,
    useEffect,
    useReducer,
    useCallback,
} from "react";
import AsyncStorage from "@react-native-async-storage/async-storage";
import { UserInfo, Server, ConnectionInfo } from "../types";
import * as api from "../api/client";

const STORAGE_KEYS = {
    token: "@fire_vpn_token",
    email: "@fire_vpn_email",
    apiUrl: "@fire_vpn_api_url",
};

const DEFAULT_API_URL = "http://172.20.167.237:8080";

interface AppState {
    token: string | null;
    user: UserInfo | null;
    apiUrl: string;
    servers: Server[];
    connectedServer: Server | null;
    connectionInfo: ConnectionInfo | null;
    isLoading: boolean;
    error: string | null;
    savedEmail: string;
}

type Action =
    | { type: "SET_LOADING"; payload: boolean }
    | { type: "SET_ERROR"; payload: string | null }
    | { type: "LOGIN_SUCCESS"; payload: { token: string; user: UserInfo } }
    | { type: "LOGOUT" }
    | { type: "SET_SERVERS"; payload: Server[] }
    | {
          type: "CONNECT_SUCCESS";
          payload: { server: Server; info: ConnectionInfo };
      }
    | { type: "DISCONNECT_SUCCESS" }
    | {
          type: "RESTORE_SESSION";
          payload: { token: string; email: string; apiUrl: string };
      }
    | { type: "SET_API_URL"; payload: string }
    | { type: "SET_SAVED_EMAIL"; payload: string }
    | { type: "UPDATE_PROFILE"; payload: UserInfo };

function reducer(state: AppState, action: Action): AppState {
    switch (action.type) {
        case "SET_LOADING":
            return { ...state, isLoading: action.payload };
        case "SET_ERROR":
            return { ...state, error: action.payload, isLoading: false };
        case "LOGIN_SUCCESS":
            return {
                ...state,
                token: action.payload.token,
                user: action.payload.user,
                savedEmail: action.payload.user.email,
                error: null,
                isLoading: false,
            };
        case "LOGOUT":
            return {
                ...state,
                token: null,
                user: null,
                servers: [],
                connectedServer: null,
                connectionInfo: null,
                error: null,
                isLoading: false,
            };
        case "SET_SERVERS":
            return { ...state, servers: action.payload, isLoading: false };
        case "CONNECT_SUCCESS":
            return {
                ...state,
                connectedServer: action.payload.server,
                connectionInfo: action.payload.info,
                error: null,
                isLoading: false,
            };
        case "DISCONNECT_SUCCESS":
            return {
                ...state,
                connectedServer: null,
                connectionInfo: null,
                error: null,
                isLoading: false,
            };
        case "RESTORE_SESSION":
            return {
                ...state,
                token: action.payload.token,
                savedEmail: action.payload.email,
                apiUrl: action.payload.apiUrl || DEFAULT_API_URL,
            };
        case "SET_API_URL":
            return { ...state, apiUrl: action.payload };
        case "SET_SAVED_EMAIL":
            return { ...state, savedEmail: action.payload };
        case "UPDATE_PROFILE":
            return {
                ...state,
                user: action.payload,
                savedEmail: action.payload.email,
                isLoading: false,
                error: null,
            };
        default:
            return state;
    }
}

const initialState: AppState = {
    token: null,
    user: null,
    apiUrl: DEFAULT_API_URL,
    servers: [],
    connectedServer: null,
    connectionInfo: null,
    isLoading: false,
    error: null,
    savedEmail: "",
};

interface AuthContextValue extends AppState {
    login: (email: string, password: string) => Promise<void>;
    register: (
        username: string,
        email: string,
        password: string,
    ) => Promise<void>;
    logout: () => Promise<void>;
    loadServers: () => Promise<void>;
    connectToServer: (server: Server) => Promise<void>;
    disconnectFromServer: () => Promise<void>;
    switchServer: (server: Server) => Promise<void>;
    clearError: () => void;
    setApiUrl: (url: string) => void;
    refreshProfile: () => Promise<void>;
    updateProfile: (
        username: string,
        email: string,
        password: string,
    ) => Promise<void>;
    deleteAccount: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const [state, dispatch] = useReducer(reducer, initialState);

    useEffect(() => {
        (async () => {
            const [token, email, apiUrl] = await Promise.all([
                AsyncStorage.getItem(STORAGE_KEYS.token),
                AsyncStorage.getItem(STORAGE_KEYS.email),
                AsyncStorage.getItem(STORAGE_KEYS.apiUrl),
            ]);
            if (token || email || apiUrl) {
                dispatch({
                    type: "RESTORE_SESSION",
                    payload: {
                        token: token ?? "",
                        email: email ?? "",
                        apiUrl: apiUrl ?? DEFAULT_API_URL,
                    },
                });
            }
        })();
    }, []);

    const handleLogin = useCallback(
        async (email: string, password: string) => {
            dispatch({ type: "SET_LOADING", payload: true });
            try {
                const res = await api.login(state.apiUrl, email, password);
                await AsyncStorage.setItem(STORAGE_KEYS.token, res.token);
                await AsyncStorage.setItem(STORAGE_KEYS.email, res.user.email);
                await AsyncStorage.setItem(STORAGE_KEYS.apiUrl, state.apiUrl);
                dispatch({ type: "LOGIN_SUCCESS", payload: res });
            } catch (e: any) {
                dispatch({
                    type: "SET_ERROR",
                    payload: e.message ?? "Erreur de connexion",
                });
            }
        },
        [state.apiUrl],
    );

    const handleRegister = useCallback(
        async (username: string, email: string, password: string) => {
            dispatch({ type: "SET_LOADING", payload: true });
            try {
                const res = await api.register(
                    state.apiUrl,
                    username,
                    email,
                    password,
                );
                await AsyncStorage.setItem(STORAGE_KEYS.token, res.token);
                await AsyncStorage.setItem(STORAGE_KEYS.email, res.user.email);
                await AsyncStorage.setItem(STORAGE_KEYS.apiUrl, state.apiUrl);
                dispatch({ type: "LOGIN_SUCCESS", payload: res });
            } catch (e: any) {
                dispatch({
                    type: "SET_ERROR",
                    payload: e.message ?? "Erreur d'inscription",
                });
            }
        },
        [state.apiUrl],
    );

    const handleLogout = useCallback(async () => {
        if (state.connectedServer && state.token) {
            try {
                await api.disconnectFromServer(
                    state.apiUrl,
                    state.token,
                    state.connectedServer.id,
                );
            } catch {}
        }
        await AsyncStorage.removeItem(STORAGE_KEYS.token);
        dispatch({ type: "LOGOUT" });
    }, [state.apiUrl, state.token, state.connectedServer]);

    const handleLoadServers = useCallback(async () => {
        if (!state.token) return;
        dispatch({ type: "SET_LOADING", payload: true });
        try {
            const servers = await api.listServers(state.apiUrl, state.token);
            dispatch({ type: "SET_SERVERS", payload: servers });
        } catch (e: any) {
            dispatch({
                type: "SET_ERROR",
                payload: e.message ?? "Erreur chargement serveurs",
            });
        }
    }, [state.apiUrl, state.token]);

    const handleConnect = useCallback(
        async (server: Server) => {
            if (!state.token) return;
            dispatch({ type: "SET_LOADING", payload: true });
            try {
                const info = await api.connectToServer(
                    state.apiUrl,
                    state.token,
                    server.id,
                );
                dispatch({
                    type: "CONNECT_SUCCESS",
                    payload: { server, info },
                });
            } catch (e: any) {
                dispatch({
                    type: "SET_ERROR",
                    payload: e.message ?? "Erreur de connexion VPN",
                });
            }
        },
        [state.apiUrl, state.token],
    );

    const handleDisconnect = useCallback(async () => {
        if (!state.token || !state.connectedServer) return;
        dispatch({ type: "SET_LOADING", payload: true });
        try {
            await api.disconnectFromServer(
                state.apiUrl,
                state.token,
                state.connectedServer.id,
            );
            dispatch({ type: "DISCONNECT_SUCCESS" });
        } catch (e: any) {
            dispatch({
                type: "SET_ERROR",
                payload: e.message ?? "Erreur de deconnexion",
            });
        }
    }, [state.apiUrl, state.token, state.connectedServer]);

    const handleSwitchServer = useCallback(
        async (server: Server) => {
            if (!state.token || !state.connectedServer) return;
            dispatch({ type: "SET_LOADING", payload: true });
            try {
                await api.disconnectFromServer(
                    state.apiUrl,
                    state.token,
                    state.connectedServer.id,
                );
                const info = await api.connectToServer(
                    state.apiUrl,
                    state.token,
                    server.id,
                );
                dispatch({
                    type: "CONNECT_SUCCESS",
                    payload: { server, info },
                });
            } catch (e: any) {
                dispatch({
                    type: "SET_ERROR",
                    payload: e.message ?? "Erreur changement de serveur",
                });
            }
        },
        [state.apiUrl, state.token, state.connectedServer],
    );

    const handleRefreshProfile = useCallback(async () => {
        if (!state.token) return;
        dispatch({ type: "SET_LOADING", payload: true });
        try {
            const user = await api.getProfileInfo(state.apiUrl, state.token);
            dispatch({ type: "UPDATE_PROFILE", payload: user });
        } catch (e: any) {
            dispatch({
                type: "SET_ERROR",
                payload: e.message ?? "Erreur chargement profil",
            });
        }
    }, [state.apiUrl, state.token]);

    const handleUpdateProfile = useCallback(
        async (username: string, email: string, password: string) => {
            if (!state.token) return;
            dispatch({ type: "SET_LOADING", payload: true });
            try {
                const user = await api.updateProfile(
                    state.apiUrl,
                    state.token,
                    username,
                    email,
                    password,
                );
                dispatch({ type: "UPDATE_PROFILE", payload: user });
            } catch (e: any) {
                dispatch({
                    type: "SET_ERROR",
                    payload: e.message ?? "Erreur mise a jour profil",
                });
            }
        },
        [state.apiUrl, state.token],
    );

    const handleDeleteAccount = useCallback(async () => {
        if (!state.token) return;
        dispatch({ type: "SET_LOADING", payload: true });
        try {
            await api.deleteProfile(state.apiUrl, state.token);
            await AsyncStorage.removeItem(STORAGE_KEYS.token);
            dispatch({ type: "LOGOUT" });
        } catch (e: any) {
            dispatch({
                type: "SET_ERROR",
                payload: e.message ?? "Erreur suppression du compte",
            });
        }
    }, [state.apiUrl, state.token]);

    const clearError = useCallback(() => {
        dispatch({ type: "SET_ERROR", payload: null });
    }, []);

    const setApiUrl = useCallback((url: string) => {
        dispatch({ type: "SET_API_URL", payload: url });
    }, []);

    const value: AuthContextValue = {
        ...state,
        login: handleLogin,
        register: handleRegister,
        logout: handleLogout,
        loadServers: handleLoadServers,
        connectToServer: handleConnect,
        disconnectFromServer: handleDisconnect,
        switchServer: handleSwitchServer,
        clearError,
        setApiUrl,
        refreshProfile: handleRefreshProfile,
        updateProfile: handleUpdateProfile,
        deleteAccount: handleDeleteAccount,
    };

    return (
        <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
    );
}

export function useAuth(): AuthContextValue {
    const ctx = useContext(AuthContext);
    if (!ctx) throw new Error("useAuth must be used within AuthProvider");
    return ctx;
}
