module github.com/cyverse-de/p/go/tools

go 1.20

replace github.com/cyverse-de/p/go/containers => ../containers

require (
	github.com/cyverse-de/p/go/apps v0.0.0-20230417204035-621d538d8e56
	github.com/cyverse-de/p/go/containers v0.0.0-00010101000000-000000000000
	google.golang.org/protobuf v1.30.0
)

require (
	github.com/cyverse-de/p/go/header v0.0.1 // indirect
	github.com/cyverse-de/p/go/svcerror v0.0.5 // indirect
	github.com/cyverse-de/p/go/user v0.0.10 // indirect
)
