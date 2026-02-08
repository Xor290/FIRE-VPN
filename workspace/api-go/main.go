package main

import (
	"log"

	"github.com/gin-gonic/gin"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"vpn-api/config"
	"vpn-api/models"
	"vpn-api/routes"
	"vpn-api/services"
)

func main() {
	cfg := config.Load()

	db, err := gorm.Open(postgres.Open(cfg.DSN), &gorm.Config{})
	if err != nil {
		log.Fatal("failed to connect to database:", err)
	}
	db.AutoMigrate(&models.User{}, &models.VPNServer{}, &models.Peer{})

	sshClient := services.NewSSHClient(cfg.SSHKey)

	r := gin.Default()
	routes.Setup(r, db, cfg, sshClient)

	log.Printf("API starting on :%s", cfg.Port)
	if err := r.Run(":" + cfg.Port); err != nil {
		log.Fatal("failed to start server:", err)
	}
}
