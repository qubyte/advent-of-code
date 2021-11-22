package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	required := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	scanner := bufio.NewScanner(os.Stdin)
	count := 0

	for scanner.Scan() {
		lineStr := strings.TrimSpace(scanner.Text())

		if len(lineStr) == 0 {
			continue
		}

		document := map[string]string{}

		for _, item := range strings.Split(lineStr, " ") {
			nameval := strings.Split(item, ":")

			document[nameval[0]] = nameval[1]
		}

		valid := true

		for _, key := range required {
			_, exists := document[key]

			if !exists {
				valid = false
				break
			}
		}

		if valid {
			count++
		}
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(count)
}
