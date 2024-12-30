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
	seqToTotal := make(map[[4]int]int)
	nums := parse(file)

	for _, num := range nums {

		buyer := []int{num % 10}
		for i := 0; i < 2000; i++ {
			num = next(num)
			buyer = append(buyer, num%10)
		}

		seen := make(map[[4]int]struct{})
		for i := 0; i < len(buyer)-4; i++ {
			a, b, c, d, e := buyer[i], buyer[i+1], buyer[i+2], buyer[i+3], buyer[i+4]
			seq := [4]int{b - a, c - b, d - c, e - d}

			if _, found := seen[seq]; found {
				continue
			}
			seen[seq] = struct{}{}

			if _, found := seqToTotal[seq]; !found {
				seqToTotal[seq] = 0
			}
			seqToTotal[seq] += e
		}
	}

	maxValue := 0
	for _, total := range seqToTotal {
		if total > maxValue {
			maxValue = total
		}
	}

	return maxValue
}
