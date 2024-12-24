package main

import (
	"fmt"
	"image"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func parseCorruptions(file string) (corruptions []image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	for _, l := range strings.Split(string(f), "\n") {
		parts := strings.Split(l, ",")
		x, err := strconv.Atoi(parts[0])
		if err != nil {
			panic(err)
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			panic(err)
		}
		corruptions = append(corruptions, image.Pt(x, y))
	}

	return corruptions
}

type Step struct {
	X    int
	Y    int
	Cost int
}

func bfs(xSize, ySize int, corruptions []image.Point) int {

	seen := map[image.Point]interface{}{}
	q := []Step{{}}

	for len(q) > 0 {
		var node Step
		node, q = q[0], q[1:]
		point := image.Pt(node.X, node.Y)

		if (node.X < 0 || node.Y < 0 || node.X > xSize || node.Y > ySize) || slices.Contains(corruptions, point) {
			continue
		}
		if seen[point] != nil {
			continue
		}
		seen[point] = true

		if node.X == xSize && node.Y == ySize {
			return node.Cost
		}

		for _, dir := range []Step{
			{X: 1, Y: 0},
			{X: -1, Y: 0},
			{X: 0, Y: 1},
			{X: 0, Y: -1},
		} {
			next := node
			next.X += dir.X
			next.Y += dir.Y
			next.Cost += 1

			q = append(q, next)

		}
	}
	return math.MaxInt
}

func A(file string, xSize int, ySize int, bytes int) int {
	return bfs(xSize, ySize, parseCorruptions(file)[:bytes])
}

func B(file string, xSize int, ySize int, bytes int) string {
	corruptions := parseCorruptions(file)
	for i := bytes; i < len(corruptions); i++ {
		if bfs(xSize, ySize, corruptions[:i]) == math.MaxInt {
			return fmt.Sprintf("%d,%d", corruptions[i-1].X, corruptions[i-1].Y)
		}
	}
	return ""
}
