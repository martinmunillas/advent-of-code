package main

import (
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func A(file string) int {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\n")
	result := 0
	listA := []int{}
	listB := []int{}
	for _, line := range lines {
		values := strings.Split(line, "   ")
		a, err := strconv.Atoi(values[0])
		if err != nil {
			panic(err)
		}
		b, err := strconv.Atoi(values[1])
		if err != nil {
			panic(err)
		}
		listA = append(listA, a)
		listB = append(listB, b)
	}
	slices.Sort(listA)
	slices.Sort(listB)
	for i, a := range listA {
		b := listB[i]
		points := int(math.Abs(float64(a) - float64(b)))
		result += points
	}
	return result
}

func B(file string) int {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\n")
	result := 0
	listA := []int{}
	listB := []int{}
	for _, line := range lines {
		values := strings.Split(line, "   ")
		a, err := strconv.Atoi(values[0])
		if err != nil {
			panic(err)
		}
		b, err := strconv.Atoi(values[1])
		if err != nil {
			panic(err)
		}
		listA = append(listA, a)
		listB = append(listB, b)
	}
	slices.Sort(listA)
	slices.Sort(listB)
	counts := make(map[int]int, 0)

	for _, a := range listA {
		if _, ok := counts[a]; ok {
			result += a * counts[a]
			continue
		}
		count := 0
		for _, b := range listB {
			if b > a {
				break
			} else if b == a {
				count += 1
			}
		}
		result += a * count
		counts[a] = count
	}
	return result
}
