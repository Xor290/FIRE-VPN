package config

import (
	"fmt"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	Port      string
	DSN       string
	JWTSecret string
	SSHKey    string
}

func Load() *Config {
	godotenv.Load()

	dsn := fmt.Sprintf(
		"host=%s port=%s user=%s password=%s dbname=%s sslmode=%s",
		getEnv("DB_HOST", "localhost"),
		getEnv("DB_PORT", "5432"),
		getEnv("DB_USER", "vpn_admin"),
		getEnv("DB_PASSWORD", ""),
		getEnv("DB_NAME", "vpn_db"),
		getEnv("DB_SSLMODE", "disable"),
	)

	return &Config{
		Port:      getEnv("API_PORT", "8080"),
		DSN:       dsn,
		JWTSecret: getEnv("JWT_SECRET", ""),
		SSHKey:    getEnv("SSH_KEY_PATH", "~/.ssh/id_rsa"),
	}
}

func getEnv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
