package parser

import "errors"

// ErrWrongFormat indicates a formatting error
var ErrWrongFormat = errors.New("Wrong format")

// ErrEOF indicates an EOF is encountered way too early
var ErrEOF = errors.New("EOF encountered")

// ErrUnfinished indicates the file continues for too long
// which implies that some part is missed
var ErrUnfinished = errors.New("EOF expected")
