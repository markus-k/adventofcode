package main

import (
	"log"
	"os"
	"strings"
)

func CountTrees(Map string, SlopeX int, SlopeY int) int {
	lines := strings.Split(Map, "\n")

	trees := 0

	x := 0

	for y, line := range lines {
		if line == "" || y%SlopeY != 0 {
			continue
		}

		//log.Printf("x = %d. y = %d", x, y)

		c := line[x%len(line)]

		if c == '#' {
			trees = trees + 1
		}

		x = x + SlopeX
	}

	return trees
}

func CheckAllSlopes(Map string) int {
	slopes := []struct {
		X, Y int
	}{{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}}

	product := 1

	for _, slope := range slopes {
		trees := CountTrees(Map, slope.X, slope.Y)

		log.Printf("Slope %dx%d has %d trees", slope.X, slope.Y, trees)

		product = product * trees
	}

	return product
}

func main() {
	mapFile, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	trees := CountTrees(string(mapFile), 3, 1)

	log.Printf("Found %d trees.", trees)

	product := CheckAllSlopes(string(mapFile))

	log.Printf("Product of all slopes is %d", product)
}
