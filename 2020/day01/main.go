package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func ParseNumbers(File string) ([]int, error) {
	contents, err := os.ReadFile(File)
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(contents), "\n")

	var numbers []int
	for _, line := range lines {
		if line == "" {
			continue
		}
		number, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		numbers = append(numbers, number)
	}

	return numbers, nil
}

func Find2020Numbers(Numbers []int) (int, int) {
	for _, n1 := range Numbers {
		for _, n2 := range Numbers {
			if n1+n2 == 2020 {
				return n1, n2
			}
		}
	}

	return 0, 0
}

func Find2020Numbers3(Numbers []int) (int, int, int) {
	for _, n1 := range Numbers {
		for _, n2 := range Numbers {
			for _, n3 := range Numbers {
				if n1+n2+n3 == 2020 {
					return n1, n2, n3
				}
			}
		}
	}

	return 0, 0, 0
}

func main() {
	numbers, err := ParseNumbers("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	n1, n2 := Find2020Numbers(numbers)

	log.Println("2 numbers are: ", n1, n2)
	log.Println("Multiplied that is ", n1*n2)

	n1, n2, n3 := Find2020Numbers3(numbers)

	log.Println("3 numbers are: ", n1, n2, n3)
	log.Println("Multiplied that is ", n1*n2*n3)

}
