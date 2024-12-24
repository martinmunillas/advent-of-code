package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt")
	if result != 7 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt"))
}

func TestB(t *testing.T) {
	result := B("example.txt")
	if result != "co,de,ka,ta" {
		t.Fatalf("Invalid b example result: %s", result)
	}
	fmt.Println(B("input.txt"))
}
