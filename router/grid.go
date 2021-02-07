package router

// Direction is a type of direction
type Direction uint

const (
	// DirectionX is the x direction
	DirectionX Direction = iota
	// DirectionY is the y direction
	DirectionY
)

// PairInt represents an Integer pair
type PairInt [2]int

// X of the point
func (p PairInt) X() int {
	return p[0]
}

// Y of the point
func (p PairInt) Y() int {
	return p[1]
}

// SetX to some value
func (p *PairInt) SetX(i int) {
	p[0] = i
}

// SetY to some value
func (p *PairInt) SetY(i int) {
	p[1] = i
}

// Point represents a 2D point
type Point = PairInt

// Segment is a pair of points
type Segment [2]Point

// Source of the segment
func (seg Segment) Source() Point {
	return seg[0]
}

// Target of the segment
func (seg Segment) Target() Point {
	return seg[1]
}

// SetSource to a point
func (seg *Segment) SetSource(p Point) {
	seg[0] = p
}

// SetTarget to a point
func (seg *Segment) SetTarget(p Point) {
	seg[1] = p
}

// Direction can only be x or y
func (seg Segment) Direction() Direction {
	if seg.Source().X() == seg.Target().X() {
		return DirectionX
	}
	AssertEqual(seg.Source().Y(), seg.Target().Y())
	return DirectionY
}

// X is only used when the segment is horizontal
func (seg Segment) X() int {
	AssertEqual(seg.Source().X(), seg.Target().X())
	return seg.Source().X()
}

// Y is only used when the segment is vertical
func (seg Segment) Y() int {
	AssertEqual(seg.Source().Y(), seg.Target().Y())
	return seg.Source().Y()
}

// Box is defined by its 4 sides
type Box [4]int

// MakeBox makes a box
func MakeBox(top, bottom, left, right int) Box {
	AssertTrue(bottom <= top)
	AssertTrue(left <= right)
	return Box{top, bottom, left, right}
}

// NewBox creates a pointer to a new box
func NewBox(top, bottom, left, right int) *Box {
	box := MakeBox(top, bottom, left, right)
	return &box
}

// Top returns the top side of the box
func (box Box) Top() int {
	return box[0]
}

// Bottom returns the bottom side of the box
func (box Box) Bottom() int {
	return box[1]
}

// Left returns the left side of the box
func (box Box) Left() int {
	return box[2]
}

// Right returns the right side of the box
func (box Box) Right() int {
	return box[3]
}

// SetTop sets the top side of a box
func (box *Box) SetTop(i int) {
	box[0] = i
}

// SetBottom sets the bottom side of a box
func (box *Box) SetBottom(i int) {
	box[1] = i
}

// SetLeft sets the left side of a box
func (box *Box) SetLeft(i int) {
	box[2] = i
}

// SetRight sets the right side of a box
func (box *Box) SetRight(i int) {
	box[3] = i
}

// CenterX2 is center times 2 for comparison purposes
func (box Box) CenterX2() PairInt {
	return PairInt{box.Left() + box.Right(), box.Top() + box.Bottom()}
}

// Width of the box
func (box Box) Width() int {
	return box.Right() - box.Left()
}

// Height of the box
func (box Box) Height() int {
	return box.Top() - box.Bottom()
}
