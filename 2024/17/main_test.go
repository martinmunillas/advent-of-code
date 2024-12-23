package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt")
	if result != "4,6,3,5,6,3,5,2,1,0" {
		t.Fatalf("Invalid a example result: %s", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	result := B("example.txt")
	if result != 0 {
		t.Fatalf("Invalid b example result: %d", result)
	}
	fmt.Println(B("input.txt"))
}
