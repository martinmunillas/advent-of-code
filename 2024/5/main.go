package main

import (
	"os"
	"slices"
	"strconv"
)

func parseOrderings(file string) (map[int][]int, [][]int) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	orderings := map[int][]int{}
	updates := [][]int{}
	currNumber := ""
	currLine := []int{}
	isUpdates := false
	for i, c := range f {
		if c == '\n' && f[i-1] == '\n' {
			isUpdates = true
			continue
		} else if c == '|' || c == '\n' || c == ',' {
			n, err := strconv.Atoi(currNumber)
			if err != nil {
				panic(err)
			}
			currLine = append(currLine, n)
			currNumber = ""
		} else {
			currNumber += string(c)
		}

		if c == '\n' {
			if isUpdates {
				updates = append(updates, currLine)
			} else {
				if _, ok := orderings[currLine[0]]; !ok {
					orderings[currLine[0]] = []int{currLine[1]}
				} else {
					orderings[currLine[0]] = append(orderings[currLine[0]], currLine[1])
				}
			}
			currLine = nil
		}
	}
	n, err := strconv.Atoi(currNumber)
	if err != nil {
		panic(err)
	}
	currLine = append(currLine, n)
	currNumber = ""
	updates = append(updates, currLine)
	return orderings, updates
}

func A(file string) int {
	result := 0
	orderings, updates := parseOrderings(file)
looking:
	for _, update := range updates {
		for i, n := range update[:len(update)-1] {
			m := update[i+1]
			if !slices.Contains(orderings[n], m) {
				continue looking
			}
		}
		result += update[int(float32(len(update)-1)/2+0.6)]
	}

	return result
}

func B(file string) int {
	result := 0
	orderings, updates := parseOrderings(file)
	for _, update := range updates {
		isSorted := true
		for i, n := range update[:len(update)-1] {
			m := update[i+1]
			if !slices.Contains(orderings[n], m) {
				isSorted = false
				break
			}
		}
		if isSorted {
			continue
		}

		slices.SortFunc(update, func(a int, b int) int {
			isGreater := slices.Contains(orderings[a], b)
			if isGreater {
				return -1
			} else {
				return 1
			}
		})

		result += update[int(float32(len(update)-1)/2+0.6)]
	}

	return result
}
