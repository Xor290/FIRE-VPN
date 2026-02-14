// handlers/vpn.go
package handlers

import (
	"log"
	"net/http"
	"vpn-api/db"
	"vpn-api/models"
	"vpn-api/services"
	"vpn-api/utils"

	"github.com/gin-gonic/gin"
)

func (h *VPNHandler) Connect(c *gin.Context) {
	userID := c.GetUint("user_id")
	var req connectRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		utils.Error(c, http.StatusBadRequest, err.Error())
		return
	}

	server, err := db.GetServerByID(h.DB, req.ServerID)
	if err != nil {
		utils.Error(c, http.StatusNotFound, "server not found")
		return
	}

	if !server.IsActive {
		utils.Error(c, http.StatusBadRequest, "server is not active")
		return
	}

	if _, err := db.GetPeerByUserAndServer(h.DB, userID, req.ServerID); err == nil {
		utils.Error(c, http.StatusConflict, "already connected to this server")
		return
	}

	keyPair, err := services.GenerateKeyPair()
	if err != nil {
		log.Printf("[CONNECT] failed to generate keys: %v", err)
		utils.Error(c, http.StatusInternalServerError, "failed to generate keys")
		return
	}

	peerIP, err := services.AllocatePeerIP(h.DB, server)
	if err != nil {
		log.Printf("[CONNECT] failed to allocate IP: %v", err)
		utils.Error(c, http.StatusInternalServerError, "failed to allocate IP")
		return
	}

	log.Printf("[CONNECT] adding peer on %s (key=%s, ip=%s)", server.IP, keyPair.PublicKey, peerIP)
	if err := h.SSH.AddPeer(server, keyPair.PublicKey, peerIP); err != nil {
		log.Printf("[CONNECT] SSH error: %v", err)
		utils.Error(c, http.StatusInternalServerError, "failed to add peer on VPS")
		return
	}

	peer := models.Peer{
		UserID:     userID,
		ServerID:   server.ID,
		PublicKey:  keyPair.PublicKey,
		PrivateKey: keyPair.PrivateKey,
		AllowedIP:  peerIP,
	}

	if err := db.CreatePeer(h.DB, &peer); err != nil {
		_ = h.SSH.RemovePeer(server, keyPair.PublicKey)
		utils.Error(c, http.StatusInternalServerError, "failed to save peer")
		return
	}

	clientConfig := services.GenerateClientConfig(keyPair.PrivateKey, peerIP, server)
	utils.Success(c, http.StatusCreated, gin.H{
		"peer_ip": peerIP,
		"config":  clientConfig,
	})
}

func (h *VPNHandler) Disconnect(c *gin.Context) {
	userID := c.GetUint("user_id")
	var req disconnectRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		utils.Error(c, http.StatusBadRequest, err.Error())
		return
	}

	peer, err := db.GetPeerByUserAndServer(h.DB, userID, req.ServerID)
	if err != nil {
		utils.Error(c, http.StatusNotFound, "no active connection to this server")
		return
	}

	server, err := db.GetServerByID(h.DB, req.ServerID)
	if err != nil {
		utils.Error(c, http.StatusNotFound, "server not found")
		return
	}

	log.Printf("[DISCONNECT] removing peer from %s (key=%s)", server.IP, peer.PublicKey)

	// Supprimer le peer de la BDD d'abord
	if err := db.DeletePeer(h.DB, peer); err != nil {
		log.Printf("[DISCONNECT] failed to delete peer from DB: %v", err)
		utils.Error(c, http.StatusInternalServerError, "failed to delete peer")
		return
	}

	// Essayer de supprimer du VPS (ne pas bloquer si ça échoue)
	if err := h.SSH.RemovePeer(server, peer.PublicKey); err != nil {
		log.Printf("[DISCONNECT] warning: failed to remove peer from VPS: %v", err)
		// Ne pas retourner d'erreur, la connexion est déjà supprimée de la BDD
	}

	utils.Success(c, http.StatusOK, gin.H{"message": "disconnected"})
}

func (h *VPNHandler) Status(c *gin.Context) {
	userID := c.GetUint("user_id")
	peers, _ := db.GetPeersByUser(h.DB, userID)
	utils.Success(c, http.StatusOK, peers)
}
