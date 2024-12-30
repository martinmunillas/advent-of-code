package main

import (
	"os"
	"strconv"
	"strings"
)

type wire struct {
	a        string
	b        string
	operator string
	result   string

	executed bool
}

func (w wire) execute(gates map[string]int) {
	if w.executed {
		panic("already executed")
	}
	result := 0
	switch w.operator {
	case "XOR":
		result = gates[w.a] ^ gates[w.b]
	case "OR":
		result = gates[w.a] | gates[w.b]
	case "AND":
		result = gates[w.a] & gates[w.b]
	default:
		panic("invalid operator")
	}
	gates[w.result] = result
}

func (w wire) canExecute(gates map[string]int) bool {
	_, hasResult := gates[w.result]
	_, hasResultA := gates[w.a]
	_, hasResultB := gates[w.b]

	return !hasResult && !w.executed && hasResultA && hasResultB
}

func parse(file string) (map[string]int, []wire) {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	gates := map[string]int{}
	parts := strings.Split(string(f), "\n\n")
	for _, l := range strings.Split(parts[0], "\n") {
		chunks := strings.Split(l, ": ")
		n, err := strconv.Atoi(chunks[1])
		if err != nil {
			panic(err)
		}
		gates[chunks[0]] = n
	}
	wires := []wire{}
	for _, l := range strings.Split(parts[1], "\n") {
		chunks := strings.Split(l, " ")
		wires = append(wires, wire{
			a:        chunks[0],
			b:        chunks[2],
			operator: chunks[1],
			result:   chunks[4],
		})
	}

	return gates, wires
}

func A(file string) int {
	gates, wires := parse(file)
	for {
		altered := false
		for _, wire := range wires {
			if wire.canExecute(gates) {
				altered = true
				wire.execute(gates)
			}
		}
		if !altered {
			break
		}
	}
	result := 0
	for k, v := range gates {
		if k[0] != 'z' {
			continue
		}
		n, err := strconv.Atoi(k[1:])
		if err != nil {
			panic(err)
		}
		result += v << n
	}
	return result
}

func B(file string) int {
	return 0
}
