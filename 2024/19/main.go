package main

import (
	"os"
	"slices"
	"strings"
)

func parse(file string) ([]string, []string) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	parts := strings.Split(string(f), "\n\n")

	return strings.Split(parts[0], ", "), strings.Split(parts[1], "\n")
}

func isPossible(design string, patterns []string, maxLength int, memo map[string]bool) bool {
	if v, ok := memo[design]; ok {
		return v
	}
	if len(design) == 0 {
		return true
	}

	for i := 1; i <= maxLength; i++ {
		if i > len(design) {
			break
		}
		if slices.Contains(patterns, design[:i]) {
			if isPossible(design[i:], patterns, maxLength, memo) {
				memo[design] = true
				return true
			}
		}
	}
	memo[design] = false
	return false
}

func A(file string) int {
	result := 0
	patterns, designs := parse(file)
	maxLength := 0
	for _, pattern := range patterns {
		if maxLength < len(pattern) {
			maxLength = len(pattern)
		}
	}
	for _, design := range designs {
		if isPossible(design, patterns, maxLength, map[string]bool{}) {
			result += 1
		}
	}
	return result
}

func possibleWays(design string, patterns []string, maxLength int, memo map[string]int) int {
	if v, ok := memo[design]; ok {
		return v
	}
	if len(design) == 0 {
		return 1
	}
	count := 0
	for i := 1; i <= maxLength; i++ {
		if i > len(design) {
			break
		}
		if slices.Contains(patterns, design[:i]) {
			count += possibleWays(design[i:], patterns, maxLength, memo)
		}
	}
	memo[design] = count
	return count
}

func B(file string) int {
	result := 0
	patterns, designs := parse(file)
	maxLength := 0
	for _, pattern := range patterns {
		if maxLength < len(pattern) {
			maxLength = len(pattern)
		}
	}
	for _, design := range designs {
		result += possibleWays(design, patterns, maxLength, map[string]int{})
	}
	return result
}
