package stringmanipulations

import "bytes"

// ReverseWords is ...
func ReverseWords(input string) string {
	reversed := ""
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
