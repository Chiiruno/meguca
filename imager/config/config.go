// Package config stores and exports the configuration for server-side use and
// the public availability JSON struct, which includes a small subset of the
// server configuration.
package config

import (
	"sync"
)

var (
	// Ensures no reads happen, while the configuration is reloading
	globalMu sync.RWMutex

	// Contains currently loaded global server configuration
	global *Configs
)

// Uploads size constraints
type UploadMaximums struct {
	// Max size in MB
	Size float64

	// Max width in pixels
	Width uint64

	// Max height in pixels
	Height uint64
}

// Upload configurations
type Uploads struct {
	// Use JPEG thumbnails instead of WEBP
	JPEGThumbnails bool `json:"jpeg_thumbnails"`

	// Uploads size constraints
	Max UploadMaximums
}

// Global server configurations exposed to the client
type Public struct {
	//  Enable captchas and antispam
	EnableAntispam bool `json:"enable_antispam"`

	// Upload configurations
	Uploads Uploads
}

/// Global server configurations
type Configs struct {
	// Global server configurations exposed to the client
	Public Public
}

// Get returns a pointer to the current server configuration struct. Callers
// should not modify this struct.
func Get() *Configs {
	globalMu.RLock()
	defer globalMu.RUnlock()
	return global
}

// Set sets the internal configuration struct
func Set(c Configs) {
	globalMu.Lock()
	defer globalMu.Unlock()
	global = &c
}

// Clear resets package state. Only use in tests.
func Clear() {
	globalMu.Lock()
	defer globalMu.Unlock()
	global = &Configs{}
}
