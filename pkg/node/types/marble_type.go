package node

type marbleType interface {
	validate() bool
	constraint()
}

type marbleInteger struct {
	value int32
}
