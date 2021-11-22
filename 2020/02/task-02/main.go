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
		match := regex.FindStringSubmatch(lineStr)

		if len(match) == 0 {
			continue
		}

		first, _ := strconv.Atoi(match[1])
		second, _ := strconv.Atoi(match[2])
		letter := match[3]
		password := []rune(match[4])
		firstLetter := string(password[first-1])
		secondLetter := string(password[second-1])

		if firstLetter == secondLetter {
			continue
		}

		if firstLetter == letter || secondLetter == letter {
			valid++
		}
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(valid)
}
