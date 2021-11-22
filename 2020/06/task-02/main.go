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
		split := strings.Split(lineStr, ",")
		deduped := split[0]

		for _, passenger := range split[1:] {
			refined := ""
			for _, c := range deduped {
				if strings.ContainsRune(passenger, c) {
					refined += string(c)
				}
			}
			deduped = refined
		}

		total += len(deduped)
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(total)
}
