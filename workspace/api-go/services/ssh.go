package services

import (
	"fmt"
	"os"
	"path/filepath"
	"time"
	"vpn-api/models"

	"golang.org/x/crypto/ssh"
	"golang.org/x/crypto/ssh/knownhosts"
)

type SSHClient struct {
	KeyPath        string
	KnownHostsPath string
}

func NewSSHClient(keyPath, knownHostsPath string) *SSHClient {
	return &SSHClient{
		KeyPath:        keyPath,
		KnownHostsPath: knownHostsPath,
	}
}

func (s *SSHClient) connect(serverIP string) (*ssh.Client, error) {
	keyBytes, err := os.ReadFile(s.KeyPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read SSH key %s: %w", s.KeyPath, err)
	}

	signer, err := ssh.ParsePrivateKey(keyBytes)
	if err != nil {
		return nil, fmt.Errorf("failed to parse SSH key: %w", err)
	}

	// Créer le callback sécurisé pour la validation des host keys
	hostKeyCallback, err := s.getHostKeyCallback()
	if err != nil {
		return nil, fmt.Errorf("failed to setup host key validation: %w", err)
	}

	config := &ssh.ClientConfig{
		User:            "ubuntu",
		Auth:            []ssh.AuthMethod{ssh.PublicKeys(signer)},
		HostKeyCallback: hostKeyCallback,
		Timeout:         10 * time.Second,
	}

	client, err := ssh.Dial("tcp", serverIP+":22", config)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to %s: %w", serverIP, err)
	}

	return client, nil
}

func (s *SSHClient) getHostKeyCallback() (ssh.HostKeyCallback, error) {
	// Vérifier si le fichier known_hosts existe
	if _, err := os.Stat(s.KnownHostsPath); os.IsNotExist(err) {
		// Créer le répertoire si nécessaire
		dir := filepath.Dir(s.KnownHostsPath)
		if err := os.MkdirAll(dir, 0700); err != nil {
			return nil, fmt.Errorf("failed to create known_hosts directory: %w", err)
		}

		// Créer le fichier known_hosts vide
		if err := os.WriteFile(s.KnownHostsPath, []byte{}, 0600); err != nil {
			return nil, fmt.Errorf("failed to create known_hosts file: %w", err)
		}
	}

	// Utiliser knownhosts pour valider les host keys
	hostKeyCallback, err := knownhosts.New(s.KnownHostsPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load known_hosts: %w", err)
	}

	return hostKeyCallback, nil
}

// AddKnownHost ajoute une nouvelle host key au fichier known_hosts
func (s *SSHClient) AddKnownHost(hostname string, remote string, pubKey ssh.PublicKey) error {
	// Format: hostname ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC...
	line := knownhosts.Line([]string{hostname}, pubKey)

	f, err := os.OpenFile(s.KnownHostsPath, os.O_APPEND|os.O_WRONLY|os.O_CREATE, 0600)
	if err != nil {
		return fmt.Errorf("failed to open known_hosts: %w", err)
	}
	defer f.Close()

	if _, err := f.WriteString(line + "\n"); err != nil {
		return fmt.Errorf("failed to write to known_hosts: %w", err)
	}

	return nil
}

func (s *SSHClient) run(serverIP, command string) (string, error) {
	client, err := s.connect(serverIP)
	if err != nil {
		return "", err
	}
	defer client.Close()

	session, err := client.NewSession()
	if err != nil {
		return "", fmt.Errorf("failed to create SSH session: %w", err)
	}
	defer session.Close()

	output, err := session.CombinedOutput(command)
	if err != nil {
		return string(output), fmt.Errorf("command failed: %s: %w", string(output), err)
	}

	return string(output), nil
}

func (s *SSHClient) AddPeer(server *models.VPNServer, peerPublicKey, peerAllowedIP string) error {
	cmd := fmt.Sprintf("sudo wg set wg0 peer %s allowed-ips %s", peerPublicKey, peerAllowedIP)
	_, err := s.run(server.IP, cmd)
	if err != nil {
		return fmt.Errorf("failed to add peer on %s: %w", server.IP, err)
	}

	_, err = s.run(server.IP, "sudo wg-quick save wg0")
	if err != nil {
		return fmt.Errorf("failed to save wg config on %s: %w", server.IP, err)
	}

	return nil
}

func (s *SSHClient) RemovePeer(server *models.VPNServer, peerPublicKey string) error {
	cmd := fmt.Sprintf("sudo wg set wg0 peer %s remove", peerPublicKey)
	_, err := s.run(server.IP, cmd)
	if err != nil {
		return fmt.Errorf("failed to remove peer on %s: %w", server.IP, err)
	}

	_, err = s.run(server.IP, "sudo wg-quick save wg0")
	if err != nil {
		return fmt.Errorf("failed to save wg config on %s: %w", server.IP, err)
	}

	return nil
}
