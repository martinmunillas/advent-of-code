package main

import (
	"fmt"
	"image"
	"math"
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
func (i Image) Copy() Image {
	newImage := make(Image, len(i))
	for j := range newImage {
		newLine := make([]rune, len(i[j]))
		copy(newLine, i[j])
		newImage[j] = newLine
	}

	return newImage
}

func (i Image) InBounds(p image.Point) bool {
	return p.X >= 0 && p.Y >= 0 && p.X < len(i[0]) && p.Y < len(i)
}

func (m Image) Print() {
	for _, l := range m {
		fmt.Println(string(l))
	}
}

type Distances [][]int

func (i Distances) In(p image.Point) int {
	return i[p.Y][p.X]
}

func (i Distances) Set(p image.Point, r int) {
	i[p.Y][p.X] = r
}

func (m Distances) Print() {
	for _, l := range m {
		for _, n := range l {
			fmt.Printf("|%d|", n)
		}
		fmt.Println()
	}
}

func parse(file string) (Image, image.Point) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	img := Image{}
	var start image.Point
	for i, line := range strings.Split(string(f), "\n") {
		img = append(img, []rune(line))
		xStart := strings.Index(line, "S")
		if xStart != -1 {
			start = image.Pt(xStart, i)
		}
	}

	img.Set(start, '.')

	return img, start
}

func getDistances(img Image, coord image.Point) Distances {
	distances := make(Distances, len(img))
	for i := range distances {
		distances[i] = make([]int, len(img[i]))
	}
	for i, line := range img {
		for j, c := range line {
			if c == '#' {
				distances[i][j] = -1
			}
		}
	}
	distance := 0
traveling:
	for {
		distances.Set(coord, distance)
		img.Set(coord, '#')
		for _, dir := range []image.Point{
			{1, 0},
			{-1, 0},
			{0, -1},
			{0, 1},
		} {
			newCoord := coord.Add(dir)
			if img.In(newCoord) == '.' || img.In(newCoord) == 'E' {
				coord = newCoord
				distance++
				continue traveling
			}
		}
		break
	}

	return distances

}

func A(file string) int {
	img, start := parse(file)

	distances := getDistances(img.Copy(), start)

	result := 0

	for i, line := range img {
		for j, c := range line {
			if c == '#' {
				continue
			}

			pos := image.Pt(j, i)
			for _, possibleCheat := range []image.Point{
				{2, 0},
				{1, 1},
				{0, 2},
				{-1, 1},
			} {
				cheatPos := pos.Add(possibleCheat)
				if img.InBounds(cheatPos) && img.In(cheatPos) != '#' && math.Abs(float64(distances.In(cheatPos)-distances.In(pos))) > 101 {
					result++
				}
			}
		}
	}

	return result
}

func B(file string) int {
	img, start := parse(file)

	distances := getDistances(img.Copy(), start)

	result := 0

	for i, line := range img {
		for j, c := range line {
			if c == '#' {
				continue
			}

			pos := image.Pt(j, i)
			for r := range 21 {
				if r < 2 {
					continue
				}
				for di := range r + 1 {
					dj := r - di

					for possibleCheat := range map[image.Point]interface{}{
						image.Pt(dj, di):   true,
						image.Pt(dj, -di):  true,
						image.Pt(-dj, -di): true,
						image.Pt(-dj, di):  true,
					} {
						cheatPos := pos.Add(possibleCheat)
						if img.InBounds(cheatPos) && img.In(cheatPos) != '#' && distances.In(pos)-distances.In(cheatPos) >= 100+r {
							result++
						}
					}
				}
			}
		}
	}

	return result
}
