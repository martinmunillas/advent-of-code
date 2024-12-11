package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getStones(file string) []int {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	s := []int{}
	for _, c := range strings.Split(string(f), " ") {
		n, err := strconv.Atoi(c)
		if err != nil {
			panic(err)
		}
		s = append(s, n)
	}
	return s

}

func blink(stones []int, times int) []int {
	if times == 0 {
		return stones
	}
	newStones := make([]int, 0, len(stones))
	for _, stone := range stones {
		if stone == 0 {
			newStones = append(newStones, 1)
			continue
		}
		str := fmt.Sprintf("%d", stone)
		if len(str)%2 == 0 {
			left, err := strconv.Atoi(str[:(len(str) / 2)])
			if err != nil {
				panic(err)
			}
			right, err := strconv.Atoi(str[(len(str) / 2):])
			if err != nil {
				panic(err)
			}
			newStones = append(newStones, left, right)
			continue
		}
		newStones = append(newStones, stone*2024)
	}
	return blink(newStones, times-1)
}

func A(file string) int {
	stones := getStones(file)
	return len(blink(stones, 25))
}

func makeId(stone int, times int) string { return fmt.Sprintf("%d-%d", stone, times) }

func blink2(stone int, times int, memo map[string]int) int {
	if times == 0 {
		return 1
	}
	id := makeId(stone, times)
	if result, ok := memo[id]; ok {
		return result
	}

	result := 0
	str := fmt.Sprintf("%d", stone)
	if stone == 0 {
		result = blink2(1, times-1, memo)
	} else if len(str)%2 == 0 {
		left, err := strconv.Atoi(str[:(len(str) / 2)])
		if err != nil {
			panic(err)
		}
		right, err := strconv.Atoi(str[(len(str) / 2):])
		if err != nil {
			panic(err)
		}
		result = blink2(left, times-1, memo) + blink2(right, times-1, memo)
	} else {
		result = blink2(stone*2024, times-1, memo)
	}
	memo[id] = result
	return result
}

func B(file string) int {
	result := 0
	stones := getStones(file)
	for _, stone := range stones {
		result += blink2(stone, 75, map[string]int{})
	}
	return result
}
