package db

import (
	"gorm.io/gorm"
	"vpn-api/models"
)

func GetPeerByUserAndServer(db *gorm.DB, userID, serverID uint) (*models.Peer, error) {
	var peer models.Peer
	err := db.Where("user_id = ? AND server_id = ?", userID, serverID).First(&peer).Error
	return &peer, err
}

func GetPeersByUser(db *gorm.DB, userID uint) ([]models.Peer, error) {
	var peers []models.Peer
	err := db.Preload("Server").Where("user_id = ?", userID).Find(&peers).Error
	return peers, err
}

func GetPeersByServer(db *gorm.DB, serverID uint) ([]models.Peer, error) {
	var peers []models.Peer
	err := db.Where("server_id = ?", serverID).Find(&peers).Error
	return peers, err
}

func CreatePeer(db *gorm.DB, peer *models.Peer) error {
	return db.Create(peer).Error
}

func DeletePeer(db *gorm.DB, peer *models.Peer) error {
	return db.Delete(peer).Error
}
