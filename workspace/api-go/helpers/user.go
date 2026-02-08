package helpers

import (
	"crypto/rand"
	"encoding/hex"
	"vpn-api/models"

	"golang.org/x/crypto/bcrypt"
)

func SetPassword(u *models.User, plain string) error {
	hash, err := bcrypt.GenerateFromPassword([]byte(plain), bcrypt.DefaultCost)
	if err != nil {
		return err
	}
	u.Password = string(hash)
	return nil
}

func CheckPassword(u *models.User, plain string) bool {
	return bcrypt.CompareHashAndPassword([]byte(u.Password), []byte(plain)) == nil
}

func GenerateSessionSecret(u *models.User) error {
	b := make([]byte, 32)
	if _, err := rand.Read(b); err != nil {
		return err
	}
	u.SessionSecret = hex.EncodeToString(b)
	return nil
}
