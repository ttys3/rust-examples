package main

import (
    "bytes"
    "crypto/aes"
    "crypto/cipher"
    "crypto/rand"
    "encoding/hex"
    "fmt"
    "io"
)

// pkcs7Padding pads the input to be a multiple of the AES block size using the PKCS7 standard.
func pkcs7Padding(data []byte, blockSize int) []byte {
    padding := blockSize - len(data)%blockSize
    padtext := bytes.Repeat([]byte{byte(padding)}, padding)
    return append(data, padtext...)
}

// encryptAES128CBC encrypts the plaintext using AES-128 in CBC mode with PKCS7 padding.
func encryptAES128CBC(plaintext, key []byte) ([]byte, error) {
    block, err := aes.NewCipher(key)
    if err != nil {
        return nil, err
    }

    blockSize := block.BlockSize()
    plaintext = pkcs7Padding(plaintext, blockSize)

    ciphertext := make([]byte, blockSize+len(plaintext))
    iv := ciphertext[:blockSize]

    if _, err := io.ReadFull(rand.Reader, iv); err != nil {
        return nil, err
    }

    mode := cipher.NewCBCEncrypter(block, iv)
    mode.CryptBlocks(ciphertext[blockSize:], plaintext)

    return ciphertext, nil
}

func main() {
    key := []byte("1234567890abcdef")
    plaintext := []byte("hello world! this is my plaintext.")

    ciphertext, err := encryptAES128CBC(plaintext, key)
    if err != nil {
        fmt.Printf("Error encrypting: %v\n", err)
        return
    }

    fmt.Printf("Ciphertext (hex): %s\n", hex.EncodeToString(ciphertext))
}