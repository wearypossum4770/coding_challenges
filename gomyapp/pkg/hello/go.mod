module hello

go 1.16

replace (
	example.com/algorithms => ../algorithms
	example.com/arraymanipulations => ../arraymanipulations
	example.com/greetings => ../greetings
	example.com/stringmanipulations => ../stringmanipulations
	example.com/veryeasy => ../veryeasy
)

require (
	example.com/arraymanipulations v0.0.0-00010101000000-000000000000 // indirect
	example.com/greetings v0.0.0-00010101000000-000000000000 // indirect
	example.com/stringmanipulations v0.0.0-00010101000000-000000000000 // indirect
	example.com/veryeasy v0.0.0-00010101000000-000000000000 // indirect
	github.com/ramya-rao-a/go-outline v0.0.0-20200117021646-2a048b4510eb // indirect
	github.com/uudashr/gopkgs/v2 v2.1.2 // indirect
	golang.org/x/tools v0.1.0 // indirect
)
