package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Position struct {
	x int
	y int
}

func main() {
	fmt.Println("input_01.txt --", solve("input_01.txt"))
	fmt.Println("input_02.txt --", solve("input_02.txt"))
}

func solve(filename string) int64 {
	bytes, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	contents := string(bytes)

	lines := strings.Split(contents, "\n")
	sum := int64(0)
	for i, line := range lines {
		for j, char := range line {
			if char == '.' {
				continue
			}
			// Skip if char is a number
			if char >= '0' && char <= '9' {
				continue
			}

			seenPositions := make(map[Position]bool)

			// Find adjacent seats
			for _, pos := range adjacent(i, j, len(lines), len(line)) {
				// Skip if we've already seen this position
				if seenPositions[pos] {
					continue
				}
				seenPositions[pos] = true

				adjacentChar := lines[pos.x][pos.y]

				// Skip anything that isn't a number
				if adjacentChar < '0' || adjacentChar > '9' {
					continue
				}

				start := pos.y
				end := pos.y
				// Grow left till we encounter a non-number
				for i := pos.y; i >= 0; i-- {
					if lines[pos.x][i] < '0' || lines[pos.x][i] > '9' {
						break
					}
					start = i
					seenPositions[Position{pos.x, i}] = true
				}
				// Grow right till we encounter a non-number
				for i := pos.y; i < len(line); i++ {
					if lines[pos.x][i] < '0' || lines[pos.x][i] > '9' {
						break
					}
					end = i
					seenPositions[Position{pos.x, i}] = true
				}

				num, err := strconv.ParseInt(lines[pos.x][start:end+1], 10, 64)
				if err != nil {
					panic(err)
				}

				sum += num
			}
		}
	}
	return sum
}

func adjacent(x, y, lenX, lenY int) []Position {
	positions := []Position{
		{x - 1, y - 1},
		{x - 1, y},
		{x - 1, y + 1},
		{x, y - 1},
		{x, y + 1},
		{x + 1, y - 1},
		{x + 1, y},
		{x + 1, y + 1},
	}
	res := make([]Position, 0, len(positions))
	for _, pos := range positions {
		if inBounds(pos.x, pos.y, lenX, lenY) {
			res = append(res, pos)
		}
	}
	return res
}

func inBounds(x, y, lenX, lenY int) bool {
	return x >= 0 && x < lenX && y >= 0 && y < lenY
}
