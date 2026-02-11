package services

import (
	"fmt"
	"os"
	"time"

	"vpn-api/models"

	"golang.org/x/crypto/ssh"
)

type SSHClient struct {
	KeyPath string
}

func NewSSHClient(keyPath string) *SSHClient {
	return &SSHClient{KeyPath: keyPath}
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

	config := &ssh.ClientConfig{
		User:            "ubuntu",
		Auth:            []ssh.AuthMethod{ssh.PublicKeys(signer)},
		HostKeyCallback: ssh.InsecureIgnoreHostKey(),
		Timeout:         10 * time.Second,
	}

	client, err := ssh.Dial("tcp", serverIP+":22", config)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to %s: %w", serverIP, err)
	}

	return client, nil
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
