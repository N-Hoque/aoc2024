package main

import "testing"

const sample = `3   4
4   3
2   5
1   3
3   9
3   3`

func TestSolvePartOne(t *testing.T) {
	expected := 11
	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d does not equal %d", expected, actual)
	}
}

func TestSolvePartTwo(t *testing.T) {
	expected := 31
	actual := SolvePartTwo(sample)

	if expected != actual {
		t.Fatalf("%d does not equal %d", expected, actual)
	}
}
