package router

// TreeNet is an implementation of a net
type TreeNet struct {
	node *TreeNode
}

// TreeNode represents a single node in a tree
type TreeNode struct {
	left, rigth *TreeNode
	data        Any
}

// NewTreeNet creates a new TreeNet from connected pins and given segments
func NewTreeNet(points []Point, segments []Segment) *TreeNet {
	allPins := make(map[Point]None)
	groupByX := make(map[int][]Segment)
	groupByY := make(map[int][]Segment)

	for _, seg := range segments {
		switch seg.Direction() {
		case DirectionX:
			x := seg.X()
			if list, ok := groupByX[x]; ok {
				groupByX[x] = append(list, seg)
			} else {
				groupByX[x] = []Segment{seg}
			}
		case DirectionY:
			y := seg.Y()
			if list, ok := groupByY[y]; ok {
				groupByY[y] = append(list, seg)
			} else {
				groupByY[y] = []Segment{seg}
			}
		default:
			Unreachable()
		}

		source := seg.Source()
		target := seg.Target()

		allPins[source] = Exist
		allPins[target] = Exist
	}

	for _, pin := range points {
		_, ok := allPins[pin]
		AssertTrue(ok)
	}

	allPinsList := make([]Point, 0)
	for key := range allPins {
		allPinsList = append(allPinsList, key)
	}

	allPinsIdx := make(map[Point]int)
	for idx, elem := range allPinsList {
		allPinsIdx[elem] = idx
	}

	pinsUF := MakeUnionFind(len(allPins))
	noRedundantSegs := make([]Segment, 0)
	for _, seg := range segments {
		source := seg.Source()
		target := seg.Target()

		sIdx := allPinsIdx[source]
		tIdx := allPinsIdx[target]

		if ha, hb, same := pinsUF.SameGroupHead(sIdx, tIdx); !same {
			noRedundantSegs = append(noRedundantSegs, seg)
			pinsUF.UnionHead(ha, hb)
		}
	}

	Todo("find all pseudo pins")
	Todo("create a tree representation")

	return nil
}
