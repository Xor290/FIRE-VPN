package models

import "github.com/golang-jwt/jwt/v5"

// ── Middleware ───────────────────────────────────────────────────────────

type Claims struct {
	UserID uint `json:"user_id"`
	jwt.RegisteredClaims
}
