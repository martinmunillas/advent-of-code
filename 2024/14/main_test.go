package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt", 11, 7)
	if result != 12 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt", 101, 103))
}

func TestB(t *testing.T) {
	B("input.txt", 101, 103)
}
