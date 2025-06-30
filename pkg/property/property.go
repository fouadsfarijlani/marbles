package property

import "fmt"

type Property[T any] struct {
	InternalId int32
	Name       string
	Nullable   bool
	Field      MarbleField[T]
}

func (self Property[T]) Show() {
	fmt.Printf("id:%-10v\nname:%-10v\nnullable:%-10v\nvalue:%-10v\n", self.InternalId, self.Name, self.Nullable, self.Field.Value)
}

func newIntegerProperty[T any](name *string, value *T, nullable *bool, fieldType MarbleFieldType) Property[T] {
	field := NewField(fieldType, value)
	property := Property[T]{
		InternalId: 1, Name: *name, Nullable: *nullable,
		Field: field,
	}
	return property
}

func NewIntegerProperty(name *string, value *int32, nullable *bool) Property[int32] {
	return newIntegerProperty(name, value, nullable, Integer)
}
