package main

import "testing"

func TestCountValidPasswords(t *testing.T) {
	passwords, err := ParseFile("test.txt")
	if err != nil {
		t.Fatal(err)
	}

	count := CountValidPasswords(passwords, false)
	if count != 2 {
		t.Errorf("Valid count is wrong: %d", count)
	}

	count = CountValidPasswords(passwords, true)
	if count != 1 {
		t.Errorf("Valid count for toboggan is wrong: %d", count)
	}

}
