package main

import (
	"fmt"
	"os"
	"strconv"
)

type TokenID int

const (
	IdentMul TokenID = iota
	IdentDo
	IdentDont
	Digit
	Sep
	LeftParens
	RightParens
	Other
)

func (id TokenID) String() string {
	switch id {
	case IdentMul:
		return "MUL"
	case IdentDo:
		return "DO"
	case IdentDont:
		return "DONT"
	case Digit:
		return "DIGIT"
	case Sep:
		return "SEP"
	case LeftParens:
		return "LEFT_PARENS"
	case RightParens:
		return "RIGHT_PARENS"
	default:
		return "OTHER"
	}
}

type Token struct {
	Symbol byte
	ID     TokenID
}

func (tok Token) String() string {
	switch tok.ID {
	case Other:
		return fmt.Sprintf("<%s, %s>", tok.ID.String(), string(tok.Symbol))
	case Digit:
		return string(tok.Symbol)
	default:
		return tok.ID.String()
	}
}

type Multiply struct {
	Left  int
	Right int
}

func lex(input string) []Token {
	var tokens []Token

	for idx := 0; idx < len(input); {
		symbol := input[idx]
		switch symbol {
		case 'd':
			if input[idx+1] == 'o' {
				if input[idx+2] == 'n' && input[idx+3] == '\'' && input[idx+4] == 't' {
					tokens = append(tokens, Token{Symbol: symbol, ID: IdentDont})
					idx += 4
				} else {
					tokens = append(tokens, Token{Symbol: symbol, ID: IdentDo})
					idx += 2
				}
				continue
			} else {
				tokens = append(tokens, Token{Symbol: symbol, ID: Other})
			}
		case 'm':
			if input[idx+1] == 'u' && input[idx+2] == 'l' {
				tokens = append(tokens, Token{Symbol: symbol, ID: IdentMul})
				idx += 3
				continue
			}
			tokens = append(tokens, Token{Symbol: symbol, ID: Other})
		case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
			tokens = append(tokens, Token{Symbol: symbol, ID: Digit})
		case '(':
			tokens = append(tokens, Token{Symbol: symbol, ID: LeftParens})
		case ')':
			tokens = append(tokens, Token{Symbol: symbol, ID: RightParens})
		case ',':
			tokens = append(tokens, Token{Symbol: symbol, ID: Sep})
		default:
			tokens = append(tokens, Token{Symbol: symbol, ID: Other})
		}
		idx += 1
	}

	return tokens
}

func parseMul(tokens []Token) (ins *Multiply, readOffset int) {
	peekOffset := readOffset + 1
	tok := tokens[readOffset]
	if tok.ID != LeftParens {
		return nil, peekOffset
	}

	var first string
	for x := 0; x < 4; x++ {
		readOffset += 1
		peekOffset += 1
		tok := tokens[readOffset]
		if tok.ID != Digit {
			break
		}
		first += string(tok.Symbol)
	}

	tok = tokens[readOffset]
	if tok.ID != Sep {
		return nil, peekOffset
	}

	var second string
	for x := 0; x < 4; x++ {
		readOffset += 1
		peekOffset += 1
		tok := tokens[readOffset]
		if tok.ID == Digit {
			second += string(tok.Symbol)
		} else {
			break
		}
	}

	tok = tokens[readOffset]
	if tok.ID != RightParens {
		return nil, peekOffset
	}

	o1, _ := strconv.Atoi(first)
	o2, _ := strconv.Atoi(second)

	ins = &Multiply{Left: o1, Right: o2}
	return ins, peekOffset + 1
}

func parse(tokens []Token, enable_conditions bool) []Multiply {
	var idx int
	var muls []Multiply
	useMultiply := true
	for idx < len(tokens) {
		switch tokens[idx].ID {
		case IdentMul:
			mul, offset := parseMul(tokens[idx+1:])
			if mul != nil && useMultiply {
				muls = append(muls, *mul)
			}
			idx += offset
		case IdentDo:
			if enable_conditions {
				useMultiply = true
			}
			idx += 1
		case IdentDont:
			if enable_conditions {
				useMultiply = false
			}
			idx += 1
		default:
			idx += 1
		}
	}
	return muls
}

func SolvePartOne(input string) int {
	tokens := lex(input)
	ins := parse(tokens, false)

	var total int
	for _, in := range ins {
		total += in.Left * in.Right
	}

	return total
}

func SolvePartTwo(input string) int {
	tokens := lex(input)
	ins := parse(tokens, true)

	var total int

	for _, in := range ins {
		total += in.Left * in.Right
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
