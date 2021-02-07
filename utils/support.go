package utils

// None takes up no space
type None struct {
}

// Exist is used with sets
var Exist = None{}

// Any can be anything
type Any interface {
}
