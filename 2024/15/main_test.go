package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example2.txt")
	if result != 2028 {
		t.Fatalf("Invalid a example2 result: %d", result)
	}
	result = A("example.txt")
	if result != 10092 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	B("input.txt")
}
