package handlers

import (
	"gorm.io/gorm"
	"vpn-api/config"
	"vpn-api/models"
	"vpn-api/services"
)

type AuthHandler struct {
	DB  *gorm.DB
	Cfg *config.Config
}

type ServerHandler struct {
	DB *gorm.DB
}

type VPNHandler struct {
	DB  *gorm.DB
	Cfg *config.Config
	SSH *services.SSHClient
}

type registerRequest = models.RegisterRequest
type loginRequest = models.LoginRequest
type connectRequest = models.ConnectRequest
type disconnectRequest = models.DisconnectRequest
