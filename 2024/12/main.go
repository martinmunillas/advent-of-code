package main

import (
	"image"
	"os"
	"slices"
	"strings"
)

func getMap(file string) [][]rune {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var matrix [][]rune
	for _, line := range strings.Split(string(f), "\n") {
		var row []rune
		for _, c := range line {
			row = append(row, c)
		}
		matrix = append(matrix, row)
	}
	return matrix
}

func exploreRegion(matrix [][]rune, start image.Point, visited map[image.Point]interface{}) (int, int) {

	kind := matrix[start.Y][start.X]
	stack := []image.Point{start}
	directions := []image.Point{
		image.Pt(0, 1),
		image.Pt(1, 0),
		image.Pt(0, -1),
		image.Pt(-1, 0),
	}
	perimeter := 0
	area := 0
	visited[start] = true
	for len(stack) > 0 {
		curr := stack[0]
		stack = stack[1:]
		for _, direction := range directions {
			newCoord := curr.Add(direction)
			if _, ok := visited[newCoord]; ok {
				continue
			}
			isOutBounds := newCoord.X < 0 || newCoord.X >= len(matrix[0]) || newCoord.Y < 0 || newCoord.Y >= len(matrix)
			if isOutBounds || matrix[newCoord.Y][newCoord.X] != kind {
				perimeter++
			} else {
				visited[newCoord] = true
				stack = append(stack, newCoord)
			}
		}
		area++
	}

	return perimeter, area

}

func A(file string) int {
	result := 0
	matrix := getMap(file)
	visited := map[image.Point]interface{}{}
	for i, row := range matrix {
		for j := range row {
			curr := image.Pt(j, i)
			if _, ok := visited[curr]; ok {
				continue
			}
			region := map[image.Point]interface{}{}
			perimeter, area := exploreRegion(matrix, curr, region)
			for k := range region {
				visited[k] = true
			}
			result += perimeter * area
		}
	}
	return result
}

type Edge struct {
	In  image.Point
	Out image.Point
}

func exploreRegion2(matrix [][]rune, start image.Point, visited map[image.Point]interface{}) (int, int) {

	kind := matrix[start.Y][start.X]
	stack := []image.Point{start}
	perimeter := []Edge{}
	area := 0
	visited[start] = true
	for len(stack) > 0 {
		curr := stack[0]
		stack = stack[1:]
		for _, direction := range []image.Point{
			image.Pt(0, 1),
			image.Pt(1, 0),
			image.Pt(0, -1),
			image.Pt(-1, 0),
		} {
			newCoord := curr.Add(direction)
			if _, ok := visited[newCoord]; ok {
				continue
			}
			isOutBounds := newCoord.X < 0 || newCoord.X >= len(matrix[0]) || newCoord.Y < 0 || newCoord.Y >= len(matrix)
			if isOutBounds || matrix[newCoord.Y][newCoord.X] != kind {
				perimeter = append(perimeter, Edge{In: curr, Out: newCoord})
			} else {
				visited[newCoord] = true
				stack = append(stack, newCoord)
			}
		}
		area++
	}

	redundant := 0
	for _, edge := range perimeter {
		for _, direction := range []image.Point{
			image.Pt(1, 0),
			image.Pt(0, 1),
		} {
			newEdge := Edge{In: edge.In.Add(direction), Out: edge.Out.Add(direction)}
			if slices.Contains(perimeter, newEdge) {
				redundant++
			}
		}
	}

	sides := len(perimeter) - redundant

	return sides, area

}

func B(file string) int {
	result := 0
	matrix := getMap(file)
	visited := map[image.Point]interface{}{}
	for i, row := range matrix {
		for j := range row {
			curr := image.Pt(j, i)
			if _, ok := visited[curr]; ok {
				continue
			}
			region := map[image.Point]interface{}{}
			sides, area := exploreRegion2(matrix, curr, region)
			for k := range region {
				visited[k] = true
			}
			result += area * sides
		}
	}
	return result
}
