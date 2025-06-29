package node

import "fmt"

type Node struct {
	Id       int32 //temporary
	Name     string
	Data     map[string]interface{}
	Incoming []*Node
	Outgoing []*Node
}

func (self *Node) AddIncoming(sibling *Node) {
	self.Incoming = append(self.Incoming, sibling)
}

func (self *Node) AddOutgoing(sibling *Node) {
	self.Outgoing = append(self.Outgoing, sibling)
}

func (self *Node) ShowOutGoing() {
	fmt.Printf("---- Node   %v outgoing:\n", self.Id)
	fmt.Printf("%-20v %-20v %-20v\n", "id", "name", "address")

	for _, s := range self.Outgoing {
		fmt.Printf("%-20v%-20v%-20v\n", s.Name, s.Id, &s)
	}
}

func (self *Node) ShowIncoming() {
	fmt.Printf("---- Node %v incoming:\n", self.Id)
	fmt.Printf("%-20v %-20v %-20v\n", "id", "name", "address")

	for _, s := range self.Incoming {
		fmt.Printf("%-20v%-20v%-20v\n", s.Name, s.Id, &s)
	}

}

func (self *Node) Show() {
	self.ShowIncoming()
	self.ShowOutGoing()
}
