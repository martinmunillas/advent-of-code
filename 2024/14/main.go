package main

import (
	"fmt"
	"image"
	"os"
	"strconv"
	"strings"
)

const (
	NoneQuadrant = iota
	FirstQuadrant
	SecondQuadrant
	ThirdQuadrant
	FourthQuadrant
)

type Robot struct {
	Position image.Point
	Velocity image.Point
}

func (r Robot) After(seconds int, maxX int, maxY int) Robot {
	r.Position = r.Position.Add(r.Velocity.Mul(seconds))
	r.Position.X = r.Position.X % maxX
	r.Position.Y = r.Position.Y % maxY
	if r.Position.X < 0 {
		r.Position.X = maxX + r.Position.X
	}
	if r.Position.Y < 0 {
		r.Position.Y = maxY + r.Position.Y
	}
	return r
}

func (r Robot) Quadrant(maxX int, maxY int) int {
	middleX := (maxX / 2)
	middleY := (maxY / 2)
	if r.Position.X == middleX || r.Position.Y == middleY {
		return NoneQuadrant
	}

	if r.Position.X < middleX {
		if r.Position.Y < middleY {
			return FirstQuadrant
		} else {
			return ThirdQuadrant
		}
	} else {
		if r.Position.Y < middleY {
			return SecondQuadrant
		} else {
			return FourthQuadrant
		}
	}
}

func getRobots(file string) []Robot {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var robots []Robot
	for _, robotLine := range strings.Split(string(f), "\n") {
		nums := "-0123456789"
		all := make([]int, 0, 4)
		curr := ""
		robotLine += " "
		for _, c := range robotLine {
			if strings.Contains(nums, string(c)) {
				curr += string(c)
				continue
			} else if len(curr) > 0 {
				n, err := strconv.Atoi(curr)
				if err != nil {
					panic(err)
				}
				all = append(all, n)
			}
			curr = ""
		}
		robots = append(robots, Robot{
			Position: image.Pt(all[0], all[1]),
			Velocity: image.Pt(all[2], all[3]),
		})

	}
	return robots
}

func printRepresentation(maxX, maxY int, robots []Robot) string {
	matrix := make([][]bool, maxY)
	str := ""
	for i := range matrix {
		matrix[i] = make([]bool, maxX)
	}
	for _, robot := range robots {
		matrix[robot.Position.Y][robot.Position.X] = true
	}
	for _, row := range matrix {
		for _, n := range row {
			if !n {
				str += "."
			} else {
				str += "#"
			}
		}
		str += "\n"
	}

	return str
}

func A(file string, maxX int, maxY int) int {
	robots := getRobots(file)
	first := 0
	second := 0
	third := 0
	fourth := 0
	for i, robot := range robots {
		robot = robot.After(100, maxX, maxY)
		robots[i] = robot
		switch robot.Quadrant(maxX, maxY) {
		case FirstQuadrant:
			first++
		case SecondQuadrant:
			second++
		case ThirdQuadrant:
			third++
		case FourthQuadrant:
			fourth++
		}
	}

	return first * second * third * fourth
}

func B(file string, maxX, maxY int) {
	robots := getRobots(file)
	f, err := os.OpenFile("findTheTree.txt", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0666)
	if err != nil {
		panic(err)
	}
	for i := range 10000 {
		if i == 0 {
			continue
		}
		for i, robot := range robots {
			robots[i] = robot.After(1, maxX, maxY)
		}
		representation := printRepresentation(maxX, maxY, robots)
		if strings.Contains(representation, "#######") {
			f.WriteString(fmt.Sprintf("\nSecond %d\n", i))
			f.WriteString(representation)
		}
	}

	f.Close()
}
