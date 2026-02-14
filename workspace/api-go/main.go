package main

import (
	"log"
	"os"
	"path/filepath"

	"vpn-api/config"
	"vpn-api/models"
	"vpn-api/routes"
	"vpn-api/services"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func main() {
	cfg := config.Load()

	db, err := gorm.Open(postgres.Open(cfg.DSN), &gorm.Config{})
	if err != nil {
		log.Fatal("failed to connect to database:", err)
	}
	db.AutoMigrate(&models.User{}, &models.VPNServer{}, &models.Peer{})

	// Chemin vers le fichier known_hosts
	homeDir, err := os.UserHomeDir()
	if err != nil {
		log.Fatalf("Failed to get home directory: %v", err)
	}
	knownHostsPath := filepath.Join(homeDir, ".ssh", "known_hosts")

	// Initialiser le client SSH avec le chemin du known_hosts
	sshKeyPath := os.Getenv("SSH_KEY_PATH")
	if sshKeyPath == "" {
		sshKeyPath = filepath.Join(homeDir, ".ssh", "id_rsa")
	}
	sshClient := services.NewSSHClient(sshKeyPath, knownHostsPath)
	r := gin.Default()
	r.Use(cors.New(cors.Config{
		AllowOrigins: []string{
			"http://localhost:8080",
			"http://172.20.167.237",
		},
		AllowMethods:     []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
	}))
	routes.Setup(r, db, cfg, sshClient)

	log.Printf("API starting on :%s", cfg.Port)
	if err := r.Run(":" + cfg.Port); err != nil {
		log.Fatal("failed to start server:", err)
	}
}
