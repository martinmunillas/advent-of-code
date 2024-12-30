package main

import (
	"fmt"
	"os"
	"sort"
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

func executeWires(gates map[string]int, wires []wire) int {
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

func A(file string) int {
	gates, wires := parse(file)
	return executeWires(gates, wires)
}

func makeWire(char string, num int) string {
	return fmt.Sprintf("%s%02d", char, num)
}

func verifyZ(wires map[string]wire, wire string, num int) bool {
	formula, exists := wires[wire]
	if !exists || formula.operator != "XOR" {
		return false
	}

	if num == 0 {
		return areSortedEqual([]string{formula.a, formula.b}, []string{"x00", "y00"})
	}

	return (verifyIntermediateXOR(wires, formula.a, num) && verifyCarryBit(wires, formula.b, num)) ||
		(verifyIntermediateXOR(wires, formula.b, num) && verifyCarryBit(wires, formula.a, num))
}

func verifyIntermediateXOR(wires map[string]wire, wire string, num int) bool {
	formula, exists := wires[wire]
	if !exists || formula.operator != "XOR" {
		return false
	}

	return areSortedEqual([]string{formula.a, formula.b}, []string{makeWire("x", num), makeWire("y", num)})
}

func verifyCarryBit(wires map[string]wire, wire string, num int) bool {
	formula, exists := wires[wire]
	if !exists {
		return false
	}

	if num == 1 {
		return formula.operator == "AND" &&
			areSortedEqual([]string{formula.a, formula.b}, []string{"x00", "y00"})
	}

	if formula.operator != "OR" {
		return false
	}

	return (verifyDirectCarry(wires, formula.a, num-1) && verifyRecarry(wires, formula.b, num-1)) ||
		(verifyDirectCarry(wires, formula.b, num-1) && verifyRecarry(wires, formula.a, num-1))
}

func verifyDirectCarry(wires map[string]wire, wire string, num int) bool {
	formula, exists := wires[wire]
	if !exists || formula.operator != "AND" {
		return false
	}

	return areSortedEqual([]string{formula.a, formula.b}, []string{makeWire("x", num), makeWire("y", num)})
}

func verifyRecarry(wires map[string]wire, wire string, num int) bool {
	formula, exists := wires[wire]
	if !exists || formula.operator != "AND" {
		return false
	}

	return (verifyIntermediateXOR(wires, formula.a, num) && verifyCarryBit(wires, formula.b, num)) ||
		(verifyIntermediateXOR(wires, formula.b, num) && verifyCarryBit(wires, formula.a, num))
}

func verify(wires map[string]wire, num int) bool {
	return verifyZ(wires, makeWire("z", num), num)
}

func progress(wires map[string]wire) int {
	i := 0
	for verify(wires, i) {
		i++
	}
	return i
}

func areSortedEqual(a, b []string) bool {
	sort.Strings(a)
	sort.Strings(b)
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func B(file string) string {
	_, wires := parse(file)

	var wiresMap = make(map[string]wire, len(wires))
	for _, w := range wires {
		wiresMap[w.result] = w
	}

	var swaps []string
	for i := 0; i < 4; i++ {
		baseline := progress(wiresMap)
		foundSwap := false

		for x := range wiresMap {
			for y := range wiresMap {
				if x == y {
					continue
				}
				wiresMap[x], wiresMap[y] = wiresMap[y], wiresMap[x]

				newProgress := progress(wiresMap)

				if newProgress > baseline {
					swaps = append(swaps, x, y)
					baseline = newProgress
					foundSwap = true
					break
				}

				wiresMap[x], wiresMap[y] = wiresMap[y], wiresMap[x]
			}

			if foundSwap {
				break
			}
		}

		if !foundSwap {
			break
		}
	}

	sort.Strings(swaps)
	return strings.Join(swaps, ",")
}
