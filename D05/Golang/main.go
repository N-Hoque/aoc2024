package main

import (
	"errors"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Node int
type Edge [2]Node
type Graph struct{ edges []Edge }

type EdgeType int

const (
	In EdgeType = iota
	Out
	None
)

func (g *Graph) AddEdge(edge Edge) {
	g.edges = append(g.edges, edge)
}

func (g *Graph) FindEdgeType(edge Edge) EdgeType {
	for _, e := range g.edges {
		first := e[0]
		second := e[1]
		if edge[0] == first && edge[1] == second {
			return Out
		}
		if edge[0] == second && edge[1] == first {
			return In
		}
	}
	return None
}

func parsePair(input string) (Edge, error) {
	pairs := strings.Split(input, "|")
	if len(pairs) != 2 {
		return Edge{}, errors.New("not a valid pair")
	}
	first, _ := strconv.Atoi(pairs[0])
	second, _ := strconv.Atoi(pairs[1])

	return Edge{Node(first), Node(second)}, nil
}

func parseLine(input string) []Node {
	values := strings.Split(input, ",")
	var vals []Node
	for _, v := range values {
		x, _ := strconv.Atoi(v)
		vals = append(vals, Node(x))
	}
	return vals
}

func parse(input string) (Graph, [][]Node) {
	graph := Graph{}
	var table [][]Node

	for _, line := range strings.Split(strings.Trim(input, "\n"), "\n") {
		edge, err := parsePair(line)
		if err == nil {
			graph.AddEdge(edge)
		} else if len(line) > 0 {
			table = append(table, parseLine(line))
		}

	}

	return graph, table
}

func sortUpdate(update []Node, g *Graph) {
	slices.SortFunc(update, func(n1 Node, n2 Node) int {
		switch g.FindEdgeType(Edge{n1, n2}) {
		case Out:
			return -1
		case None:
			return 0
		default:
			return 1
		}
	})
}

func isOrdered(update []Node, g *Graph) bool {
	for idx := 0; idx < len(update)-1; idx++ {
		first := update[idx]
		second := update[idx+1]
		if g.FindEdgeType(Edge{first, second}) == In {
			return false
		}
	}
	return true
}

func SolvePartOne(graph *Graph, table [][]Node) int {
	var total int
	for _, update := range table {
		if isOrdered(update, graph) {
			total += int(update[len(update)/2])
		}
	}
	return total
}

func SolvePartTwo(graph *Graph, table [][]Node) int {
	var total int
	for _, update := range table {
		if !isOrdered(update, graph) {
			sortUpdate(update, graph)
			total += int(update[len(update)/2])
		}
	}
	return total
}

func main() {
	data, _ := os.ReadFile("../input.txt")

	graph, table := parse(string(data))

	a1 := SolvePartOne(&graph, table)

	fmt.Println(a1)

	a2 := SolvePartTwo(&graph, table)

	fmt.Println(a2)
}
