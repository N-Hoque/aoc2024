package main

import "testing"

const sample = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`

func TestSolveOne(t *testing.T) {
	expected := 143

	graph, table := parse(sample)

	actual := SolvePartOne(&graph, table)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveTwo(t *testing.T) {
	expected := 123

	graph, table := parse(sample)

	actual := SolvePartTwo(&graph, table)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}
