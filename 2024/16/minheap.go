package main

type MinHeap []Node

func (h MinHeap) Len() int {
	return len(h)
}

func (h MinHeap) Less(i, j int) bool {
	return h[i].Cost < h[j].Cost
}

func (h MinHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *MinHeap) Push(val interface{}) {
	*h = append(*h, val.(Node))
}

func (h *MinHeap) Pop() interface{} {
	heapDerefrenced := *h

	size := len(heapDerefrenced)
	val := heapDerefrenced[size-1]
	*h = heapDerefrenced[:size-1]

	return val
}
