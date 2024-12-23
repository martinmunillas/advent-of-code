package main

import (
	"fmt"
	"image"
	"os"
	"strings"
)

type Image [][]rune

func (i Image) In(p image.Point) rune {
	return i[p.Y][p.X]
}

func (i Image) Set(p image.Point, r rune) {
	i[p.Y][p.X] = r
}

func (m Image) Print() {
	for _, l := range m {
		fmt.Println(string(l))
	}
}

func parse(file string) (Image, []rune, image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	parts := strings.Split(string(f), "\n\n")
	img := Image{}
	var start image.Point
	for i, line := range strings.Split(parts[0], "\n") {
		img = append(img, []rune(line))
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

	return img, instructions, start
}

var direction = map[rune]image.Point{
	'>': image.Pt(1, 0),
	'<': image.Pt(-1, 0),
	'v': image.Pt(0, 1),
	'^': image.Pt(0, -1),
}

func exec(instruction rune, img Image, pos image.Point) image.Point {
	c := img.In(pos)
	nextPos := pos.Add(direction[instruction])

	if img.In(nextPos) == 'O' {
		exec(instruction, img, nextPos)
	}

	if img.In(nextPos) == '.' {
		img.Set(nextPos, c)
		img.Set(pos, '.')
		return nextPos
	}

	return pos
}

func count(img Image) int {
	points := 0
	for i, row := range img {
		for j, c := range row {
			if c == 'O' {
				points += i*100 + j
			}
		}
	}
	return points
}

func A(file string) int {
	img, instructions, pos := parse(file)
	for _, instruction := range instructions {
		pos = exec(instruction, img, pos)
	}

	return count(img)
}

func parse2(file string) (Image, []rune, image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	parts := strings.Split(string(f), "\n\n")
	img := Image{}
	var start image.Point
	for i, line := range strings.Split(parts[0], "\n") {
		l := []rune{}
		for j, c := range line {
			switch c {
			case '#':
				l = append(l, '#', '#')
			case '.':
				l = append(l, '.', '.')
			case '@':
				start = image.Pt(j*2, i)
				l = append(l, '@', '.')
			case 'O':
				l = append(l, '[', ']')

			}
		}
		img = append(img, l)
	}
	instructions := []rune{}
	for _, c := range parts[1] {
		if c == '\n' {
			continue
		}
		instructions = append(instructions, c)
	}

	return img, instructions, start
}

var verticalDirection = map[rune]image.Point{
	'v': image.Pt(0, 1),
	'^': image.Pt(0, -1),
	'[': image.Pt(1, 0),
	']': image.Pt(-1, 0),
}

func canMoveVertically(img Image, pos image.Point, direction rune) bool {
	c := img.In(pos)
	if c == '.' {
		return true
	} else if c == '#' {
		return false
	} else if c == '[' || c == ']' {
		return canMoveVertically(img, pos.Add(verticalDirection[direction]), direction) &&
			canMoveVertically(img, pos.Add(verticalDirection[direction]).Add(verticalDirection[c]), direction)
	}

	panic("shouldnt reach")
}

func exec2(instruction rune, img Image, pos image.Point) image.Point {
	c := img.In(pos)
	nextPos := pos.Add(direction[instruction])

	if instruction == '>' || instruction == '<' {
		if img.In(nextPos) == '[' || img.In(nextPos) == ']' {
			exec2(instruction, img, nextPos)
		}

		if img.In(nextPos) == '.' {
			img.Set(nextPos, c)
			img.Set(pos, '.')
			return nextPos
		}
	} else {
		if img.In(nextPos) == '[' || img.In(nextPos) == ']' {
			if !canMoveVertically(img, nextPos, instruction) {
				return pos
			}
			exec2(instruction, img, nextPos)
		}
		if c == '@' {
			if img.In(nextPos) == '.' {
				img.Set(nextPos, c)
				img.Set(pos, '.')
				return nextPos
			}
		} else if c == '[' {
			closingBox := image.Pt(1, 0)
			if img.In(nextPos.Add(closingBox)) == '[' || img.In(nextPos.Add(closingBox)) == ']' {
				exec2(instruction, img, nextPos.Add(closingBox))
			}
			if img.In(nextPos) == '.' && img.In(nextPos.Add(closingBox)) == '.' {
				img.Set(nextPos, c)
				img.Set(nextPos.Add(closingBox), img.In(pos.Add(closingBox)))
				img.Set(pos, '.')
				img.Set(pos.Add(closingBox), '.')
				return nextPos
			}
		} else if c == ']' {
			openingBox := image.Pt(-1, 0)
			if img.In(nextPos.Add(openingBox)) == '[' || img.In(nextPos.Add(openingBox)) == ']' {
				exec2(instruction, img, nextPos.Add(image.Pt(-1, 0)))
			}
			if img.In(nextPos) == '.' && img.In(nextPos.Add(openingBox)) == '.' {
				img.Set(nextPos, c)
				img.Set(nextPos.Add(openingBox), img.In(pos.Add(openingBox)))
				img.Set(pos, '.')
				img.Set(pos.Add(openingBox), '.')
				return nextPos
			}
		}

	}

	return pos
}

func count2(img Image) int {
	points := 0
	for i, row := range img {
		for j, c := range row {
			if c == '[' {
				points += i*100 + j
			}
		}
	}
	return points
}

func B(file string) int {
	img, instructions, pos := parse2(file)
	for _, instruction := range instructions {
		pos = exec2(instruction, img, pos)
	}

	return count2(img)
}
