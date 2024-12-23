package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt")
	if result != 7036 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	result = A("example2.txt")
	if result != 11048 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	result := B("example.txt")
	if result != 45 {
		t.Fatalf("Invalid b example result: %d", result)
	}
	result = B("example2.txt")
	if result != 64 {
		t.Fatalf("Invalid b example2 result: %d", result)
	}
	fmt.Println(B("input.txt"))
}
