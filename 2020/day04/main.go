package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func ParsePassportFields(PassportString string) map[string]string {
	fields := make(map[string]string)
	fieldStrings := strings.Fields(PassportString)

	for _, fieldString := range fieldStrings {
		splittedField := strings.SplitN(fieldString, ":", 2)
		name := strings.Trim(splittedField[0], " \n")
		value := strings.Trim(splittedField[1], " \n")

		fields[name] = value
	}

	return fields
}

func ParsePassports(Input string) []map[string]string {
	passportStrings := strings.Split(Input, "\n\n")
	passports := []map[string]string{}

	for _, passportString := range passportStrings {
		fields := ParsePassportFields(passportString)

		passports = append(passports, fields)
	}

	return passports
}

func ValidatePassport(Passport map[string]string) bool {
	requiredFields := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

	hasAllFields := true
	for _, name := range requiredFields {
		_, ok := Passport[name]

		if !ok {
			hasAllFields = false
			break
		}
	}

	return hasAllFields
}

func ValidateYearField(Passport map[string]string, Field string, Min int, Max int) bool {
	yearString, ok := Passport[Field]
	if !ok {
		return false
	}

	year, err := strconv.Atoi(yearString)
	if err != nil {
		return false
	}

	return year >= Min && year <= Max
}

func ValidateHeightField(Passport map[string]string, Field string) bool {
	field, ok := Passport[Field]
	if !ok {
		return false
	}

	if strings.HasSuffix(field, "cm") {
		var height int
		_, err := fmt.Sscanf(field, "%dcm", &height)
		if err != nil {
			return false
		}

		return height >= 150 && height <= 193
	} else if strings.HasSuffix(field, "in") {
		var height int
		_, err := fmt.Sscanf(field, "%din", &height)
		if err != nil {
			return false
		}

		return height >= 59 && height <= 76
	}

	return false
}

func ValidateRegexField(Passport map[string]string, Field string, Regex string) bool {
	field, ok := Passport[Field]
	if !ok {
		return false
	}

	matches, err := regexp.MatchString(Regex, field)
	if err != nil {
		panic(err)
	}

	return matches
}

func ValidatePassportFields(Passport map[string]string) bool {
	return ValidateYearField(Passport, "byr", 1920, 2002) &&
		ValidateYearField(Passport, "iyr", 2010, 2020) &&
		ValidateYearField(Passport, "eyr", 2020, 2030) &&
		ValidateHeightField(Passport, "hgt") &&
		ValidateRegexField(Passport, "hcl", "^#[0-9a-f]{6}$") &&
		ValidateRegexField(Passport, "ecl", "^(amb|blu|brn|gry|grn|hzl|oth)$") &&
		ValidateRegexField(Passport, "pid", "^[0-9]{9}$")
}

func CountCompletePassports(Passports []map[string]string) int {
	count := 0
	for _, passport := range Passports {
		if ValidatePassport(passport) {
			count += 1
		}
	}

	return count
}

func CountValidPassports(Passports []map[string]string) int {
	count := 0
	for _, passport := range Passports {
		if ValidatePassportFields(passport) {
			count += 1
		}
	}

	return count
}

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	passports := ParsePassports(string(input))

	count := CountCompletePassports(passports)
	log.Println("Complete passports: ", count)

	count = CountValidPassports(passports)
	log.Println("Valid passports: ", count)
}
