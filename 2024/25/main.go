package main

import (
	"os"
	"strings"
)

func parseHeights(file string) (locks [][]int, keys [][]int) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	for _, g := range strings.Split(string(f), "\n\n") {
		lines := strings.Split(g, "\n")
		curr := make([]int, len(lines[0]))
		isLock := g[0] == '#' && g[1] == '#' && g[2] == '#'
		for _, l := range lines {
			for i, c := range l {
				if c == '#' {
					curr[i]++
				}
			}
		}
		if isLock {
			locks = append(locks, curr)
		} else {
			keys = append(keys, curr)
		}

	}
	return locks, keys
}

func A(file string) int {
	locks, keys := parseHeights(file)
	result := 0
	for _, lock := range locks {
		for _, key := range keys {
			can := true
			for i := range lock {
				if lock[i]+key[i] > 7 {
					can = false
					break
				}
			}
			if can {
				result++
			}
		}
	}
	return result
}
