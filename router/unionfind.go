package router

// UnionFindNode represents a node in UnionFind
type UnionFindNode struct {
	head, depth int
}

// MakeUnionFindNode creates a new UnionFindNode
func MakeUnionFindNode(head int) UnionFindNode {
	return UnionFindNode{head, 0}
}

// Head shows the head of a UnionFindNode
func (ufn UnionFindNode) Head() int {
	return ufn.head
}

// Depth shows the depth of a UnionFindNode
func (ufn UnionFindNode) Depth() int {
	return ufn.depth
}

// SetHead of the UnionFindNode
func (ufn *UnionFindNode) SetHead(h int) {
	ufn.head = h
}

// IncDepth of the UnionFindNode
func (ufn *UnionFindNode) IncDepth() {
	ufn.depth++
}

// SetDepth of the UnionFindNode
func (ufn *UnionFindNode) SetDepth(h int) {
	ufn.depth = h
}

// UnionFind algorithm
type UnionFind struct {
	list []UnionFindNode
}

// MakeUnionFind creates a new UnionFind
func MakeUnionFind(size int) UnionFind {
	list := make([]UnionFindNode, size)
	for i := 0; i < size; i++ {
		list[i] = MakeUnionFindNode(i)
	}
	return UnionFind{list}
}

// SameGroup shows whether two indices are in the same union
func (uf *UnionFind) SameGroup(a, b int) bool {
	_, _, same := uf.SameGroupHead(a, b)
	return same
}

// SameGroupHead shows whether two indices are in the same union and return their head
func (uf *UnionFind) SameGroupHead(a, b int) (int, int, bool) {
	headA := uf.Find(a)
	headB := uf.Find(b)

	return headA, headB, headA == headB
}

// Union two groups
func (uf *UnionFind) Union(a, b int) {
	ha, hb, same := uf.SameGroupHead(a, b)
	if !same {
		uf.UnionHead(ha, hb)
	}
}

// UnionHead two groups by their head indices
func (uf *UnionFind) UnionHead(ha, hb int) {
	nodeA := &uf.list[ha]
	nodeB := &uf.list[hb]

	depthA := nodeA.Depth()
	depthB := nodeB.Depth()

	if depthA == depthB {
		nodeA.IncDepth()
		nodeB.SetHead(ha)
	} else if depthA > depthB {
		nodeB.SetHead(ha)
	} else {
		AssertTrue(depthA < depthB)
		nodeA.SetHead(hb)
	}
}

// Find the group of a node
func (uf *UnionFind) Find(i int) int {
	node := uf.list[i]
	head := node.head
	if i == head {
		return i
	}
	groupHead := uf.Find(head)
	node.SetHead(groupHead)
	return groupHead
}

// Depth of the head node of the current group
func (uf *UnionFind) Depth(i int) int {
	head := uf.Find(i)
	return uf.list[head].Depth()
}
