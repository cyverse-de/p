module github.com/cyverse-de/p/goldens

go 1.25.0

require (
	github.com/cyverse-de/p/go/analysis v0.0.0
	github.com/cyverse-de/p/go/apps v0.0.3
	github.com/cyverse-de/p/go/containers v0.0.3
	github.com/cyverse-de/p/go/header v0.0.5
	github.com/cyverse-de/p/go/monitoring v0.0.0
	github.com/cyverse-de/p/go/qms v0.0.0
	github.com/cyverse-de/p/go/requests v0.0.0
	github.com/cyverse-de/p/go/svcerror v0.0.9
	github.com/cyverse-de/p/go/user v0.0.12
	google.golang.org/protobuf v1.36.11
)

replace (
	github.com/cyverse-de/p/go/analysis => ../go/analysis
	github.com/cyverse-de/p/go/apps => ../go/apps
	github.com/cyverse-de/p/go/containers => ../go/containers
	github.com/cyverse-de/p/go/header => ../go/header
	github.com/cyverse-de/p/go/monitoring => ../go/monitoring
	github.com/cyverse-de/p/go/qms => ../go/qms
	github.com/cyverse-de/p/go/requests => ../go/requests
	github.com/cyverse-de/p/go/svcerror => ../go/svcerror
	github.com/cyverse-de/p/go/user => ../go/user
)
