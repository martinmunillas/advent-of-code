package main

import (
	"fmt"
	"image"
	"os"
	"strings"
)

func getMap(file string) ([][]rune, image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var matrix [][]rune
	var start image.Point
	for i, line := range strings.Split(string(f), "\n") {
		var row []rune
		for j, c := range line {
			row = append(row, c)
			if c == '^' || c == '>' || c == '<' || c == 'v' {
				start = image.Pt(j, i)
			}
		}
		matrix = append(matrix, row)
	}
	return matrix, start
}

var turn = map[rune]rune{
	'^': '>',
	'>': 'v',
	'<': '^',
	'v': '<',
}

func printMatrix(matrix [][]rune) {
	for _, line := range matrix {
		for _, c := range line {
			fmt.Print(string(c))
		}
		fmt.Println()
	}
}

func explore(matrix [][]rune, curr image.Point, visited map[image.Point]interface{}) {
	visited[curr] = true
	next := curr
	c := matrix[curr.Y][curr.X]
	switch c {
	case '^':
		next = next.Add(image.Pt(0, -1))
	case '>':
		next = next.Add(image.Pt(1, 0))
	case '<':
		next = next.Add(image.Pt(-1, 0))
	case 'v':
		next = next.Add(image.Pt(0, 1))
	default:
		panic(fmt.Sprintf("invalid character %s", string(c)))
	}
	// printMatrix(matrix)
	if next.Y < 0 || next.Y > len(matrix)-1 || next.X < 0 || next.X > len(matrix[0])-1 {
		return
	}
	if matrix[next.Y][next.X] == '#' {
		matrix[curr.Y][curr.X] = turn[c]
		next = curr
	} else {
		matrix[next.Y][next.X] = c
	}

	explore(matrix, next, visited)
}

func A(file string) int {
	matrix, start := getMap(file)
	visited := map[image.Point]interface{}{}
	explore(matrix, start, visited)
	return len(visited)
}

func B(file string) int {
	result := 0

	return result
}
