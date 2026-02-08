package db

import (
	"vpn-api/models"

	"gorm.io/gorm"
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

func GetUserByID(db *gorm.DB, userID uint) (*models.User, error) {
	var user models.User
	err := db.Where("ID = ?", userID).First(&user).Error
	return &user, err
}

func GetUserNameByID(db *gorm.DB, userID uint) (string, error) {
	var user models.User
	err := db.Where("ID = ?", userID).First(&user).Error
	if err != nil {
		return "", err
	}
	return user.Username, nil
}

func DeleteUser(db *gorm.DB, userID uint) error {
	return db.Delete(&models.User{}, userID).Error
}
