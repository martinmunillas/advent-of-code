package main

import (
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
		curr = ""
	}
	for j := 0; j < columns; j++ {
		for i := 0; i < rows; i++ {
			if foundXmas(&curr, matrix[i][j]) {
				result++
			}
		}
		curr = ""
	}
	for sum := 0; sum < rows+columns-1; sum++ {
		for i := 0; i < rows; i++ {
			j := sum - i
			if j >= 0 && j < columns {
				r, c := i, j
				if foundXmas(&curr, matrix[r][c]) {
					result++
				}
			}
		}
		curr = ""
	}
	for sum := 0; sum < rows+columns-1; sum++ {
		for i := 0; i < rows; i++ {
			j := sum - (rows - 1 - i)
			if j >= 0 && j < columns {
				r, c := i, j
				if foundXmas(&curr, matrix[r][c]) {
					result++
				}
			}
		}
		curr = ""
	}
	return result
}

func B(file string) int {
	result := 0

	return result
}
