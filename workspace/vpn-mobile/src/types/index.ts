export interface UserInfo {
  id: number;
  username: string;
  email: string;
}

export interface AuthResponse {
  token: string;
  user: UserInfo;
}

export interface Server {
  id: number;
  name: string;
  country: string;
  ip: string;
  public_key: string;
  listen_port: number;
  subnet: string;
  is_active: boolean;
}

export interface ConnectionInfo {
  peer_ip: string;
  config: string;
}

export interface PeerStatus {
  id: number;
  user_id: number;
  server_id: number;
  public_key: string;
  allowed_ip: string;
  server: Server;
}
