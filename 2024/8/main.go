package main

import (
	"image"
	"os"
	"strings"
)

func getMap(file string) ([][]rune, map[rune][]image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var matrix [][]rune
	antenas := map[rune][]image.Point{}
	for i, line := range strings.Split(string(f), "\n") {
		var row []rune
		for j, c := range line {
			row = append(row, c)
			if c != '.' {
				point := image.Pt(j, i)
				if _, ok := antenas[c]; !ok {
					antenas[c] = []image.Point{point}
				} else {
					antenas[c] = append(antenas[c], point)
				}
			}
		}
		matrix = append(matrix, row)
	}
	return matrix, antenas
}
func A(file string) int {
	antinodes := map[image.Point]interface{}{}
	matrix, antenas := getMap(file)
	for _, positions := range antenas {
		for i, a := range positions {
			for _, b := range positions[i+1:] {
				diff := a.Sub(b)
				newA := a.Add(diff)
				newB := b.Sub(diff)
				if newA.X >= 0 && newA.X < len(matrix[0]) && newA.Y >= 0 && newA.Y < len(matrix) {
					antinodes[newA] = true
				}
				if newB.X >= 0 && newB.X < len(matrix[0]) && newB.Y >= 0 && newB.Y < len(matrix) {
					antinodes[newB] = true
				}
			}
		}
	}
	return len(antinodes)
}

func B(file string) int {
	antinodes := map[image.Point]interface{}{}
	matrix, antenas := getMap(file)
	for _, positions := range antenas {
		for i, a := range positions {
			for _, b := range positions[i+1:] {
				diff := a.Sub(b)
				antinodes[a] = true
				antinodes[b] = true
				newA := a.Add(diff)
				newB := b.Sub(diff)
				for newA.X >= 0 && newA.X < len(matrix[0]) && newA.Y >= 0 && newA.Y < len(matrix) {
					antinodes[newA] = true
					newA = newA.Add(diff)
				}
				for newB.X >= 0 && newB.X < len(matrix[0]) && newB.Y >= 0 && newB.Y < len(matrix) {
					antinodes[newB] = true
					newB = newB.Sub(diff)
				}
			}
		}
	}
	return len(antinodes)
}
