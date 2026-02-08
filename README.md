# FIRE-VPN

Service VPN base sur WireGuard avec architecture client-serveur. Les utilisateurs s'authentifient, decouvrent les serveurs disponibles et s'y connectent dynamiquement. Le systeme gere automatiquement le cycle de vie des peers WireGuard sur les VPS distants via SSH.

## Architecture

```mermaid
graph TB
    subgraph Clients
        Desktop["Desktop (Rust / egui)<br/>wg-quick"]
        Mobile["Mobile Android<br/>VpnService + WireGuard SDK"]
    end

    subgraph "vpn-core (Rust lib)"
        Auth["auth/<br/>register / login"]
        API["api/<br/>ApiClient HTTP"]
        WG["wireguard/<br/>Parsing config INI"]
        Session["session/<br/>Orchestration"]
    end

    subgraph "vpn-desktop (Rust / egui)"
        UI["ui/<br/>login, servers, connection"]
        Theme["ui/theme<br/>Design system"]
        Tunnel["vpn/tunnel<br/>wg-quick"]
    end

    subgraph "API Go (Gin)"
        Handlers["Handlers HTTP"]
        Middleware["Middleware JWT"]
        Services["Services<br/>WireGuard + SSH"]
    end

    subgraph Infrastructure
        DB[(PostgreSQL<br/>Users, Servers, Peers)]
        VPS1["VPS WireGuard<br/>Data plane"]
        VPS2["VPS WireGuard<br/>Data plane"]
    end

    Desktop --> UI
    UI --> Session
    Mobile --> Session
    Session --> Auth
    Session --> API
    API --> WG
    Tunnel --> WG
    Auth --> Handlers
    API --> Handlers
    Handlers --> Middleware
    Handlers --> Services
    Services --> DB
    Services -->|SSH| VPS1
    Services -->|SSH| VPS2
    Middleware --> DB
```

**API Go** : plan de controle -- authentification, gestion des utilisateurs, allocation des peers, execution SSH sur les VPS.

**vpn-core (Rust)** : bibliotheque client partagee -- communication avec l'API, parsing des configs WireGuard, orchestration de session.

**vpn-desktop (Rust / egui)** : client desktop Linux -- interface graphique, application des tunnels WireGuard via wg-quick.

**WireGuard sur VPS** : plan de donnees -- tunneling VPN.

## Schema de la base de donnees

```mermaid
erDiagram
    Users {
        uint id PK
        string username UK
        string email UK
        string password "bcrypt hash"
        string session_secret "32 bytes hex"
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    VPNServers {
        uint id PK
        string name
        string country
        string ip
        string public_key
        string private_key
        int listen_port "default 51820"
        string subnet "CIDR ex 10.0.1.0/24"
        bool is_active
        timestamp created_at
        timestamp updated_at
        timestamp deleted_at
    }

    Peers {
        uint id PK
        uint user_id FK
        uint server_id FK
        string public_key
        string private_key
        string allowed_ip "ex 10.0.1.5/32"
        timestamp created_at
        timestamp deleted_at
    }

    Users ||--o{ Peers : "possede"
    VPNServers ||--o{ Peers : "heberge"
```

## Stack technique

| Composant | Technologies |
|-----------|-------------|
| API Backend | Go 1.24, Gin, GORM, PostgreSQL, golang-jwt, curve25519, SSH |
| Lib Client | Rust 2021, reqwest, serde, thiserror |
| Client Desktop | Rust 2021, eframe/egui 0.29, tokio |
| VPN | WireGuard (kernel Linux sur VPS) |
| Base de donnees | PostgreSQL 16 (Docker) |

## Structure du projet

