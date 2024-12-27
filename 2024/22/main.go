package main

import (
	"os"
	"strconv"
	"strings"
)

func parse(file string) []int {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	all := []int{}
	for _, s := range strings.Split(string(f), "\n") {
		n, err := strconv.Atoi(s)
		if err != nil {
			panic(err)
		}
		all = append(all, n)
	}
	return all
}

func mix(n int, m int) int {
	return n ^ m
}

func prune(n int) int {
	return n % 16777216
}

func next(n int) int {
	n = prune(mix((n * 64), n))
	n = prune(mix(n/32, n))
	n = prune(mix(n*2048, n))
	return n
}
func A(file string) int {
	result := 0
	numbers := parse(file)
	for i := range numbers {
		for range 2000 {
			numbers[i] = next(numbers[i])
		}
		result += numbers[i]
	}
	return result
}

func B(file string) int {
	return 0
}
