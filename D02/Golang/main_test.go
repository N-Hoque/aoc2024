package main

import "testing"

const sample = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

func TestSampleOne(t *testing.T) {
	expected := 2
	actual := SolvePartOne(sample)
	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSampleTwo(t *testing.T) {
	expected := 4
	actual := SolvePartTwo(sample)
	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}
