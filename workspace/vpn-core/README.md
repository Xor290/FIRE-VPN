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
    ├── lib.rs           # Re-exports des modules publics
    ├── auth.rs          # register(), login() -> AuthResponse
    ├── api.rs           # ApiClient (list_servers, connect, disconnect, status)
    ├── wireguard.rs     # WireGuardConfig parse/serialise + tests
    └── session.rs       # Session (orchestration login -> connect -> switch)
```

## Dependances

- `reqwest` (blocking + json) - Client HTTP
- `serde` / `serde_json` - Serialisation/deserialisation
- `thiserror` - Gestion d'erreurs

## Modules

### wireguard.rs

Parse la config WireGuard au format INI retournee par l'API et l'expose en struct typee.

```rust
let config = WireGuardConfig::parse(raw_ini_string)?;
println!("{}", config.endpoint);       // "1.2.3.4:51820"
println!("{}", config.private_key);    // cle privee du peer

let ini = config.to_ini();             // re-serialise en format INI
```

### auth.rs

Appels HTTP vers `/auth/register` et `/auth/login`.

```rust
let resp = auth::login("http://localhost:8080", "john@example.com", "pass")?;
println!("Token: {}", resp.token);
println!("User: {} ({})", resp.user.username, resp.user.email);
```

### api.rs

Client HTTP authentifie pour tous les endpoints VPN.

```rust
let client = ApiClient::new("http://localhost:8080", &token);

let servers = client.list_servers()?;
let conn = client.connect(1)?;           // retourne peer_ip + config INI
client.disconnect(1)?;
let peers = client.status()?;
```

### session.rs

Orchestre l'ensemble : login, connexion, switch serveur.

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
