package handlers

import (
	"gorm.io/gorm"
	"vpn-api/config"
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


type registerRequest struct {
	Username string `json:"username" binding:"required,min=3,max=32"`
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required,min=8"`
}

type loginRequest struct {
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required"`
}

type connectRequest struct {
	ServerID uint `json:"server_id" binding:"required"`
}

type disconnectRequest struct {
	ServerID uint `json:"server_id" binding:"required"`
}
