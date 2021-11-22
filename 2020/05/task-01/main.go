package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	highest := 0

	for scanner.Scan() {
		lineStr := scanner.Text()

		if len(lineStr) == 0 {
			continue
		}

		encodedRows := lineStr[:7]
		encodedRowLen := len(encodedRows)
		encodedCols := lineStr[7:]
		encodedColLen := len(encodedCols)

		row := 127

		for i, c := range encodedRows {
			if c == 'F' {
				row -= (1 << (encodedRowLen - 1 - i))
			}
		}

		col := 7

		for i, c := range encodedCols {
			if c == 'L' {
				col -= (1 << (encodedColLen - 1 - i))
			}
		}

		id := row*8 + col

		if id > highest {
			highest = id
		}
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(highest)
}
