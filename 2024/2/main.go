package main

import (
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
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
	reports := [][]int{}
	for _, line := range strings.Split(string(f), "\n") {
		report := []int{}
		for _, s := range strings.Split(line, " ") {
			n, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}
			report = append(report, n)
		}
		reports = append(reports, report)
	}
	for _, report := range reports {
		if ReportIsValid(report) {
			result++
		}
	}
	return result
}

func B(file string) int {
	f, _ := os.ReadFile(file)
	result := 0
	reports := [][]int{}
	for _, line := range strings.Split(string(f), "\n") {
		report := []int{}
		for _, s := range strings.Split(line, " ") {
			n, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}
			report = append(report, n)
		}
		reports = append(reports, report)
	}

	for _, report := range reports {
		if ReportIsValid(report) {
			result++
			continue
		}
		for i := 0; i < len(report); i++ {
			r := make([]int, len(report))
			copy(r, report)
			reportWithoutOne := slices.Delete(r, i, i+1)
			if ReportIsValid(reportWithoutOne) {
				result++
				break
			}

		}
	}
	return result
}
