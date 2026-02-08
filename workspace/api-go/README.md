# VPN API (Go)

API centrale du projet VPN. Gere l'authentification, les utilisateurs, les serveurs VPN et la gestion dynamique des peers WireGuard sur les VPS.

## Stack

- **Go 1.24+**
- **Gin** - HTTP router
- **GORM** - ORM (PostgreSQL)
- **golang-jwt** - Authentification JWT avec session secret par utilisateur
- **x/crypto** - bcrypt (passwords), curve25519 (cles WireGuard), SSH

## Structure

```
api-go/
├── main.go                 # Point d'entree, init DB + demarrage serveur
├── .env                    # Variables d'environnement
├── config/
│   └── config.go           # Chargement config (.env)
├── models/
│   ├── user.go             # User (username, email, password, session_secret)
│   ├── server.go           # VPNServer (ip, keys, subnet, port)
│   └── peer.go             # Peer WireGuard (user <-> server)
├── db/
│   ├── users.go            # Requetes DB users
│   ├── servers.go          # Requetes DB serveurs
│   └── peers.go            # Requetes DB peers
├── handlers/
│   ├── handlers.go         # Structs handlers + DTOs requetes
│   ├── auth.go             # Register, Login
│   ├── servers.go          # Liste serveurs
│   └── vpn.go              # Connect, Disconnect, Status
├── middleware/
│   └── auth.go             # Middleware JWT (signature per-user)
├── routes/
│   └── routes.go           # Definition des routes Gin
├── services/
│   ├── wireguard.go        # Generation cles WireGuard + configs + allocation IP
│   └── ssh.go              # Execution commandes WireGuard sur VPS via SSH
└── utils/
    └── response.go         # Helpers reponses JSON
```

## Configuration

Copier `.env` et modifier les valeurs :

```env
API_PORT=8080

DB_HOST=localhost
DB_PORT=5432
DB_USER=vpn_admin
DB_PASSWORD=change-me-in-production
DB_NAME=vpn_db
DB_SSLMODE=disable

JWT_SECRET=change-me-in-production

SSH_KEY_PATH=~/.ssh/id_rsa
```

## Pre-requis

- PostgreSQL en cours d'execution
- Creer la base de donnees :

```sql
CREATE USER vpn_admin WITH PASSWORD 'change-me-in-production';
CREATE DATABASE vpn_db OWNER vpn_admin;
```

## Lancement

```bash
cd workspace/api-go
go mod tidy
go run main.go
```

L'API demarre sur `http://localhost:8080`. Les tables sont creees automatiquement via GORM AutoMigrate.

## Endpoints

### Public

| Methode | Route            | Description          | Body                                        |
|---------|------------------|----------------------|---------------------------------------------|
| POST    | /auth/register   | Inscription          | `{"username", "email", "password"}`         |
| POST    | /auth/login      | Connexion            | `{"email", "password"}`                     |

### Protege (JWT)

Header requis : `Authorization: Bearer <token>`

| Methode | Route            | Description                        | Body                  |
|---------|------------------|------------------------------------|-----------------------|
| GET     | /vpn/servers     | Liste des serveurs actifs          | -                     |
| POST    | /vpn/connect     | Connecter a un serveur             | `{"server_id": 1}`   |
| POST    | /vpn/disconnect  | Deconnecter d'un serveur           | `{"server_id": 1}`   |
| GET     | /vpn/status      | Connexions VPN actives de l'user   | -                     |

## Exemples

### Register

```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "john", "email": "john@example.com", "password": "securepass123"}'
```

### Login

```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "john@example.com", "password": "securepass123"}'
```

### Lister les serveurs

```bash
curl http://localhost:8080/vpn/servers \
  -H "Authorization: Bearer <token>"
```

### Se connecter a un serveur

```bash
curl -X POST http://localhost:8080/vpn/connect \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"server_id": 1}'
```

La reponse contient la config WireGuard complete a appliquer cote client.

## Securite

- Mots de passe hashes avec bcrypt
- JWT signe avec `JWTSecret + SessionSecret` (unique par utilisateur)
- Chaque login regenere le session secret et invalide les anciens tokens
- Validation de l'algorithme de signature (bloque `alg: none`)
- Verification issuer, audience, expiration, not-before
- Cles WireGuard generees via curve25519 (pas de dependance CLI)
- Cles privees jamais exposees dans les reponses (sauf config client a la connexion)

## Flux /vpn/connect

1. Genere une paire de cles WireGuard pour le peer
2. Attribue une IP dans le subnet du serveur
3. Ajoute le peer sur le VPS via SSH (`wg set wg0 peer ...`)
4. Sauvegarde le peer en DB
5. Retourne la config WireGuard client au format INI
