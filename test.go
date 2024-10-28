package main 

import (
	"crypto/sha256"
	"endcoding/hex"
	"time"
)

type Block struct {
	Index int  // Blok numarası
	Timestamp string // Oluşturulma zamanı
	Data string  // Blok içerisindeki veri
	PrevHash string // Önceki bloğumuzun hash değeri
	Hash string  // Bu bloğun hash değeri
}

// Hash hesaplama fonksiyonu.

func calculateHash(block Block) string {
	record := string(block.Index) + block.Timestamp + block.Data + block.PrevHash
    hash := sha256.New()
    hash.Write([]byte(record))
    return hex.EncodeToString(hash.Sum(nil))
}

// Yeni blok oluşturulumu.