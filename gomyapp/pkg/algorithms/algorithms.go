package algorithms

func fibonacci(num int) int {
	if num == 0 || num == 1 {
		return num
	} else {
		return (fibonacci(num-1) + fibonacci(num-2))
	}
}
