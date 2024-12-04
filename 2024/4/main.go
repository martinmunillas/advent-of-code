package main

import (
	"math"
	"os"
	"strings"
)

func checkCharacter(str *string, c rune) bool {
	if c == 'X' {
		*str = "X"
		return false
	} else if c == 'M' && len(*str) == 1 {
		*str += "M"
		return false
	} else if c == 'A' && len(*str) == 2 {
		*str += "A"
		return false
	} else if c == 'S' && len(*str) == 3 {
		*str = ""
		return true
	} else {
		*str = ""
		return false
	}
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
			if foundWord := checkCharacter(&curr, matrix[i][j]); foundWord {
				result++
			}
		}
	}
	for j := 0; j < columns; j++ {
		for i := 0; i < rows; i++ {
			if foundWord := checkCharacter(&curr, matrix[i][j]); foundWord {
				result++
			}
		}
	}
	for i := rows - 1; i >= 0; i-- {
		for j := columns - 1; j >= 0; j-- {
			if foundWord := checkCharacter(&curr, matrix[i][j]); foundWord {
				result++
			}
		}
	}
	for j := columns - 1; j >= 0; j-- {
		for i := rows - 1; i >= 0; i-- {
			if foundWord := checkCharacter(&curr, matrix[i][j]); foundWord {
				result++
			}
		}
	}
	for i := 1; i <= (rows + columns - 1); i++ {
		startCol := int(math.Max(0, float64(i-rows)))
		count := int(math.Min(math.Min(float64(i), (float64(columns-startCol))), float64(rows)))
		for j := 0; j < count; j++ {
			if foundWord := checkCharacter(&curr, matrix[int(math.Min(float64(rows), float64(i)))-j-1][startCol+j]); foundWord {
				result++
			}
		}

	}
	for i := (rows + columns - 1); i > 0; i-- {
		startCol := int(math.Max(0, float64(i-rows)))
		count := int(math.Min(math.Min(float64(i), (float64(columns-startCol))), float64(rows)))
		for j := 0; j < count; j++ {
			if foundWord := checkCharacter(&curr, matrix[int(math.Min(float64(rows), float64(i)))-j-1][startCol+j]); foundWord {
				result++
			}
		}

	}
	for i := 1; i <= (rows + columns - 1); i++ {
		startCol := int(math.Max(0, float64(i-rows)))
		count := int(math.Min(math.Min(float64(i), (float64(columns-startCol))), float64(rows)))
		for j := count - 1; j >= 0; j-- {
			if foundWord := checkCharacter(&curr, matrix[int(math.Min(float64(rows), float64(i)))-j-1][startCol+j]); foundWord {
				result++
			}
		}

	}
	for i := (rows + columns - 1); i > 0; i-- {
		startCol := int(math.Max(0, float64(i-rows)))
		count := int(math.Min(math.Min(float64(i), (float64(columns-startCol))), float64(rows)))
		for j := count - 1; j >= 0; j-- {
			if foundWord := checkCharacter(&curr, matrix[int(math.Min(float64(rows), float64(i)))-j-1][startCol+j]); foundWord {
				result++
			}
		}

	}
	return result
}

func B(file string) int {
	result := 0

	return result
}
