package main

import (
	"testing"
)

const small = `..X...
.SAMX.
.A..A.
XMAS.S
.X....`

const sample = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`

func TestSolveSmallOne(t *testing.T) {
	expected := 4

	actual := SolvePartOne(small)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveSmallTwo(t *testing.T) {
	expected := 0

	actual := SolvePartTwo(small)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveOne(t *testing.T) {
	expected := 18

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveTwo(t *testing.T) {
	expected := 9

	actual := SolvePartTwo(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}
