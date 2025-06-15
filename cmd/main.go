package main

import "fmt"
import "github.com/fouadsfarijlani/marbles/pkg/node"

func main() {
	fmt.Printf("Welcome to Marbles\n")
	node_1 := new(node.Node)
	node_1.Id = 1
	node_2 := node.Node{}
	node_2.Id = 2

	node_1.AddSibling(&node_2)
	fmt.Printf("%p, %v\n", node_1, *node_1)
}
