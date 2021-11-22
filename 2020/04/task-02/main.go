package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Document = map[string]string

func isValidYear(document Document, key string, min int64, max int64) bool {
	year, exists := document[key]

	if !exists {
		return false
	}

	value, err := strconv.ParseInt(year, 10, 16)

	if err != nil || value < min || value > max {
		return false
	}

	return true
}

func isValidHeight(document Document, key string) bool {
	value, exists := document[key]

	if !exists {
		return false
	}

	valuerunes := []rune(value)
	units := string(valuerunes[len(valuerunes)-2:])
	num, err := strconv.ParseInt(string(valuerunes[:len(valuerunes)-2]), 10, 16)

	if err != nil {
		return false
	}

	if units == "cm" && num >= 150 && num <= 193 {
		return true
	}

	if units == "in" && num >= 59 && num <= 76 {
		return true
	}

	return false
}

func isValidHairColor(document Document, key string) bool {
	value, exists := document[key]

	if !exists {
		return false
	}

	found, err := regexp.MatchString("^#[0-9a-f]{6}", value)

	if err != nil || found {
		return true
	}

	return false
}

func isValidEyeColor(document Document, key string) bool {
	value, exists := document[key]

	if !exists {
		return false
	}

	if value == "amb" || value == "blu" || value == "brn" || value == "gry" || value == "grn" || value == "hzl" || value == "oth" {
		return true
	}

	return false
}

func isValidPassportNumber(document Document, key string) bool {
	value, exists := document[key]

	if !exists {
		return false
	}

	found, err := regexp.MatchString(`^\d{9}$`, value)

	if err == nil && found {
		return true
	}

	return false
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	count := 0

	for scanner.Scan() {
		lineStr := strings.TrimSpace(scanner.Text())

		if len(lineStr) == 0 {
			continue
		}

		document := Document{}

		for _, item := range strings.Split(lineStr, " ") {
			nameval := strings.Split(item, ":")

			document[nameval[0]] = nameval[1]
		}

		if !isValidYear(document, "byr", 1920, 2002) {
			continue
		}
		if !isValidYear(document, "iyr", 2010, 2020) {
			continue
		}
		if !isValidYear(document, "eyr", 2020, 2030) {
			continue
		}
		if !isValidHeight(document, "hgt") {
			continue
		}
		if !isValidHairColor(document, "hcl") {
			continue
		}
		if !isValidEyeColor(document, "ecl") {
			continue
		}
		if !isValidPassportNumber(document, "pid") {
			continue
		}

		count++
	}

	if scanner.Err() != nil {
		os.Exit(1)
	}

	fmt.Println(count)
}
