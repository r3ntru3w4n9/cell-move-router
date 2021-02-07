package utils

import (
	"log"
	"strings"
)

// HandleNoDebug handles NoDebug flag
func HandleNoDebug(noDebug string) {
	switch strings.ToLower(noDebug) {
	case "y", "yes", "true":
		log.Println("panic is turned off")
		panicInternalFlag = false
	default:
		log.Println("panic is turned on")
		panicInternalFlag = true
	}
}

var panicInternalFlag bool = true

func panicIfOn(value Any) {
	if panicInternalFlag {
		panic(value)
	}
}
