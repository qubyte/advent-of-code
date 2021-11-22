package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	lineLen := 0
	trees := 0
	line := -1

	for scanner.Scan() {
		lineStr := []rune(scanner.Text())

		if len(lineStr) == 0 {
			continue
		}

		line++

		if line == 0 {
			lineLen = len(lineStr)
		}

		column := (line * 3) % lineLen

		if lineStr[column] == '#' {
			trees++
		}
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(trees)
}
