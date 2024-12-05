package main

import (
	"fmt"
	"os"
	"strings"
)

const XmasSpan = 3

var XmasExtents = []Point{
	{0, XmasSpan},
	{0, -XmasSpan},
	{-XmasSpan, 0},
	{XmasSpan, 0},
	{XmasSpan, XmasSpan},
	{XmasSpan, -XmasSpan},
	{-XmasSpan, -XmasSpan},
	{-XmasSpan, XmasSpan},
}

var XmasCrossExtents = []Point{
	{1, 1},
	{1, -1},
	{-1, -1},
	{-1, 1},
}

type Table struct {
	cells   [][]rune
	numRows int
	numCols int
}

type Point struct {
	x int
	y int
}

type Bound struct {
	startX int
	endX   int
	startY int
	endY   int
}

func (c Bound) String() string {
	return fmt.Sprintf("(%d, %d) -> (%d, %d)", c.startX, c.startY, c.endX, c.endY)
}

func (t Table) String() string {
	var table string
	for _, row := range t.cells {
		var rowStr string
		for _, ch := range row {
			rowStr += string(ch)
		}
		table += rowStr + "\n"
	}
	return table
}

func processTable(input string) Table {
	input = strings.Trim(input, "\n")
	var table Table
	var rows [][]rune
	for _, line := range strings.Split(input, "\n") {
		var row []rune
		for _, ch := range line {
			row = append(row, ch)
		}
		table.numCols = len(row)
		rows = append(rows, row)
	}
	table.numRows = len(rows)
	table.cells = rows
	return table
}

func scanTable(table Table, x, y int) []Bound {
	var xmasCount []Bound

	if table.cells[x][y] != 'X' {
		return nil
	}

	for _, extent := range XmasExtents {
		maxX := x + extent.x
		maxY := y + extent.y
		if !(maxX >= 0 && maxX < table.numRows && maxY >= 0 && maxY < table.numCols) {
			continue
		}

		var xmasCheck string
		for step := 0; step < 4; step++ {
			curX := x
			if maxX-x == XmasSpan {
				curX += step
			} else if maxX-x == -XmasSpan {
				curX -= step
			}

			curY := y
			if maxY-y == XmasSpan {
				curY += step
			} else if maxY-y == -XmasSpan {
				curY -= step
			}
			xmasCheck += string(table.cells[curX][curY])
		}
		if xmasCheck == "XMAS" {
			xmasCount = append(xmasCount, Bound{
				startX: x,
				startY: y,
				endX:   maxX,
				endY:   maxY,
			})
		}
	}
	return xmasCount
}

func scanTableCross(table Table, x, y int) *Point {
	if table.cells[x][y] != 'A' {
		return nil
	}

	for _, extent := range XmasCrossExtents {
		maxX := x + extent.x
		maxY := y + extent.y
		if !(maxX >= 0 && maxX < table.numRows && maxY >= 0 && maxY < table.numCols) {
			return nil
		}
	}

	tl := table.cells[x-1][y-1]
	tr := table.cells[x-1][y+1]
	bl := table.cells[x+1][y-1]
	br := table.cells[x+1][y+1]

	m1 := string(tl) + "A" + string(br)
	m2 := string(bl) + "A" + string(tr)

	if (m1 == "MAS" || m1 == "SAM") && (m2 == "MAS" || m2 == "SAM") {
		return &Point{x: x, y: y}
	}

	return nil
}

func SolvePartOne(input string) int {
	table := processTable(input)

	var total int

	for idx, row := range table.cells {
		for jdx := range row {
			total += len(scanTable(table, idx, jdx))
		}
	}

	return total
}

func SolvePartTwo(input string) int {
	table := processTable(input)

	var total int

	for idx, row := range table.cells {
		for jdx := range row {
			extent := scanTableCross(table, idx, jdx)
			if extent != nil {
				total += 1
			}
		}
	}

	return total
}

func main() {
	data, _ := os.ReadFile("../input.txt")

	a1 := SolvePartOne(string(data))

	fmt.Println(a1)

	a2 := SolvePartTwo(string(data))

	fmt.Println(a2)
}
