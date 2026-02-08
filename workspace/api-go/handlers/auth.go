package handlers

import (
	"net/http"

	"vpn-api/db"
	"vpn-api/helpers"
	"vpn-api/middleware"
	"vpn-api/models"
	"vpn-api/utils"

	"github.com/gin-gonic/gin"
)

func (h *AuthHandler) Register(c *gin.Context) {
	var req registerRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		utils.Error(c, http.StatusBadRequest, err.Error())
		return
	}

	if _, err := db.GetUserByEmail(h.DB, req.Email); err == nil {
		utils.Error(c, http.StatusConflict, "email already registered")
		return
	}

	if _, err := db.GetUserByUsername(h.DB, req.Username); err == nil {
		utils.Error(c, http.StatusConflict, "username already taken")
		return
	}

	user := models.User{Username: req.Username, Email: req.Email}
	if err := helpers.SetPassword(&user, req.Password); err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to hash password")
		return
	}

	if err := helpers.GenerateSessionSecret(&user); err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to generate session secret")
		return
	}

	if err := db.CreateUser(h.DB, &user); err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to create user")
		return
	}

	token, err := middleware.GenerateToken(user.ID, user.SessionSecret, h.Cfg)
	if err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to generate token")
		return
	}

	utils.Success(c, http.StatusCreated, gin.H{
		"user":  gin.H{"id": user.ID, "username": user.Username, "email": user.Email},
		"token": token,
	})
}

func (h *AuthHandler) Login(c *gin.Context) {
	var req loginRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		utils.Error(c, http.StatusBadRequest, err.Error())
		return
	}

	user, err := db.GetUserByEmail(h.DB, req.Email)
	if err != nil {
		utils.Error(c, http.StatusUnauthorized, "invalid credentials")
		return
	}

	if !helpers.CheckPassword(user, req.Password) {
		utils.Error(c, http.StatusUnauthorized, "invalid credentials")
		return
	}

	if err := helpers.GenerateSessionSecret(user); err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to generate session secret")
		return
	}
	if err := db.UpdateUser(h.DB, user); err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to update session")
		return
	}

	token, err := middleware.GenerateToken(user.ID, user.SessionSecret, h.Cfg)
	if err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to generate token")
		return
	}

	utils.Success(c, http.StatusOK, gin.H{
		"user":  gin.H{"id": user.ID, "username": user.Username, "email": user.Email},
		"token": token,
	})
}
