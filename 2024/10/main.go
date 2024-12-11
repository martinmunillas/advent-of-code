package main

import (
	"image"
	"os"
	"strconv"
	"strings"
)

func getMap(file string) [][]int {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var matrix [][]int
	for _, line := range strings.Split(string(f), "\n") {
		var row []int
		for _, c := range line {
			n, err := strconv.Atoi(string(c))
			if err != nil {
				panic(err)
			}
			row = append(row, n)
		}
		matrix = append(matrix, row)
	}
	return matrix
}

func getTrailheadPoints(matrix [][]int, coord image.Point, summits map[image.Point]interface{}) int {

	curr := matrix[coord.Y][coord.X]
	if curr == 9 {
		summits[coord] = true
		return len(summits)
	}

	directions := []image.Point{
		image.Pt(0, 1),
		image.Pt(1, 0),
		image.Pt(0, -1),
		image.Pt(-1, 0),
	}
	for _, direction := range directions {
		newCoord := coord.Add(direction)
		if newCoord.X < 0 || newCoord.X >= len(matrix[0]) || newCoord.Y < 0 || newCoord.Y >= len(matrix) {
			continue
		}
		if matrix[newCoord.Y][newCoord.X]-curr != 1 {
			continue
		}
		getTrailheadPoints(matrix, newCoord, summits)
	}
	return len(summits)
}

func A(file string) int {
	result := 0
	matrix := getMap(file)
	for i, row := range matrix {
		for j, n := range row {
			if n == 0 {
				result += getTrailheadPoints(matrix, image.Pt(j, i), map[image.Point]interface{}{})
			}
		}
	}
	return result
}

func getTrailheadRating(matrix [][]int, coord image.Point) int {
	curr := matrix[coord.Y][coord.X]
	if curr == 9 {
		return 1
	}

	reached := 0
	directions := []image.Point{
		image.Pt(0, 1),
		image.Pt(1, 0),
		image.Pt(0, -1),
		image.Pt(-1, 0),
	}
	for _, direction := range directions {
		newCoord := coord.Add(direction)
		if newCoord.X < 0 || newCoord.X >= len(matrix[0]) || newCoord.Y < 0 || newCoord.Y >= len(matrix) {
			continue
		}
		if matrix[newCoord.Y][newCoord.X]-curr != 1 {
			continue
		}
		reached += getTrailheadRating(matrix, newCoord)
	}
	return reached
}

func B(file string) int {
	result := 0
	matrix := getMap(file)
	for i, row := range matrix {
		for j, n := range row {
			if n == 0 {
				result += getTrailheadRating(matrix, image.Pt(j, i))
			}
		}
	}
	return result
}
