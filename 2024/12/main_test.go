package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example2.txt")
	if result != 140 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	result = A("example.txt")
	if result != 1930 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	result := B("example2.txt")
	if result != 80 {
		t.Fatalf("Invalid b example result: %d", result)
	}
	fmt.Println(B("input.txt"))
}
