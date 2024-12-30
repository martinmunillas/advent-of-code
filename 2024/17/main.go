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

func (p Program) combo(operand int) int {
	if operand == 4 {
		operand = p.A
	} else if operand == 5 {
		operand = p.B
	} else if operand == 6 {
		operand = p.C
	}

	return operand

}

func (p *Program) Execute() (output []int) {
	index := 0

	for index+1 < len(p.Program) {
		fmt.Println(index)
		opcode := p.Program[index]
		operand := p.Program[index+1]

		switch opcode {
		case 0:
			p.A = int(float64(p.A) / math.Pow(2, float64(p.combo(operand))))
		case 1:
			p.B = p.B ^ operand
		case 2:
			p.B = p.combo(operand) % 8
		case 3:
			if p.A != 0 {
				index = operand
				continue
			}
		case 4:
			p.B = p.B ^ p.C
		case 5:
			output = append(output, p.combo(operand)%8)
		case 6:
			p.B = int(float64(p.A) / math.Pow(2, float64(p.combo(operand))))
		case 7:
			p.C = int(float64(p.A) / math.Pow(2, float64(p.combo(operand))))
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
	output := program.Execute()
	res := ""
	for i, n := range output {
		if i != 0 {
			res += ","
		}
		res += fmt.Sprintf("%d", n)
	}
	return res
}

func find(program []int, target []int, ans int) int {
	if len(target) == 0 {
		return ans
	}

	for t := 0; t < 8; t++ {
		a := (ans << 3) | t
		b, c := 0, 0
		var output *int
		adv3 := false

		combo := func(operand int) int {
			switch operand {
			case 0, 1, 2, 3:
				return operand
			case 4:
				return a
			case 5:
				return b
			case 6:
				return c
			default:
				panic(fmt.Sprintf("unrecognized combo operand %d", operand))
			}
		}

		for pointer := 0; pointer < len(program)-2; pointer += 2 {
			opcode := program[pointer]
			operand := program[pointer+1]

			switch opcode {
			case 0:
				if adv3 {
					panic("program has multiple ADVs")
				}
				if operand != 3 {
					panic(fmt.Sprintf("program has ADV with operand %d other than 3", operand))
				}
				adv3 = true
			case 1:
				b ^= operand
			case 2:
				b = combo(operand) % 8
			case 3:
				panic("program has JNZ inside expected loop body")
			case 4:
				b ^= c
			case 5:
				if output != nil {
					panic("program has multiple OUT")
				}
				out := combo(operand) % 8
				output = &out
			case 6:
				b = a >> combo(operand)
			case 7:
				c = a >> combo(operand)
			}

			if output != nil && *output == target[len(target)-1] {
				sub := find(program, target[:len(target)-1], a)
				if sub != -1 {
					return sub
				}
			}
		}
	}

	return -1
}

func B(file string) int {
	program := parseProgram(file)
	return find(program.Program, program.Program, 0)
}
