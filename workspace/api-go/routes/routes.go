package routes

import (
	"vpn-api/config"
	"vpn-api/handlers"
	"vpn-api/middleware"
	"vpn-api/services"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

func Setup(r *gin.Engine, db *gorm.DB, cfg *config.Config, sshClient *services.SSHClient) {
	authHandler := &handlers.AuthHandler{DB: db, Cfg: cfg}
	serverHandler := &handlers.ServerHandler{DB: db}
	vpnHandler := &handlers.VPNHandler{DB: db, Cfg: cfg, SSH: sshClient}
	profileHandler := &handlers.ProfileHandler{DB: db}
	auth := r.Group("/auth")
	{
		auth.POST("/register", authHandler.Register)
		auth.POST("/login", authHandler.Login)
	}

	profile := r.Group("/profile")
	profile.Use(middleware.JWTAuth(cfg, db))
	{
		profile.GET("/info", profileHandler.ProfileInfo)
		profile.PUT("/update", profileHandler.ProfileUpdate)
		profile.DELETE("/delete", profileHandler.ProfileDelete)
	}
	vpn := r.Group("/vpn")
	vpn.Use(middleware.JWTAuth(cfg, db))
	{
		vpn.GET("/servers", serverHandler.ListServers)
		vpn.POST("/connect", vpnHandler.Connect)
		vpn.POST("/disconnect", vpnHandler.Disconnect)
		vpn.GET("/status", vpnHandler.Status)
	}
}
