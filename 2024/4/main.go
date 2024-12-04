package main

import (
	"math"
	"os"
	"strings"
)

func foundXmas(str *string, c rune) bool {
	*str += string(c)
	if len(*str) > 4 {
		*str = (*str)[1:]
	}
	return *str == "XMAS" || *str == "SAMX"
}

func A(file string) int {
	f, _ := os.ReadFile(file)
	result := 0
	matrix := [][]rune{}
	for _, line := range strings.Split(string(f), "\n") {
		row := []rune{}
		for _, s := range line {
			row = append(row, s)
		}
		matrix = append(matrix, row)
	}
	rows := len(matrix)
	columns := len(matrix[0])
	curr := ""
	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			if foundXmas(&curr, matrix[i][j]) {
				result++
			}
		}
	}
	curr = ""
	for j := 0; j < columns; j++ {
		for i := 0; i < rows; i++ {
			if foundXmas(&curr, matrix[i][j]) {
				result++
			}
		}
	}
	curr = ""
	for i := 1; i <= (rows + columns - 1); i++ {
		startCol := int(math.Max(0, float64(i-rows)))
		count := int(math.Min(math.Min(float64(i), (float64(columns-startCol))), float64(rows)))
		for j := 0; j < count; j++ {
			if foundXmas(&curr, matrix[int(math.Min(float64(rows), float64(i)))-j-1][startCol+j]) {
				result++
			}
		}

	}
	curr = ""
	for i := 1; i <= (rows + columns - 1); i++ {
		startCol := int(math.Max(0, float64(i-rows)))
		count := int(math.Min(math.Min(float64(i), (float64(columns-startCol))), float64(rows)))
		for j := count - 1; j >= 0; j-- {
			if foundXmas(&curr, matrix[int(math.Min(float64(rows), float64(i)))-j-1][startCol+j]) {
				result++
			}
		}

	}
	curr = ""
	return result
}

func B(file string) int {
	result := 0

	return result
}
