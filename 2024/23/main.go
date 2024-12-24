package main

import (
	"os"
	"slices"
	"strings"
)

func parsePairs(file string) [][]string {
	f, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	pairs := [][]string{}
	for _, l := range strings.Split(string(f), "\n") {
		pairs = append(pairs, strings.Split(l, "-"))
	}

	return pairs
}

func makeArray(s []string) [3]string {
	slices.Sort(s)
	return [3]string{s[0], s[1], s[2]}
}

func A(file string) int {
	pairs := parsePairs(file)
	connections := map[string][]string{}
	for _, pair := range pairs {
		connections[pair[0]] = append(connections[pair[0]], pair[1])
		connections[pair[1]] = append(connections[pair[1]], pair[0])
	}

	allThreeNodeNetworks := map[[3]string]interface{}{}
	for a, conns := range connections {
		for _, b := range conns {
			for _, c := range connections[b] {
				if a != c && slices.Contains(connections[c], a) {
					three := []string{a, b, c}
					allThreeNodeNetworks[makeArray(three)] = true
				}
			}
		}
	}

	result := 0

	for network := range allThreeNodeNetworks {
		if network[0][0] == 't' || network[1][0] == 't' || network[2][0] == 't' {
			result++
		}
	}
	return result
}

func makeKey(network []string) string {
	slices.Sort(network)
	return strings.Join(network, ",")
}

func explore(node string, connections map[string][]string, looking []string, all map[string]interface{}) {
	key := makeKey(looking)
	if all[key] != nil {
		return
	}
	all[key] = true
	for _, neighbor := range connections[node] {
		if slices.Contains(looking, neighbor) {
			continue
		}
		inAll := true
		for _, n := range looking {
			if !slices.Contains(connections[n], neighbor) {
				inAll = false
				break
			}
		}
		if inAll {
			explore(neighbor, connections, append(looking, neighbor), all)
		}
	}
}

func B(file string) string {
	pairs := parsePairs(file)
	connections := map[string][]string{}
	for _, pair := range pairs {
		connections[pair[0]] = append(connections[pair[0]], pair[1])
		connections[pair[1]] = append(connections[pair[1]], pair[0])
	}

	all := map[string]interface{}{}
	for n := range connections {
		explore(n, connections, []string{n}, all)
	}

	largest := ""
	for k := range all {
		if len(k) > len(largest) {
			largest = k
		}
	}

	return largest
}
