package main

import "fmt"
import "github.com/fouadsfarijlani/marbles/pkg/node"

func main() {
	fmt.Printf("Welcome to Marbles\n")
	node_1 := new(node.Node)
	node_1.Id = 1
	node_2 := node.Node{}
	node_2.Id = 2
	node_2.Name = "Some Name"

	node_1.AddIncoming(&node_2)
	node_1.Show()
}
