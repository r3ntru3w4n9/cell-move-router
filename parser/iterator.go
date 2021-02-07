package parser

// Iterator iterates over a slice of string
type Iterator struct {
	cursor int
	data   []string
}

// MakeIterator creates a new Iterator
func MakeIterator(data []string) Iterator {
	return Iterator{cursor: -1, data: data}
}

// NewIterator creates a new Iterator
func NewIterator(data []string) *Iterator {
	iter := MakeIterator(data)
	return &iter
}

// Next yields the next string
func (iter *Iterator) Next() string {
	iter.cursor++
	return iter.data[iter.cursor]
}

// HasNext shows whether next can still be called
func (iter *Iterator) HasNext() bool {
	return iter.cursor < len(iter.data)
}
