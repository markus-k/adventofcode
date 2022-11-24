package main

import (
	"testing"
)

func TestFind2020Numbers(t *testing.T) {
	numbers, err := ParseNumbers("test.txt")
	if err != nil {
		t.Fatal(err)
	}

	n1, n2 := Find2020Numbers(numbers)

	if n1+n2 != 2020 {
		t.Errorf("%d and %d don't add up to 2020", n1, n2)
	}

	if n1*n2 != 514579 {
		t.Error("Multiplication of numbers yields unexpected result.")
	}
}
func TestFind2020Numbers3(t *testing.T) {
	numbers, err := ParseNumbers("test.txt")
	if err != nil {
		t.Fatal(err)
	}

	n1, n2, n3 := Find2020Numbers3(numbers)

	if n1+n2+n3 != 2020 {
		t.Errorf("%d, %d and %d don't add up to 2020", n1, n2, n3)
	}

	if n1*n2*n3 != 241861950 {
		t.Error("Multiplication of numbers yields unexpected result.")
	}
}
