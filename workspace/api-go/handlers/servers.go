package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"vpn-api/db"
	"vpn-api/utils"
)

func (h *ServerHandler) ListServers(c *gin.Context) {
	servers, err := db.GetActiveServers(h.DB)
	if err != nil {
		utils.Error(c, http.StatusInternalServerError, "failed to fetch servers")
		return
	}

	utils.Success(c, http.StatusOK, servers)
}
