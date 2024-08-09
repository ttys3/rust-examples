package main

import (
	"bytes"
	"crypto/aes"
	"encoding/hex"
	"fmt"
)

// pkcs7Padding pads the plaintext with PKCS#7 padding scheme
func pkcs7Padding(plaintext []byte, blockSize int) []byte {
	padding := blockSize - len(plaintext)%blockSize
	padtext := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(plaintext, padtext...)
}

// aesEncryptECB encrypts the plaintext using AES-128 in ECB mode
func aesEncryptECB(plaintext, key []byte) ([]byte, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return nil, err
	}

	blockSize := block.BlockSize()
	plaintext = pkcs7Padding(plaintext, blockSize)

	ciphertext := make([]byte, len(plaintext))

	for i := 0; i < len(plaintext); i += blockSize {
		block.Encrypt(ciphertext[i:i+blockSize], plaintext[i:i+blockSize])
	}

	return ciphertext, nil
}

func main() {
	key := []byte("1234567890abcdef")
	plaintext := "hello world! this is my plaintext."

	fmt.Printf("Plaintext: %s\n", plaintext)

	// Encrypt the plaintext
	ciphertext, err := aesEncryptECB([]byte(plaintext), key)
	if err != nil {
		fmt.Printf("Error encrypting: %v\n", err)
		return
	}

	// Encode to hex for readable output
	encodedCiphertext := hex.EncodeToString(ciphertext)
	fmt.Printf("Ciphertext (hex): %s\n", encodedCiphertext)
	// 52e47e367c5de0c3d6f22ae852461998429d768c476c20aa151a043638fd2690e7b712b0e9d864e209126fe912f82b96
}
