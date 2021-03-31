package veryeasy

import (
	"testing"
)

func TestFibonacci(t *testing.T) {
	var tests = []struct {
		a        int
		b        int
		expected int
	}{

		{100, 1000, 1100},
		{2, 3, 5},
	}
	for _, test := range tests {
		if output := Addition(test.a, test.b); output != test.expected {
			t.Error("Test Failed: {} {} inputted, {} expected, recieved: {}", test.a, test.b, test.expected, output)
		}
	}
}
