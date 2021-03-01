package defangipaddr

import (
	"testing"
)

func testDefangIPAddr(t *testing.T) {
	var tests = []struct {
		input    string
		expected string
	}{
		{"1.1.1.1", "1[.]1[.]1[.]1"},
		{"255.100.50.0", "255[.]100[.]50[.]0"},
	}
	for _, test := range tests {
		if output := defangipaddr(test.input); output != test.expected {
			t.Error("Test Failed: {} inputted, {} expected, recieved: {}", test.input, test.expected, output)
		}
	}
}
