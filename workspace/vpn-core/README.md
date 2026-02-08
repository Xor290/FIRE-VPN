# vpn-core (Rust)

Bibliotheque Rust partagee entre le client desktop et le client mobile. Contient toute la logique metier : authentification, communication API, parsing des configs WireGuard et gestion de session.

## Ce que vpn-core fait

- Authentification (register, login) vers l'API Go
- Client HTTP pour tous les endpoints VPN (servers, connect, disconnect, status)
- Parsing et serialisation des configs WireGuard (format INI)
- Gestion de session : connexion, deconnexion, switch de serveur

## Ce que vpn-core NE fait PAS

- Pas de manipulation directe du tunnel VPN (pas de wg-quick, pas de tun)
- Pas d'UI
- Pas de code natif Android

Le code appelant (desktop ou mobile natif) recoit la config WireGuard parsee et l'applique selon la plateforme :
- **Desktop** : wg-quick / wireguard.exe
- **Android** : VpnService + WireGuard SDK

## Structure

```
vpn-core/
├── Cargo.toml
└── src/
    ├── lib.rs                # Re-exports des modules publics
    ├── api/
    │   ├── mod.rs            # Types publics (Server, ConnectionInfo, PeerStatus, ApiError)
    │   └── client.rs         # ApiClient (list_servers, connect, disconnect, status)
    ├── auth/
    │   ├── mod.rs            # Types publics (UserInfo, AuthResponse, AuthError)
    │   └── handlers.rs       # register(), login()
    ├── session/
    │   ├── mod.rs            # Types publics (SessionError) + re-exports
    │   └── manager.rs        # Session (orchestration login -> connect -> switch)
    └── wireguard/
        ├── mod.rs            # Re-exports
        └── config.rs         # WireGuardConfig parse/serialise + WireGuardError
```

## Dependances

- `reqwest` (blocking + json) - Client HTTP
- `serde` / `serde_json` - Serialisation/deserialisation
- `thiserror` - Gestion d'erreurs

## API publique

### Module `auth`

Appels HTTP vers `/auth/register` et `/auth/login`.

#### Types

```rust
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
}

pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

pub enum AuthError {
    Request(reqwest::Error),
    Api(String),
}
```

#### Fonctions

```rust
pub fn register(base_url: &str, username: &str, email: &str, password: &str)
    -> Result<AuthResponse, AuthError>

pub fn login(base_url: &str, email: &str, password: &str)
    -> Result<AuthResponse, AuthError>
```

#### Exemple

```rust
let resp = auth::login("http://localhost:8080", "john@example.com", "pass")?;
println!("Token: {}", resp.token);
println!("User: {} ({})", resp.user.username, resp.user.email);
```

### Module `api`

Client HTTP authentifie pour tous les endpoints VPN.

#### Types

```rust
pub struct Server {
    pub id: u64,
    pub name: String,
    pub country: String,
    pub ip: String,
    pub public_key: String,
    pub listen_port: u16,
    pub subnet: String,
    pub is_active: bool,
}

pub struct ConnectionInfo {
    pub peer_ip: String,
    pub config: String,
}

pub struct PeerStatus {
    pub id: u64,
    pub user_id: u64,
    pub server_id: u64,
    pub public_key: String,
    pub allowed_ip: String,
    pub server: Server,
}

pub enum ApiError {
    Request(reqwest::Error),
    Api(String),
}
```

#### Methodes de `ApiClient`

```rust
impl ApiClient {
    pub fn new(base_url: &str, token: &str) -> Self
    pub fn set_token(&mut self, token: &str)
    pub fn list_servers(&self) -> Result<Vec<Server>, ApiError>
    pub fn connect(&self, server_id: u64) -> Result<ConnectionInfo, ApiError>
    pub fn disconnect(&self, server_id: u64) -> Result<(), ApiError>
    pub fn status(&self) -> Result<Vec<PeerStatus>, ApiError>
}
```

#### Exemple

```rust
let client = ApiClient::new("http://localhost:8080", &token);

let servers = client.list_servers()?;
let conn = client.connect(1)?;           // retourne peer_ip + config INI
client.disconnect(1)?;
let peers = client.status()?;
```

### Module `wireguard`

Parse la config WireGuard au format INI retournee par l'API et l'expose en struct typee.

