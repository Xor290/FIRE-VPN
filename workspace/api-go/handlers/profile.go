package handlers

import (
	"net/http"
	"vpn-api/db"
	"vpn-api/models"

	"github.com/gin-gonic/gin"
)

func (p *ProfileHandler) ProfileInfo(c *gin.Context) {
	userID := c.GetUint("user_id")

	getInfoUser, err := db.GetUserByID(p.DB, userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"user": getInfoUser})
}

func (p *ProfileHandler) ProfileUpdate(c *gin.Context) {
	userID := c.GetUint("user_id") // récupéré depuis le middleware JWT par ex.

	var req models.UpdateProfileRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request body"})
		return
	}

	user, err := db.GetUserByID(p.DB, userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "User not found"})
		return
	}

	user.Username = req.Username
	user.Email = req.Email
	user.Password = req.Password
	if err := db.UpdateUser(p.DB, user); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update profile"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Profile updated successfully", "user": user})
}

func (p *ProfileHandler) ProfileDelete(c *gin.Context) {
	userID := c.GetUint("user_id")

	if err := db.DeleteUser(p.DB, userID); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to delete profile"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Profile deleted successfully"})
}
