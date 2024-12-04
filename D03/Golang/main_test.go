package main

import (
	"fmt"
	"testing"
)

func TestSolveSimple(t *testing.T) {
	const sample = "mul(2,4)"

	expected := 8

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveSimple2(t *testing.T) {
	const sample = "mul(20,4)"

	expected := 80

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveSimple3(t *testing.T) {
	const sample = "mul(2,40)"

	expected := 80

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveSimple4(t *testing.T) {
	const sample = "mul(20,40)"

	expected := 800

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveSimple5(t *testing.T) {
	const sample = "mul(200,40)"

	expected := 8000

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveSimple6(t *testing.T) {
	const sample = "mul(200,400)"

	expected := 80000

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveOne(t *testing.T) {
	const sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

	expected := 161

	actual := SolvePartOne(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}

func TestSolveTwo(t *testing.T) {
	const sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

	expected := 48

	tokens := lex(sample)
	fmt.Println(tokens)

	actual := SolvePartTwo(sample)

	if expected != actual {
		t.Fatalf("%d != %d", expected, actual)
	}
}