#### Types

```rust
pub struct WireGuardConfig {
    pub private_key: String,
    pub address: String,
    pub dns: String,
    pub peer_public_key: String,
    pub endpoint: String,
    pub allowed_ips: String,
    pub persistent_keepalive: u16,
}

pub enum WireGuardError {
    MissingField(String),
    InvalidFormat,
}
```

#### Methodes de `WireGuardConfig`

```rust
impl WireGuardConfig {
    pub fn parse(config_str: &str) -> Result<Self, WireGuardError>
    pub fn to_ini(&self) -> String
}
```

#### Exemple

```rust
let config = WireGuardConfig::parse(raw_ini_string)?;
println!("{}", config.endpoint);       // "1.2.3.4:51820"
println!("{}", config.private_key);    // cle privee du peer

let ini = config.to_ini();             // re-serialise en format INI
```

### Module `session`

Orchestre l'ensemble : login, connexion, switch serveur. C'est le point d'entree principal pour les clients.

#### Types

```rust
pub enum SessionError {
    Auth(AuthError),
    Api(ApiError),
    WireGuard(WireGuardError),
    NotConnected,
}
```

#### Methodes de `Session`

```rust
impl Session {
    // Constructeurs
    pub fn login(base_url: &str, email: &str, password: &str)
        -> Result<Self, SessionError>
    pub fn register(base_url: &str, username: &str, email: &str, password: &str)
        -> Result<Self, SessionError>

    // Accesseurs
    pub fn user(&self) -> &UserInfo
    pub fn token(&self) -> &str
    pub fn current_server(&self) -> Option<&Server>
    pub fn current_config(&self) -> Option<&WireGuardConfig>
    pub fn is_connected(&self) -> bool

    // Operations VPN
    pub fn list_servers(&self) -> Result<Vec<Server>, SessionError>
    pub fn connect(&mut self, server_id: u64) -> Result<&WireGuardConfig, SessionError>
    pub fn disconnect(&mut self) -> Result<(), SessionError>
    pub fn switch_server(&mut self, new_server_id: u64)
        -> Result<&WireGuardConfig, SessionError>
}
```

#### Exemple

```rust
let mut session = Session::login("http://localhost:8080", "john@example.com", "pass")?;

let servers = session.list_servers()?;

// Connecter - retourne la config WireGuard a appliquer
let config = session.connect(servers[0].id)?;
println!("{}", config.to_ini());

// Switch de serveur (disconnect + connect)
let new_config = session.switch_server(servers[1].id)?;

session.disconnect()?;
```

## Build

```bash
cd workspace/vpn-core
cargo build --lib
```

## Tests

```bash
cargo test
```

Tests inclus :
- Parse d'une config WireGuard valide
- Erreur sur champ manquant
- Roundtrip parse -> to_ini -> parse

## Crate types

Le crate produit 3 types de sortie (`Cargo.toml`) :

- `lib` - Usage Rust natif (desktop client)
- `staticlib` - Lib statique C pour linking mobile
- `cdylib` - Lib dynamique C pour FFI mobile

## Alignement avec l'API Go

| Rust                         | API Go                |
|------------------------------|-----------------------|
| `auth::register()`          | POST /auth/register   |
| `auth::login()`             | POST /auth/login      |
| `ApiClient::list_servers()` | GET /vpn/servers      |
| `ApiClient::connect()`      | POST /vpn/connect     |
| `ApiClient::disconnect()`   | POST /vpn/disconnect  |
| `ApiClient::status()`       | GET /vpn/status       |

## Flux type (desktop)

```
Session::login()
    -> POST /auth/login -> JWT

Session::list_servers()
    -> GET /vpn/servers -> Vec<Server>

Session::connect(server_id)
    -> POST /vpn/connect -> WireGuardConfig
    -> appelant applique la config via wg-quick

Session::switch_server(new_id)
    -> disconnect ancien serveur
    -> connect nouveau serveur
    -> nouvelle config WireGuard
```

## Flux type (mobile)

```
Session::login()
    -> POST /auth/login -> JWT

Session::connect(server_id)
    -> POST /vpn/connect -> WireGuardConfig
    -> config transmise via FFI au code natif
    -> Kotlin: VpnService + WireGuard SDK
```
