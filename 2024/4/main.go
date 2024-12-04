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

func getMatrix(file string) [][]rune {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	matrix := [][]rune{}
	for _, line := range strings.Split(string(f), "\n") {
		row := []rune{}
		for _, s := range line {
			row = append(row, s)
		}
		matrix = append(matrix, row)
	}
	return matrix
}

func A(file string) int {
	result := 0
	matrix := getMatrix(file)
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
	matrix := getMatrix(file)
	for i, row := range matrix {
		for j, c := range row {
			if i == 0 || j == 0 || i == len(matrix)-1 || j == len(row)-1 {
				continue
			}
			if c == 'A' {
				diagA := string([]rune{matrix[i-1][j-1], c, matrix[i+1][j+1]})
				diagB := string([]rune{matrix[i-1][j+1], c, matrix[i+1][j-1]})
				if (diagA == "MAS" || diagA == "SAM") && (diagB == "MAS" || diagB == "SAM") {
					result++
				}

			}
		}
	}

	return result
}
