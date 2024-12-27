package main

import (
	"fmt"
	"image"
	"os"
	"slices"
	"strconv"
	"strings"
)

func parse(file string) []string {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), "\n")
}

var numericCoords = map[rune]image.Point{
	'7': image.Pt(0, 0),
	'8': image.Pt(1, 0),
	'9': image.Pt(2, 0),
	'4': image.Pt(0, 1),
	'5': image.Pt(1, 1),
	'6': image.Pt(2, 1),
	'1': image.Pt(0, 2),
	'2': image.Pt(1, 2),
	'3': image.Pt(2, 2),
	'0': image.Pt(1, 3),
	'A': image.Pt(2, 3),
}

var directionalCoords = map[rune]image.Point{
	'^': image.Pt(1, 0),
	'A': image.Pt(2, 0),
	'<': image.Pt(0, 1),
	'v': image.Pt(1, 1),
	'>': image.Pt(2, 1),
}

func solveCode(code string, robots int, memo map[string]int) int {
	result := 0
	prev := 'A'
	for _, curr := range code {
		instructions := fromTo(numericCoords, prev, curr, image.Pt(0, 3))
		results := make([]int, 0, len(instructions))
		for _, instruction := range instructions {
			results = append(results, directional(instruction, robots, memo))
		}
		result += slices.Min(results)
		prev = curr
	}

	return result
}

func directional(code string, depth int, memo map[string]int) int {
	key := code + fmt.Sprint(depth)
	if v, ok := memo[key]; ok {
		return v
	}

	result := 0
	prev := 'A'
	for _, curr := range code {
		instructions := fromTo(directionalCoords, prev, curr, image.Pt(0, 0))
		results := make([]int, 0, len(instructions))
		for _, seq := range instructions {
			if depth == 1 {
				results = append(results, len(seq))
			} else {
				results = append(results, directional(seq, depth-1, memo))
			}
		}
		result += slices.Min(results)
		prev = curr
	}

	memo[key] = result
	return result
}

func fromTo(coords map[rune]image.Point, org, dst rune, avoid image.Point) []string {
	from := coords[org]
	to := coords[dst]
	diff := to.Sub(from)

	xNegative := diff.X < 0
	yNegative := diff.Y < 0
	if xNegative {
		diff.X = -diff.X
	}
	if yNegative {
		diff.Y = -diff.Y
	}

	x := ""
	y := ""
	for range diff.X {
		if xNegative {
			x += "<"
		} else {
			x += ">"
		}
	}

	for range diff.Y {
		if yNegative {
			y += "^"
		} else {
			y += "v"
		}
	}

	var all []string
	if !(from.Y == avoid.Y && to.X == avoid.X) {
		all = append(all, x+y+"A")
	}
	if !(from.X == avoid.X && to.Y == avoid.Y) {
		all = append(all, y+x+"A")
	}

	return all
}

func solve(codes []string, robots int) int {
	memo := map[string]int{}

	result := 0
	for _, code := range codes {
		numPart, err := strconv.Atoi(code[:3])
		if err != nil {
			panic(err)
		}
		result += numPart * solveCode(code, robots, memo)
	}
	return result
}

func A(file string) int {
	codes := parse(file)
	return solve(codes, 2)

}

func B(file string) int {
	codes := parse(file)
	return solve(codes, 25)
}
