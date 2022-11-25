package main

import (
	"os"
	"testing"
)

func TestCountTrees(t *testing.T) {
	map_file, err := os.ReadFile("test.txt")
	if err != nil {
		t.Fatal(err)
	}

	trees := CountTrees(string(map_file), 3, 1)

	if trees != 7 {
		t.Errorf("Tree count not correct: %d", trees)
	}

	product := CheckAllSlopes(string(map_file))

	if product != 336 {
		t.Errorf("Product of all slopes not correct: %d", product)
	}
}
