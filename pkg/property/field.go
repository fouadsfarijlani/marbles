package property

import "reflect"

type MarbleFieldType string

const (
	Integer MarbleFieldType = "integer"
	String  MarbleFieldType = "string"
)

type MarbleField[T any] struct {
	FieldType MarbleFieldType
	size      uintptr
	Value     T
}

func (self *MarbleField[T]) CalculateValueSize() {
	self.size = reflect.TypeOf(self.Value).Size()
}

func (self *MarbleField[T]) UpdateValye(value *T) {
	self.Value = *value
}

func NewField[T any](fieldType MarbleFieldType, value *T) MarbleField[T] {
	newField := MarbleField[T]{
		FieldType: fieldType,
		Value:     *value,
	}
	newField.CalculateValueSize()

	return newField
}
