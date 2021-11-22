package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	var lines []int

	for scanner.Scan() {
		lineStr := scanner.Text()
		num, _ := strconv.Atoi(lineStr)
		lines = append(lines, num)
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	length := len(lines)

outer:
	for i := 0; i < (length - 2); i++ {
		a := lines[i]

		for j := i + 1; j < (length - 1); j++ {
			b := lines[j]

			for k := j + 1; k < length; k++ {
				c := lines[k]

				if a+b+c == 2020 {
					fmt.Println(a * b * c)
					break outer
				}
			}
		}
	}
}
