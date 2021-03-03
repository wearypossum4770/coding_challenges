package veryeasy

import "fmt"

// Addition returns the summation of two integers...
func Addition(a, b int) int {
	message := "Sum Function"
	total := a + b
	fmt.Println(message, total)
	return total
}
