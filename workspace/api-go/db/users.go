package db

import (
	"gorm.io/gorm"
	"vpn-api/models"
)

func GetUserByUsername(db *gorm.DB, username string) (*models.User, error) {
	var user models.User
	err := db.Where("username = ?", username).First(&user).Error
	return &user, err
}

func GetUserByEmail(db *gorm.DB, email string) (*models.User, error) {
	var user models.User
	err := db.Where("email = ?", email).First(&user).Error
	return &user, err
}

func CreateUser(db *gorm.DB, user *models.User) error {
	return db.Create(user).Error
}

func UpdateUser(db *gorm.DB, user *models.User) error {
	return db.Save(user).Error
}
