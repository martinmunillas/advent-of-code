package main

import (
	"math"
	"os"
	"regexp"
	"strconv"
)

func ReportIsValid(report []int) bool {
	for i := 0; i < len(report)-2; i++ {
		a := report[i]
		b := report[i+1]
		c := report[i+2]
		diffA := math.Abs(float64(a - b))
		diffB := math.Abs(float64(b - c))
		if !(((a < b) == (b < c)) && (diffA > 0 && diffA < 4) && (diffB > 0 && diffB < 4)) {
			return false
		}
	}

	return true
}

func A(file string) int {
	f, _ := os.ReadFile(file)
	result := 0
	regex := regexp.MustCompile(`mul\(\d+,\d+\)`)
	multiplications := regex.FindAll(f, -1)
	for _, mul := range multiplications {
		a := 0
		b := 0
		curr := ""
		for _, c := range mul[4:] {
			if c == ',' {
				a, _ = strconv.Atoi(curr)
				curr = ""
			} else if c == ')' {
				b, _ = strconv.Atoi(curr)
			} else {
				curr += string(c)
			}
		}
		result += a * b
	}
	return result
}

func B(file string) int {
	f, _ := os.ReadFile(file)
	result := 0
	regex := regexp.MustCompile(`(?:mul\(\d+,\d+\))|(?:do(?:n't)?\(\))`)
	multiplications := regex.FindAll(f, -1)
	enabled := true
	for _, mul := range multiplications {
		if mul[0] == 'd' {
			enabled = len(mul) == 4
		} else if enabled {
			a := 0
			b := 0
			curr := ""
			for _, c := range mul[4:] {
				if c == ',' {
					a, _ = strconv.Atoi(curr)
					curr = ""
				} else if c == ')' {
					b, _ = strconv.Atoi(curr)
				} else {
					curr += string(c)
				}
			}
			result += a * b
		}
	}
	return result
}
