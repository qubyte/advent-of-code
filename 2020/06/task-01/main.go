package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	total := 0

	for scanner.Scan() {
		lineStr := scanner.Text()
		deduped := ""

		for _, c := range lineStr {
			if c != ',' && !strings.ContainsRune(deduped, c) {
				deduped += string(c)
			}
		}

		total += len(deduped)
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(total)
}
