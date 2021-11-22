package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	lineLen := 0
	lines := [][]rune{}

	for scanner.Scan() {
		lineStr := []rune(scanner.Text())

		if len(lineStr) == 0 {
			continue
		}

		if lineLen == 0 {
			lineLen = len(lineStr)
		}

		lines = append(lines, lineStr)
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	rules := [][]int{
		{1, 1},
		{1, 3},
		{1, 5},
		{1, 7},
		{2, 1},
	}

	trees := []int{}

	for _, rule := range rules {
		down := rule[0]
		right := rule[1]
		treecount := 0

		for i, n := 0, 0; i < len(lines); i, n = i+down, n+1 {
			column := (n * right) % lineLen

			if lines[i][column] == '#' {
				treecount++
			}
		}

		trees = append(trees, treecount)
	}

	total := trees[0]

	for i := 1; i < len(trees); i++ {
		total *= trees[i]
	}

	fmt.Println(total)
}
