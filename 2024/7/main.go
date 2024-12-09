package main

import (
	"os"
	"strconv"
)

type Operation struct {
	Result int
	Values []int
}

func parseOperations(file string) []Operation {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	operations := []Operation{}
	currResult := 0
	currN := ""
	currValues := []int{}
	for _, c := range f {
		if c == '\n' {
			n, err := strconv.Atoi(currN)
			if err != nil {
				panic(err)
			}
			currValues = append(currValues, n)
			operations = append(operations, Operation{
				Values: currValues,
				Result: currResult,
			})
			currN = ""
			currValues = nil
		} else if c == ':' {
			currResult, err = strconv.Atoi(currN)
			if err != nil {
				panic(err)
			}
			currN = ""
		} else if c == ' ' && currN != "" {
			n, err := strconv.Atoi(currN)
			if err != nil {
				panic(err)
			}
			currValues = append(currValues, n)
			currN = ""
		} else if c != ' ' {
			currN += string(c)
		}
	}
	n, err := strconv.Atoi(currN)
	if err != nil {
		panic(err)
	}
	currValues = append(currValues, n)
	operations = append(operations, Operation{
		Values: currValues,
		Result: currResult,
	})
	currN = ""
	currValues = nil

	return operations

}

func (o Operation) CouldBeValid() bool {
	if o.add(0, o.Values) {
		return true
	}
	if o.multiply(1, o.Values) {
		return true
	}
	return false
}

func (o Operation) add(n int, next []int) bool {
	if len(next) == 0 {
		return n == o.Result
	}

	if o.add(n+next[0], next[1:]) {
		return true
	}
	if o.multiply(n*next[0], next[1:]) {
		return true
	}

	return false
}

func (o Operation) multiply(n int, next []int) bool {
	if len(next) == 0 {
		return n == o.Result
	}

	if o.add(n+next[0], next[1:]) {
		return true
	}
	if o.multiply(n*next[0], next[1:]) {
		return true
	}

	return false
}

func A(file string) int {
	result := 0
	operations := parseOperations(file)
	for _, operation := range operations {
		if operation.CouldBeValid() {
			result += operation.Result
		}
	}
	return result
}

func B(file string) int {
	result := 0
	return result
}
