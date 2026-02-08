package models

import (
	"time"

	"gorm.io/gorm"
)

type VPNServer struct {
	ID         uint           `json:"id" gorm:"primaryKey"`
	Name       string         `json:"name" gorm:"not null"`
	Country    string         `json:"country" gorm:"not null"`
	IP         string         `json:"ip" gorm:"not null"`
	PublicKey  string         `json:"public_key" gorm:"not null"`
	PrivateKey string         `json:"-" gorm:"not null"`
	ListenPort int            `json:"listen_port" gorm:"default:51820"`
	Subnet     string         `json:"subnet" gorm:"not null"` // ex: "10.0.1.0/24"
	IsActive   bool           `json:"is_active" gorm:"default:true"`
	CreatedAt  time.Time      `json:"created_at"`
	UpdatedAt  time.Time      `json:"updated_at"`
	DeletedAt  gorm.DeletedAt `json:"-" gorm:"index"`
	Peers      []Peer         `json:"-" gorm:"foreignKey:ServerID"`
}
