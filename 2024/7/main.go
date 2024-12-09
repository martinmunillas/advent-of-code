package main

import (
	"fmt"
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

func (o Operation) CouldBeValid(withUnionOperator bool) bool {
	if o.operate("", o.Values, withUnionOperator) {
		return true
	}
	return false
}

func (o Operation) operate(str string, next []int, withUnionOperator bool) bool {
	n := 0
	if str != "" {
		num, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		n = num
	}
	if len(next) == 0 {
		return n == o.Result
	}

	if o.operate(fmt.Sprintf("%d", n+next[0]), next[1:], withUnionOperator) {
		return true
	}
	if str == "" {
		n = 1
	}
	if o.operate(fmt.Sprintf("%d", n*next[0]), next[1:], withUnionOperator) {
		return true
	}

	if withUnionOperator && o.operate(fmt.Sprintf("%s%d", str, next[0]), next[1:], withUnionOperator) {
		return true
	}

	return false
}

func A(file string) int {
	result := 0
	operations := parseOperations(file)
	for _, operation := range operations {
		if operation.CouldBeValid(false) {
			result += operation.Result
		}
	}
	return result
}

func B(file string) int {
	result := 0
	operations := parseOperations(file)
	for _, operation := range operations {
		if operation.CouldBeValid(true) {
			result += operation.Result
		}
	}
	return result
}
