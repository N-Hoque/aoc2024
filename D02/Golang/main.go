package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func processRecords(input string) [][]int {
	input = strings.Trim(input, "\n")

	lines := strings.Split(input, "\n")

	records := make([][]int, len(lines))

	for idx, line := range lines {
		values := strings.Split(line, " ")
		record := make([]int, len(values))
		for idx, value := range values {
			valueParsed, _ := strconv.Atoi(value)
			record[idx] = valueParsed
		}
		records[idx] = record
	}

	return records
}

func isRecordSafe(record []int) bool {
	first := true
	var ascending bool

	numElements := len(record)

	for idx := 0; idx < numElements-1; idx++ {
		a := record[idx]
		b := record[idx+1]
		d := int(math.Abs(float64(a) - float64(b)))

		if d < 1 || d > 3 {
			return false
		}

		if first {
			first = false
			ascending = a > b
		} else if a > b && !ascending || a < b && ascending {
			return false
		}
	}
	return true
}

func SolvePartOne(input string) int {
	var safeRecords int

	for _, record := range processRecords(input) {
		if isRecordSafe(record) {
			safeRecords++
		}
	}

	return safeRecords
}

func SolvePartTwo(input string) int {
	var safeRecords int

	for _, record := range processRecords(input) {
		if isRecordSafe(record) {
			safeRecords++
		} else {
			numElements := len(record)
			for idx := 0; idx < numElements; idx++ {
				var subRecord []int
				subRecord = append(subRecord, record[:idx]...)
				subRecord = append(subRecord, record[idx+1:]...)
				if isRecordSafe(subRecord) {
					safeRecords++
					break
				}
			}
		}
	}

	return safeRecords
}

func main() {
	d, _ := os.ReadFile("../input.txt")

	p1 := SolvePartOne(string(d))

	fmt.Println(p1)

	p2 := SolvePartTwo(string(d))

	fmt.Println(p2)
}
