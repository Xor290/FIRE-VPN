package models

import (
	"time"

	"gorm.io/gorm"
)

type Peer struct {
	ID         uint           `json:"id" gorm:"primaryKey"`
	UserID     uint           `json:"user_id" gorm:"not null;index"`
	ServerID   uint           `json:"server_id" gorm:"not null;index"`
	PublicKey  string         `json:"public_key" gorm:"not null"`
	PrivateKey string         `json:"-" gorm:"not null"`
	AllowedIP  string         `json:"allowed_ip" gorm:"not null"` // IP attribuée au peer dans le subnet
	CreatedAt  time.Time      `json:"created_at"`
	DeletedAt  gorm.DeletedAt `json:"-" gorm:"index"`
	User       User           `json:"-" gorm:"foreignKey:UserID"`
	Server     VPNServer      `json:"server" gorm:"foreignKey:ServerID"`
}
