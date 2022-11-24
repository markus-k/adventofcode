package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

type PasswordPolicy struct {
	Min      int
	Max      int
	Letter   rune
	Password string
}

func (p *PasswordPolicy) Validate() bool {
	count := 0
	for _, c := range p.Password {
		if c == p.Letter {
			count = count + 1
		}
	}

	return count >= p.Min && count <= p.Max
}

func (p *PasswordPolicy) ValidateToboggan() bool {
	return (p.Password[p.Min-1] == byte(p.Letter) && p.Password[p.Max-1] != byte(p.Letter)) ||
		(p.Password[p.Min-1] != byte(p.Letter) && p.Password[p.Max-1] == byte(p.Letter))
}

func CountValidPasswords(Passwords []PasswordPolicy, UseToboggan bool) int {
	count := 0

	for _, p := range Passwords {
		if (!UseToboggan && p.Validate()) || (UseToboggan && p.ValidateToboggan()) {
			count += 1
		}
	}

	return count
}

func ParseFile(File string) ([]PasswordPolicy, error) {
	contents, err := os.ReadFile(File)
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(contents), "\n")

	var passwords []PasswordPolicy

	for _, line := range lines {
		if line == "" {
			continue
		}
		var password PasswordPolicy
		_, err := fmt.Sscanf(
			line,
			"%d-%d %c: %s",
			&password.Min,
			&password.Max,
			&password.Letter,
			&password.Password,
		)
		if err != nil {
			return nil, err
		}

		passwords = append(passwords, password)
	}

	return passwords, nil
}

func main() {
	passwords, err := ParseFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	count := CountValidPasswords(passwords, false)
	log.Printf("Valid passwords: %d", count)

	count = CountValidPasswords(passwords, true)
	log.Printf("Valid passwords according to Toboggan: %d", count)
}
