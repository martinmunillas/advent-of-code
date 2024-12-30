package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt")
	if result != 4 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	result = A("example2.txt")
	if result != 2024 {
		t.Fatalf("Invalid a example2 result: %d", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	fmt.Println(B("input.txt"))
}
