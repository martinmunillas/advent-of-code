package main

import (
	"fmt"
	"image"
	"os"
	"strings"
)

func parse(file string) ([][]rune, []rune, image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	parts := strings.Split(string(f), "\n\n")
	matrix := [][]rune{}
	var start image.Point
	for i, line := range strings.Split(parts[0], "\n") {
		matrix = append(matrix, []rune(line))
		xStart := strings.Index(line, "@")
		if xStart != -1 {
			start = image.Pt(xStart, i)
		}
	}
	instructions := []rune{}
	for _, c := range parts[1] {
		if c == '\n' {
			continue
		}
		instructions = append(instructions, c)
	}

	return matrix, instructions, start
}

var direction = map[rune]image.Point{
	'>': image.Pt(1, 0),
	'<': image.Pt(-1, 0),
	'v': image.Pt(0, 1),
	'^': image.Pt(0, -1),
}

func exec(instruction rune, matrix [][]rune, pos image.Point) image.Point {
	c := matrix[pos.Y][pos.X]
	nextPos := pos.Add(direction[instruction])

	if nextPos.X < 0 || nextPos.Y < 0 || nextPos.X > len(matrix[0]) || nextPos.Y > len(matrix) {
		return pos
	}

	if matrix[nextPos.Y][nextPos.X] == 'O' {
		exec(instruction, matrix, nextPos)
	}

	if matrix[nextPos.Y][nextPos.X] == '.' {
		matrix[nextPos.Y][nextPos.X] = c
		matrix[pos.Y][pos.X] = '.'
		return nextPos
	}

	return pos
}

func count(matrix [][]rune) int {
	points := 0
	for i, row := range matrix {
		for j, c := range row {
			if c == 'O' {
				points += i*100 + j
			}
		}
	}
	return points
}

func printMap(m [][]rune) {
	for _, l := range m {
		fmt.Println(string(l))
	}
}

func A(file string) int {
	matrix, instructions, pos := parse(file)
	for _, instruction := range instructions {
		pos = exec(instruction, matrix, pos)
	}

	return count(matrix)
}

func B(file string) {
}