```
FIRE-VPN/
├── .github/workflows/
│   ├── api-go.yml              # CI API Go
│   ├── vpn-core.yml            # CI lib Rust
│   └── vpn-desktop.yml         # CI client desktop (Linux)
├── docker/
│   └── docker-compose.yml      # PostgreSQL 16
└── workspace/
    ├── api-go/                 # API backend Go
    │   ├── main.go             # Point d'entree
    │   ├── config/             # Chargement variables d'environnement
    │   ├── models/             # User, VPNServer, Peer, Request, Services
    │   ├── db/                 # Operations CRUD (GORM)
    │   ├── handlers/           # Handlers HTTP (auth, vpn, servers)
    │   ├── middleware/         # Authentification JWT
    │   ├── services/           # Generation cles WireGuard, SSH
    │   ├── routes/             # Definition des routes
    │   ├── helpers/            # Helpers utilisateur
    │   └── utils/              # Helpers reponses JSON
    │
    ├── vpn-core/               # Bibliotheque client Rust
    │   └── src/
    │       ├── lib.rs          # Re-exports des modules publics
    │       ├── api/
    │       │   ├── mod.rs      # Types (Server, ConnectionInfo, PeerStatus, ApiError)
    │       │   └── client.rs   # ApiClient HTTP
    │       ├── auth/
    │       │   ├── mod.rs      # Types (UserInfo, AuthResponse, AuthError)
    │       │   └── handlers.rs # register(), login()
    │       ├── session/
    │       │   ├── mod.rs      # Types (SessionError) + re-exports
    │       │   └── manager.rs  # Session (login, connect, switch, disconnect)
    │       └── wireguard/
    │           ├── mod.rs      # Re-exports
    │           └── config.rs   # WireGuardConfig parse/serialise
    │
    └── vpn-desktop/            # Client desktop Linux
        ├── Cargo.toml
        └── src/
            ├── main.rs         # Point d'entree eframe, detection WSL
            ├── app.rs          # Etat applicatif, logique metier
            ├── ui/
            │   ├── mod.rs      # Re-exports UI
            │   ├── theme.rs    # Design system (couleurs, boutons, cards)
            │   ├── login.rs    # Ecran login / inscription
            │   ├── servers.rs  # Liste des serveurs
            │   └── connection.rs # Ecran connecte + switch serveur
            └── vpn/
                ├── mod.rs
                └── tunnel.rs   # Application config WireGuard (wg-quick)
```

## Pre-requis

- **Go** 1.24+
- **Rust** (edition 2021)
- **Docker** et **Docker Compose**
- **VPS** avec WireGuard installe et accessible via SSH
- Cle SSH configuree pour l'acces aux VPS

## Installation

### Base de donnees (Docker)

```bash
cd docker
docker compose up -d
```

PostgreSQL 16 demarre sur le port `5432`. La base, l'utilisateur et le mot de passe sont configures via les variables d'environnement (voir section Configuration). Les valeurs par defaut correspondent au `.env` de l'API.

### API Go

```bash
cd workspace/api-go
cp .env .env.local    # modifier les valeurs
go mod tidy
go run main.go
```

Les tables sont creees automatiquement au demarrage via GORM AutoMigrate.

### Bibliotheque client Rust

```bash
cd workspace/vpn-core
cargo build --lib
cargo test
```

Le crate produit trois types de sortie :
- `lib` -- usage Rust natif (desktop)
- `staticlib` -- lib statique C (mobile)
- `cdylib` -- lib dynamique C (FFI mobile)

### Client desktop

```bash
cd workspace/vpn-desktop
cargo build --release
```

Dependances systeme Linux :
```bash
sudo apt-get install -y libgtk-3-dev libxdo-dev libssl-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libxkbcommon-dev libfontconfig1-dev
```

Le binaire est produit dans `target/release/vpn-desktop`. Sur WSL2, le client force automatiquement le backend X11 et le renderer Glow pour la compatibilite avec WSLg.

## Configuration

Variables d'environnement (fichier `.env` dans `workspace/api-go/`) :

| Variable | Description | Defaut |
|----------|-------------|--------|
| `API_PORT` | Port de l'API | `8080` |
| `DB_HOST` | Hote PostgreSQL | `localhost` |
| `DB_PORT` | Port PostgreSQL | `5432` |
| `DB_USER` | Utilisateur DB | `vpn_admin` |
| `DB_PASSWORD` | Mot de passe DB | - |
| `DB_NAME` | Nom de la base | `vpn_db` |
| `DB_SSLMODE` | Mode SSL PostgreSQL | `disable` |
| `JWT_SECRET` | Secret global JWT | - |
| `SSH_KEY_PATH` | Chemin cle SSH pour VPS | `~/.ssh/id_rsa` |

## Endpoints API

### Public

```
POST /auth/register    {"username", "email", "password"}
POST /auth/login       {"email", "password"}
```

### Protege (Authorization: Bearer <token>)

