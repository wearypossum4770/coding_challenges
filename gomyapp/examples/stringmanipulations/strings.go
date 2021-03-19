package strings

import (
	"bytes"
	"strings"
)
// RevcoderseWords is ...
func ReverseWords(input string) string {
	words := strings.Fields(input)
	length := len(words)
	var buff bytes.Buffer
	for i := 0; i < length; i++ {
		buff.WriteString(words[i])
		buff.WriteString(" ")
	}
	reversed := buff.String()
	return reversed
}

// DefangIPAddr is ...
func DefangIPAddr(ipAddress string) string {
	// defangs ip
	var buff bytes.Buffer
	for i := 0; i < len(ipAddress); i++ {
		if ipAddress[i] == '.' {
			buff.WriteString("[.]")
		} else {
			buff.WriteByte(ipAddress[i])
		}
	}
	ip := buff.String()
	return ip
}

