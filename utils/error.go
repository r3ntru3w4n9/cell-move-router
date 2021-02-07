package utils

import "fmt"

// PanicIfNotNull panics if the error is not null
func PanicIfNotNull(err error) {
	if err != nil {
		panicIfOn(err)
	}
}

// AssertTrue asserts if a condition is true
func AssertTrue(a bool) {
	if a {
		return
	}
	panicIfOn("AssertTrue")
}

// AssertFalse asserts if a condition is true
func AssertFalse(a bool) {
	if !a {
		return
	}
	panicIfOn("AssertFalse")
}

// AssertEqual asserts if two things are equal
func AssertEqual(a, b Any) {
	if a == b {
		return
	}
	panicIfOn(fmt.Sprintf("AssertEqual: %v == %v", a, b))
}

// AssertNotEqual asserts if two things are equal
func AssertNotEqual(a, b Any) {
	if a != b {
		return
	}
	panicIfOn(fmt.Sprintf("AssertNotEqual: %v != %v\n", a, b))
}
