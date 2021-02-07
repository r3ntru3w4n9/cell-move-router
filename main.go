package main

import (
	"os"

	"github.com/r3ntru3w4n9/delayed-routing/parser"
	"github.com/r3ntru3w4n9/delayed-routing/utils"
)

// NoDebug is overridden on command line to stop the
var NoDebug string

func main() {
	utils.HandleNoDebug(NoDebug)
	pars := parser.NewIccadParser()
	parser.ReadAndParse(pars, os.Args[1])
}
