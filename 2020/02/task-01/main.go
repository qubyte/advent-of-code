package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	regex := regexp.MustCompile(`(\d+)-(\d+) (\w): (\w+)`)
	valid := 0

	for scanner.Scan() {
		lineStr := scanner.Text()

		if len(lineStr) == 0 {
			continue
		}

		match := regex.FindStringSubmatch(lineStr)

		from, _ := strconv.Atoi(match[1])
		to, _ := strconv.Atoi(match[2])
		letter := match[3]
		password := match[4]

		count := 0

		for _, char := range password {
			if string(char) == letter {
				count++
			}
		}

		if count >= from && count <= to {
			valid++
		}
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(valid)
}
