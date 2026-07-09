.PHONY: all test tidy

MODULES = go/analysis go/apps go/containers go/header go/monitoring go/ptypes go/qms go/requests go/svcerror go/user goldens

all: test

test:
	@for m in $(MODULES); do echo "== $$m"; (cd $$m && go test ./...) || exit 1; done

# Requires all inter-module dependency versions in the go.mod files to be
# tagged and pushed; day-to-day development resolves them via go.work instead.
tidy:
	@for m in $(MODULES); do echo "== $$m"; (cd $$m && go mod tidy) || exit 1; done