```
GET  /vpn/servers      Liste des serveurs actifs
POST /vpn/connect      {"server_id": 1}  ->  config WireGuard
POST /vpn/disconnect   {"server_id": 1}
GET  /vpn/status       Connexions actives de l'utilisateur
```

## Flux de connexion

```mermaid
sequenceDiagram
    participant C as Client (vpn-core)
    participant A as API Go
    participant DB as PostgreSQL
    participant V as VPS WireGuard

    C->>A: POST /auth/login {email, password}
    A->>DB: Verifie credentials + regenere session_secret
    A-->>C: JWT token

    C->>A: POST /vpn/connect {server_id}
    A->>A: Genere keypair WireGuard (curve25519)
    A->>DB: Charge VPNServer + alloue IP dans subnet
    A->>V: SSH: wg set wg0 peer <pubkey> allowed-ips <ip>
    A->>V: SSH: wg-quick save wg0
    A->>DB: Sauvegarde Peer
    A-->>C: Config WireGuard (format INI)

    C->>C: Applique config localement
    Note over C: Desktop: wg-quick<br/>Android: VpnService
```

## Flux de deconnexion

```mermaid
sequenceDiagram
    participant C as Client (vpn-core)
    participant A as API Go
    participant DB as PostgreSQL
    participant V as VPS WireGuard

    C->>A: POST /vpn/disconnect {server_id}
    A->>DB: Charge Peer
    A->>V: SSH: wg set wg0 peer <pubkey> remove
    A->>V: SSH: wg-quick save wg0
    A->>DB: Supprime Peer
    A-->>C: {message: "disconnected"}
```

## Securite

```mermaid
flowchart LR
    subgraph Authentification
        A1["Mot de passe"] -->|bcrypt| A2["Hash stocke en DB"]
        A3["Login"] -->|"Regenere session_secret"| A4["Ancien token invalide"]
        A3 --> A5["JWT signe avec<br/>JWTSecret + SessionSecret"]
    end

    subgraph "Validation JWT (middleware)"
        B1["Verifie algorithme"] --> B2["Verifie signature<br/>(secret per-user)"]
        B2 --> B3["Verifie issuer +<br/>audience + expiration"]
    end

    subgraph "Chiffrement VPN"
        C1["Cles WireGuard"] -->|curve25519| C2["Generees cote serveur"]
        C2 --> C3["Privee: transmise<br/>uniquement au client"]
    end
```

- Requetes parametrees via GORM (protection injection SQL)
- Cles privees jamais exposees dans les reponses API (sauf config client a la connexion)

## CI/CD

Trois pipelines GitHub Actions, declenchees sur push et pull request vers `main` :

```mermaid
flowchart LR
    subgraph "API Go CI"
        direction TB
        G1["go mod tidy<br/>(verification modules)"] --> G2["go vet<br/>(analyse statique)"]
        G2 --> G3["go build"]
        G3 --> G4["go test"]
    end

    subgraph "vpn-core CI"
        direction TB
        R1["cargo fmt --check<br/>(formatage)"] --> R2["cargo clippy<br/>(linting)"]
        R2 --> R3["cargo build --lib"]
    end

    subgraph "vpn-desktop CI"
        direction TB
        D1["apt-get install<br/>(deps systeme)"] --> D2["cargo fmt --check"]
        D2 --> D3["cargo clippy"]
        D3 --> D4["cargo build --release"]
        D4 --> D5["upload artifact"]
    end

    Push["Push / PR"] --> G1
    Push --> R1
    Push --> D1
```

| Pipeline | Declencheur | Services | Etapes | Artifact |
|----------|-------------|----------|--------|----------|
| **API Go** | `workspace/api-go/**` | PostgreSQL 16 (service container) | mod tidy, vet, build, test | - |
| **vpn-core** | `workspace/vpn-core/**` | - | fmt, clippy, build | libvpn_core.a / .so |
| **vpn-desktop** | `workspace/vpn-desktop/**` ou `workspace/vpn-core/**` | - | fmt, clippy, build release | vpn-desktop (Linux) |

## Documentation detaillee

- [API Go](workspace/api-go/README.md) -- endpoints, exemples curl, flux detailles
- [vpn-core](workspace/vpn-core/README.md) -- modules Rust, API publique, exemples de code
- [vpn-desktop](workspace/vpn-desktop/) -- client desktop Linux (eframe/egui)
