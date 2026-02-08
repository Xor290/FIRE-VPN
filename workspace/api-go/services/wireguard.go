package services

import (
	"crypto/rand"
	"encoding/base64"
	"fmt"
	"net"
	"strings"

	"vpn-api/db"
	"vpn-api/models"

	"golang.org/x/crypto/curve25519"
	"gorm.io/gorm"
)

type KeyPair = models.KeyPair

func GenerateKeyPair() (*KeyPair, error) {
	var privateKey [32]byte
	if _, err := rand.Read(privateKey[:]); err != nil {
		return nil, fmt.Errorf("failed to generate private key: %w", err)
	}

	privateKey[0] &= 248
	privateKey[31] &= 127
	privateKey[31] |= 64

	publicKey, err := curve25519.X25519(privateKey[:], curve25519.Basepoint)
	if err != nil {
		return nil, fmt.Errorf("failed to derive public key: %w", err)
	}

	return &KeyPair{
		PrivateKey: base64.StdEncoding.EncodeToString(privateKey[:]),
		PublicKey:  base64.StdEncoding.EncodeToString(publicKey),
	}, nil
}

func AllocatePeerIP(gormDB *gorm.DB, server *models.VPNServer) (string, error) {
	_, ipNet, err := net.ParseCIDR(server.Subnet)
	if err != nil {
		return "", fmt.Errorf("invalid subnet %s: %w", server.Subnet, err)
	}

	peers, _ := db.GetPeersByServer(gormDB, server.ID)
	usedIPs := make(map[string]bool)
	for _, p := range peers {
		usedIPs[strings.Split(p.AllowedIP, "/")[0]] = true
	}

	ip := make(net.IP, len(ipNet.IP))
	copy(ip, ipNet.IP)

	for inc(ip); ipNet.Contains(ip); inc(ip) {
		candidate := ip.String()
		if ip[len(ip)-1] == 1 {
			continue
		}
		if !usedIPs[candidate] {
			return candidate + "/32", nil
		}
	}

	return "", fmt.Errorf("no available IP in subnet %s", server.Subnet)
}

func inc(ip net.IP) {
	for j := len(ip) - 1; j >= 0; j-- {
		ip[j]++
		if ip[j] > 0 {
			break
		}
	}
}

func GenerateClientConfig(peerPrivateKey string, peerIP string, server *models.VPNServer) string {
	return fmt.Sprintf(`[Interface]
PrivateKey = %s
Address = %s
DNS = 1.1.1.1, 8.8.8.8

[Peer]
PublicKey = %s
Endpoint = %s:%d
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
`, peerPrivateKey, peerIP, server.PublicKey, server.IP, server.ListenPort)
}
