package main

import (
	"fmt"
	"image"
	"os"
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

func fromTo(coords map[rune]image.Point, origin rune, target rune) []rune {
	diff := coords[target].Sub(coords[origin])
	xNegative := diff.X < 0
	yNegative := diff.Y < 0
	if xNegative {
		diff.X += -(diff.X * 2)
	}
	if yNegative {
		diff.Y += -(diff.Y * 2)
	}
	result := []rune{}
	for range diff.Y {
		if yNegative {
			result = append(result, '^')
		} else {
			result = append(result, 'v')
		}
	}

	for range diff.X {
		if xNegative {
			result = append(result, '<')
		} else {
			result = append(result, '>')
		}
	}

	result = append(result, 'A')
	return result
}

func compute(coords map[rune]image.Point) func([]rune) []rune {
	return func(code []rune) (all []rune) {
		prev := 'A'
		for _, curr := range code {
			all = append(all, fromTo(coords, prev, curr)...)
			prev = curr
		}
		return all
	}
}

var directional = compute(directionalCoords)
var numeric = compute(numericCoords)

func A(file string) int {
	codes := parse(file)
	result := 0
	for _, code := range codes {
		numPart, err := strconv.Atoi(code[:3])
		if err != nil {
			panic(err)
		}
		instructions := directional(directional(numeric([]rune(code))))
		fmt.Println(string(instructions))
		fmt.Println(numPart, len(instructions))
		result += numPart * len(instructions)
	}
	return result
}

func B(file string) int {
	return 0
}
