package utils

// Todo marks a line of code as unfinished and state the purpose of the missing part
func Todo(s string) {
	panicIfOn("todo: " + s)
}

// Unimplemented marks a line of code as not yet finished
func Unimplemented() {
	panicIfOn("unimplemented")
}

// Unreachable marks a line of code unreachable
func Unreachable() {
	panicIfOn("unreachable")
}
