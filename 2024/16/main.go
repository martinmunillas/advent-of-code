package main

import (
	"container/heap"
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

func (m Image) Print(visited map[image.Point]interface{}) {
	for i, l := range m {
		for j, c := range l {
			if visited[image.Pt(j, i)] != nil {
				fmt.Print("O")
			} else {
				fmt.Print(string(c))
			}
		}
		fmt.Println()
	}
	fmt.Println()
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

	return img, start
}

var next = map[rune]image.Point{
	'<': image.Pt(-1, 0),
	'>': image.Pt(1, 0),
	'v': image.Pt(0, 1),
	'^': image.Pt(0, -1),
}

var right = map[rune]rune{
	'<': '^',
	'^': '>',
	'>': 'v',
	'v': '<',
}
var left = map[rune]rune{
	'<': 'v',
	'v': '>',
	'>': '^',
	'^': '<',
}

type Step struct {
	image.Point
	Dir rune
}

type Node struct {
	Step
	Cost int
}

func A(file string) int {
	img, start := parse(file)

	firstStep := Step{Point: start, Dir: '>'}

	pq := MinHeap{Node{Step: firstStep}}
	seen := map[Step]interface{}{}

	for pq.Len() > 0 {
		node := heap.Pop(&pq).(Node)
		if seen[node.Step] != nil || img.In(node.Step.Point) == '#' {
			continue
		}
		seen[node.Step] = true
		if img.In(node.Step.Point) == 'E' {
			return node.Cost
		}
		heap.Push(&pq, Node{Cost: node.Cost + 1, Step: Step{Point: node.Step.Point.Add(next[node.Step.Dir]), Dir: node.Step.Dir}})
		heap.Push(&pq, Node{Cost: node.Cost + 1000, Step: Step{Point: node.Step.Point, Dir: right[node.Step.Dir]}})
		heap.Push(&pq, Node{Cost: node.Cost + 1000, Step: Step{Point: node.Step.Point, Dir: left[node.Step.Dir]}})
	}

	return -1
}

func B(file string) int {
	img, start := parse(file)

	firstStep := Step{Point: start, Dir: '>'}

	pq := MinHeap{Node{Step: firstStep}}
	lowestCost := map[Step]int{}
	bestCost := math.Inf(0)
	ends := []Step{}
	previous := map[Step][]Step{}

	for pq.Len() > 0 {
		node := heap.Pop(&pq).(Node)
		if img.In(node.Step.Point) == '#' {
			continue
		}

		if img.In(node.Step.Point) == 'E' {
			if float64(node.Cost) < bestCost {
				bestCost = float64(node.Cost)
				ends = append(ends, node.Step)
				continue
			} else {
				break
			}
		}
		for _, nextNode := range []Node{
			{Cost: node.Cost + 1, Step: Step{Point: node.Step.Point.Add(next[node.Step.Dir]), Dir: node.Step.Dir}},
			{Cost: node.Cost + 1000, Step: Step{Point: node.Step.Point, Dir: right[node.Step.Dir]}},
			{Cost: node.Cost + 1000, Step: Step{Point: node.Step.Point, Dir: left[node.Step.Dir]}},
		} {
			if v, ok := lowestCost[nextNode.Step]; ok {
				if nextNode.Cost > v {
					continue
				}
			} else {
				lowestCost[nextNode.Step] = nextNode.Cost
			}
			heap.Push(&pq, nextNode)
			previous[nextNode.Step] = append(previous[nextNode.Step], node.Step)
		}
	}

	completePathSeen := map[Step]interface{}{}
	visited := map[image.Point]interface{}{}
	for len(ends) > 0 {
		var curr Step
		curr, ends = ends[0], ends[1:]
		if completePathSeen[curr] != nil {
			continue
		}
		completePathSeen[curr] = true
		visited[curr.Point] = true
		ends = append(ends, previous[curr]...)

	}

	return len(visited)
}
