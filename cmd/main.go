package main

import "fmt"

import "github.com/fouadsfarijlani/marbles/pkg/property"

func main() {
	fmt.Printf("Welcome to Marbles\n")
	name := "age"
	var value int32 = 32
	nullable := false
	property := property.NewIntegerProperty(&name, &value, &nullable)

	property.Show()
	fmt.Printf("%v", property)
}
