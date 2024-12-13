package main

import (
	"image"
	"os"
	"strconv"
	"strings"
)

type Game struct {
	Prize image.Point
	A     image.Point
	B     image.Point
}

func getGames(file string) []Game {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var games []Game
	for _, game := range strings.Split(string(f), "\n\n") {
		nums := "0123456789"
		all := make([]int, 0, 6)
		curr := ""
		game += " "
		for _, c := range game {
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
		games = append(games, Game{
			A:     image.Pt(all[0], all[1]),
			B:     image.Pt(all[2], all[3]),
			Prize: image.Pt(all[4], all[5]),
		})

	}
	return games
}

func isWhole(n float64) bool {
	return n == float64(int64(n))
}

func A(file string) int {
	result := 0
	games := getGames(file)

	for _, game := range games {
		aTimes := 0

		for aTimes <= 100 {
			xTimes := float64(game.Prize.X-(aTimes*game.A.X)) / float64(game.B.X)
			yTimes := float64(game.Prize.Y-(aTimes*game.A.Y)) / float64(game.B.Y)
			if xTimes == yTimes && isWhole(xTimes) && isWhole(yTimes) {
				result += int(xTimes) + (3 * aTimes)
				break
			}
			aTimes++
		}
	}
	return result
}

func B(file string) int {
	result := 0
	games := getGames(file)

	for _, game := range games {
		aTimes := 0
		game.Prize.X += 10000000000000
		game.Prize.Y += 10000000000000

		for {
			xTimes := float64(game.Prize.X-(aTimes*game.A.X)) / float64(game.B.X)
			yTimes := float64(game.Prize.Y-(aTimes*game.A.Y)) / float64(game.B.Y)
			if xTimes == yTimes && isWhole(xTimes) && isWhole(yTimes) {
				result += int(xTimes) + (3 * aTimes)
				break
			}
			aTimes++
		}
	}

	return result
}
