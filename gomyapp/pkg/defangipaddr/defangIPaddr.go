package defangipaddr

import "bytes"

func defangipaddr(ipAddress string) string {
	// defangs ip
	var buff bytes.Buffer
	buff.WriteString("something")
	ip := buff.String()
	return ip
}
