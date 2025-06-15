package node

type Node struct {
	Id       int32 //temporary
	name     string
	siblings []*Node
}

func (self *Node) AddSibling(sibling *Node) {
	self.siblings = append(self.siblings, sibling)
}
