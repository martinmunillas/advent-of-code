package main

import (
	"fmt"
	"testing"
)

func TestA(t *testing.T) {
	result := A("example.txt", 6, 6, 12)
	if result != 22 {
		t.Fatalf("Invalid a example result: %d", result)
	}
	fmt.Println(A("input.txt", 70, 70, 1024))
}

func TestB(t *testing.T) {
	result := B("example.txt", 6, 6, 12)
	if result != "6,1" {
		t.Fatalf("Invalid b example result: %s", result)
	}
	fmt.Println(B("input.txt", 70, 70, 1024))
}
