// Canonical resource type name strings used on the wire.
// Every service that sends or matches a resource type name should use these
// constants instead of raw string literals to avoid silent mismatches.

package qms

const (
	// ResourceTypeCPUHours is the resource type name for CPU hour tracking.
	ResourceTypeCPUHours = "cpu.hours"

	// ResourceTypeDataSize is the resource type name for data storage tracking.
	ResourceTypeDataSize = "data.size"

	// ResourceTypeGPUHours is the resource type name for GPU hour tracking.
	ResourceTypeGPUHours = "gpu.hours"
)
