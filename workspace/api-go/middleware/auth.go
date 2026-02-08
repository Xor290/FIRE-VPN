package middleware

import (
	"crypto/hmac"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"gorm.io/gorm"
	"vpn-api/config"
	"vpn-api/models"
	"vpn-api/utils"
)

const (
	issuer   = "vpn-api"
	audience = "vpn-client"
)

type Claims struct {
	UserID uint `json:"user_id"`
	jwt.RegisteredClaims
}

// signingKey construit la clé de signature unique par user : JWTSecret + SessionSecret.
func signingKey(globalSecret, sessionSecret string) []byte {
	return []byte(globalSecret + ":" + sessionSecret)
}

func GenerateToken(userID uint, sessionSecret string, cfg *config.Config) (string, error) {
	now := time.Now()
	claims := Claims{
		UserID: userID,
		RegisteredClaims: jwt.RegisteredClaims{
			Issuer:    issuer,
			Audience:  jwt.ClaimStrings{audience},
			Subject:   fmt.Sprintf("%d", userID),
			IssuedAt:  jwt.NewNumericDate(now),
			NotBefore: jwt.NewNumericDate(now),
			ExpiresAt: jwt.NewNumericDate(now.Add(24 * time.Hour)),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(signingKey(cfg.JWTSecret, sessionSecret))
}

func JWTAuth(cfg *config.Config, db *gorm.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		header := c.GetHeader("Authorization")
		if header == "" {
			utils.Error(c, http.StatusUnauthorized, "missing authorization header")
			c.Abort()
			return
		}

		parts := strings.SplitN(header, " ", 2)
		if len(parts) != 2 || parts[0] != "Bearer" {
			utils.Error(c, http.StatusUnauthorized, "invalid authorization format")
			c.Abort()
			return
		}

		// Premier parse sans vérifier la signature pour extraire le UserID
		unverified := &Claims{}
		parser := jwt.NewParser(jwt.WithoutClaimsValidation())
		_, _, err := parser.ParseUnverified(parts[1], unverified)
		if err != nil || unverified.UserID == 0 {
			utils.Error(c, http.StatusUnauthorized, "invalid token")
			c.Abort()
			return
		}

		// Récupérer le SessionSecret de l'user en DB
		var user models.User
		if err := db.First(&user, unverified.UserID).Error; err != nil {
			utils.Error(c, http.StatusUnauthorized, "user not found")
			c.Abort()
			return
		}

		// Vérification complète avec la clé composée
		claims := &Claims{}
		token, err := jwt.ParseWithClaims(parts[1], claims, func(t *jwt.Token) (interface{}, error) {
			if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
				return nil, fmt.Errorf("unexpected signing method: %v", t.Header["alg"])
			}
			return signingKey(cfg.JWTSecret, user.SessionSecret), nil
		},
			jwt.WithIssuer(issuer),
			jwt.WithAudience(audience),
			jwt.WithValidMethods([]string{"HS256"}),
			jwt.WithExpirationRequired(),
		)

		if err != nil || !token.Valid {
			utils.Error(c, http.StatusUnauthorized, "invalid or expired token")
			c.Abort()
			return
		}

		if claims.NotBefore != nil && claims.NotBefore.Time.After(time.Now()) {
			utils.Error(c, http.StatusUnauthorized, "token not yet valid")
			c.Abort()
			return
		}

		if claims.Subject != fmt.Sprintf("%d", claims.UserID) {
			utils.Error(c, http.StatusUnauthorized, "token claims mismatch")
			c.Abort()
			return
		}

		if len(cfg.JWTSecret) == 0 {
			utils.Error(c, http.StatusInternalServerError, "server misconfiguration")
			c.Abort()
			return
		}

		c.Set("user_id", claims.UserID)
		c.Next()
	}
}

// CompareTokenHash compare deux tokens de manière constant-time pour éviter les timing attacks.
func CompareTokenHash(a, b []byte) bool {
	return hmac.Equal(a, b)
}
