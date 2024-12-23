package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Program struct {
	A       int
	B       int
	C       int
	Program []int
}

func (p *Program) Execute() (output []int) {
	index := 0

	for index+1 < len(p.Program) {
		fmt.Println(index)
		opcode := p.Program[index]
		operand := p.Program[index+1]
		if operand == 4 {
			operand = p.A
		} else if operand == 5 {
			operand = p.B
		} else if operand == 6 {
			operand = p.C
		}

		switch opcode {
		case 0:
			p.A = int(float64(p.A) / math.Pow(float64(operand), 2))
		case 1:
			p.B = p.B ^ operand
		case 2:
			p.B = operand % 8
		case 3:
			if p.A == 0 {
				continue
			}
			index = operand
			continue
		case 4:
			p.B = p.B ^ p.C
		case 5:
			output = append(output, operand%8)
		case 6:
			p.B = int(float64(p.A) / math.Pow(float64(operand), 2))
		case 7:
			p.C = int(float64(p.A) / math.Pow(float64(operand), 2))
		}
		index += 2
	}
	return output
}

func parseProgram(file string) Program {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	program := Program{}
	parts := strings.Split(string(f), "\n\n")
	for i, line := range strings.Split(parts[0], "\n") {
		nums := "-0123456789"
		curr := ""
		for _, c := range line {
			if strings.Contains(nums, string(c)) {
				curr += string(c)
				continue
			}
		}
		n, err := strconv.Atoi(curr)
		if err != nil {
			panic(err)
		}
		if i == 0 {
			program.A = n

		} else if i == 1 {
			program.B = n

		} else if i == 2 {
			program.C = n
		}
	}
	for i, c := range strings.Split(parts[1], " ")[1] {
		if i%2 == 1 {
			continue
		}
		n, err := strconv.Atoi(string(c))
		if err != nil {
			panic(err)
		}
		program.Program = append(program.Program, n)
	}

	return program
}

func A(file string) string {
	program := parseProgram(file)
	fmt.Print(program.Execute())
	return ""
}

func B(file string) int {
	return 0
}
