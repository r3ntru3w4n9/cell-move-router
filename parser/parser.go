package parser

import (
	"io/ioutil"
	"strconv"
	"strings"
)

// RoutingParser defines the parser
type RoutingParser interface {
	// Parse parses the
	Parse(content []string) error
	// Result defines what the result should be
	Result()
}

// ReadAndParse reads the file into parse
func ReadAndParse(parser RoutingParser, filename string) {
	byteArr, err := ioutil.ReadFile(filename)
	PanicIfNotNull(err)

	data := string(byteArr)
	content := strings.Fields(data)

	parser.Parse(content)

	parser.Result()
}

// Atoi converts from string to int
func Atoi(s string) int {
	i, err := strconv.Atoi(s)
	PanicIfNotNull(err)
	return i
}
