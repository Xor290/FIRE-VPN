package db

import (
	"gorm.io/gorm"
	"vpn-api/models"
)

func GetActiveServers(db *gorm.DB) ([]models.VPNServer, error) {
	var servers []models.VPNServer
	err := db.Where("is_active = ?", true).Find(&servers).Error
	return servers, err
}

func GetServerByID(db *gorm.DB, id uint) (*models.VPNServer, error) {
	var server models.VPNServer
	err := db.First(&server, id).Error
	return &server, err
}
