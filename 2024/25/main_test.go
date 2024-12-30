package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt")
	if result != 3 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt"))
}
