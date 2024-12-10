package main

import (
	"os"
	"slices"
)

func A(file string) int {
	result := 0
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var s []rune
	var emptyPos []int
	for i, c := range f {
		times := int(c - '0')
		for range times {
			if i%2 == 0 {
				s = append(s, rune(i/2+'0'))
			} else {
				s = append(s, '.')
				emptyPos = append(emptyPos, len(s)-1)
			}
		}
	}
	for i := len(s) - 1; i >= 0; i-- {
		c := s[i]
		if c != '.' && len(emptyPos) > 0 {
			curr := emptyPos[0]
			emptyPos = emptyPos[1:]
			if i <= curr {
				break
			}
			s[curr] = c
			s[i] = '.'
		}
	}
	for i, c := range s {
		if c == '.' {
			break
		}
		result += i * int(c-'0')
	}

	return result
}

type Block struct {
	Size int
	ID   int
}

func (b Block) isSpace() bool {
	return b.ID == -1
}

func itemize(blocks []Block) []int {
	var s []int
	for _, b := range blocks {
		for range b.Size {
			if b.ID < 0 {
				s = append(s, 0)
			} else {
				s = append(s, b.ID)
			}
		}
	}
	return s
}

func B(file string) int {
	result := 0
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	var blocks []Block
	for i, c := range f {
		size := int(c - '0')
		if size == 0 {
			continue
		}
		if i%2 == 0 {
			blocks = append(blocks, Block{Size: size, ID: i / 2})
		} else {
			blocks = append(blocks, Block{Size: size, ID: -1})
		}
	}

blocks:
	for i := len(blocks) - 1; i >= 0; i-- {
		block := blocks[i]
		if block.isSpace() {
			continue
		}
		for j, space := range blocks {
			if !space.isSpace() || j >= i {
				continue
			}
			if space.Size > block.Size {
				blocks[j].Size -= block.Size
				blocks[i] = Block{ID: -1, Size: block.Size}
				blocks = slices.Insert(blocks, j, block)
				continue blocks
			} else if space.Size == block.Size {
				blocks[i] = space
				blocks[j] = block
				continue blocks
			}
		}

	}
	items := itemize(blocks)
	for i, item := range items {
		result += i * item
	}

	return result
}
