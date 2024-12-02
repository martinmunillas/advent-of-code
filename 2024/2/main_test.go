package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt")
	if result != 2 {
		t.Errorf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	result := B("example.txt")
	if result != 4 {
		t.Errorf("Invalid b example result: %d", result)
	}
	fmt.Println(B("input.txt"))
}