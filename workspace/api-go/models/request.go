package models

type RegisterRequest struct {
	Username string `json:"username" binding:"required,min=3,max=32"`
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required,min=8"`
}

type LoginRequest struct {
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required"`
}

type ConnectRequest struct {
	ServerID uint `json:"server_id" binding:"required"`
}

type DisconnectRequest struct {
	ServerID uint `json:"server_id" binding:"required"`
}
