package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func SolvePartOne(input string) int {
	var left, right []int
	for _, line := range strings.Split(input, "\n") {
		values := strings.Split(line, "   ")
		l, _ := strconv.Atoi(values[0])
		left = append(left, l)
		r, _ := strconv.Atoi(values[1])
		right = append(right, r)
	}

	slices.Sort(left)
	slices.Sort(right)

	var totalDistance int

	numElements := len(left)
	for i := 0; i < numElements; i++ {
		distance := left[i] - right[i]
		if distance < 0 {
			distance *= -1
		}
		totalDistance += distance
	}

	return totalDistance
}

func SolvePartTwo(input string) int {
	var left, right []int
	for _, line := range strings.Split(input, "\n") {
		values := strings.Split(line, "   ")
		l, _ := strconv.Atoi(values[0])
		left = append(left, l)
		r, _ := strconv.Atoi(values[1])
		right = append(right, r)
	}

	occurrenceMap := make(map[int]int)

	for _, l := range left {
		if _, exists := occurrenceMap[l]; exists {
			continue
		}
		var occurrences int
		for _, r := range right {
			if l == r {
				occurrences++
			}
		}
		occurrenceMap[l] = occurrences
	}

	var similarity int

	for _, id := range left {
		occurrences := occurrenceMap[id]
		similarity += id * occurrences
	}

	return similarity
}

func main() {
	f, err := os.ReadFile("../input.txt")
	if err != nil {
		log.Fatal(err)
	}

	a1 := SolvePartOne(string(f))
	fmt.Println(a1)

	a2 := SolvePartTwo(string(f))
	fmt.Println(a2)
}
