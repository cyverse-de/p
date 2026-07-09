// Message types for the DE containers messages, formerly generated from containers.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package containers

import (
	"github.com/cyverse-de/p/go/ptypes"
)

// A container volume. Used for mapping host paths into a container.
//
// Correlates to the 'container_volumes' table in the 'de' database.
type Volume struct {
	// The absolute path to a file/directory on the container host/k8s node.
	HostPath string `json:"host_path"`
	// The path to mount the host path to inside of the container.
	ContainerPath string `json:"container_path"`
	// Whether the mount point in the container should be read only.
	ReadOnly bool `json:"read_only"`
	// The mode the mount point should have once mounted.
	Mode string `json:"mode"`
}

func (x *Volume) GetHostPath() string {
	if x != nil {
		return x.HostPath
	}
	return ""
}

func (x *Volume) GetContainerPath() string {
	if x != nil {
		return x.ContainerPath
	}
	return ""
}

func (x *Volume) GetReadOnly() bool {
	if x != nil {
		return x.ReadOnly
	}
	return false
}

func (x *Volume) GetMode() string {
	if x != nil {
		return x.Mode
	}
	return ""
}

// A port the container exposes.
//
// Correlates to the 'container_ports' table in the 'de' database.
type Port struct {
	// The port on the host that the container port should be mapped to. Usually unset.
	HostPort int32 `json:"host_port"`
	// The port the contained process needs to have open.
	ContainerPort int32 `json:"container_port"`
	// Whether to bind the container port to the host port. Normally left to false/null/None.
	BindToHost bool `json:"bind_to_host"`
}

func (x *Port) GetHostPort() int32 {
	if x != nil {
		return x.HostPort
	}
	return 0
}

func (x *Port) GetContainerPort() int32 {
	if x != nil {
		return x.ContainerPort
	}
	return 0
}

func (x *Port) GetBindToHost() bool {
	if x != nil {
		return x.BindToHost
	}
	return false
}

// A host device that the container needs access to.
//
// Correlates to the 'container_devices' table in the 'de' database.
type Device struct {
	// The path to the device on the host.
	HostPath string `json:"host_path"`
	// The path to the device in the container.
	ContainerPath string `json:"container_path"`
	// The permissions needed to mount the device.
	CgroupPermissions string `json:"cgroup_permissions"`
}

func (x *Device) GetHostPath() string {
	if x != nil {
		return x.HostPath
	}
	return ""
}

func (x *Device) GetContainerPath() string {
	if x != nil {
		return x.ContainerPath
	}
	return ""
}

func (x *Device) GetCgroupPermissions() string {
	if x != nil {
		return x.CgroupPermissions
	}
	return ""
}

// Another container from which the container should mount volumes.
//
// Correlates to the 'container_volumes_from' table in the 'de' database.
type VolumesFrom struct {
	// The image tag for the mountee container.
	Tag string `json:"tag"`
	// The name of the mountee container.
	Name string `json:"name"`
	// Authentication information needed to pull the mountee container image.
	Auth string `json:"auth"`
	// The prefix that should be given to the name of the container.
	NamePrefix string `json:"name_prefix"`
	// The URL to the mountee container image.
	Url string `json:"url"`
	// The path inside the mountee container that should be mounted.
	HostPath string `json:"host_path"`
	// The path to the mount point inside the mounting container.
	ContainerPath string `json:"container_path"`
	// Whether the mount point is read only.
	ReadOnly bool `json:"read_only"`
}

func (x *VolumesFrom) GetTag() string {
	if x != nil {
		return x.Tag
	}
	return ""
}

func (x *VolumesFrom) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *VolumesFrom) GetAuth() string {
	if x != nil {
		return x.Auth
	}
	return ""
}

func (x *VolumesFrom) GetNamePrefix() string {
	if x != nil {
		return x.NamePrefix
	}
	return ""
}

func (x *VolumesFrom) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

func (x *VolumesFrom) GetHostPath() string {
	if x != nil {
		return x.HostPath
	}
	return ""
}

func (x *VolumesFrom) GetContainerPath() string {
	if x != nil {
		return x.ContainerPath
	}
	return ""
}

func (x *VolumesFrom) GetReadOnly() bool {
	if x != nil {
		return x.ReadOnly
	}
	return false
}

// Information about a container image.
//
// Correlates to the 'container_images' table in the 'de' database.
type Image struct {
	// The UUID of the container image.
	Id string `json:"id"`
	// The name of the container image.
	Name string `json:"name"`
	// The container image's tag.
	Tag string `json:"tag"`
	// The URL for the image.
	Url string `json:"url"`
	// The path to the image in OSG. Might be blank.
	OsgImagePath string `json:"osg_image_path"`
}

func (x *Image) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Image) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Image) GetTag() string {
	if x != nil {
		return x.Tag
	}
	return ""
}

func (x *Image) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

func (x *Image) GetOsgImagePath() string {
	if x != nil {
		return x.OsgImagePath
	}
	return ""
}

// Contains information needed to launch an app in VICE.
//
// Corresponds roughly to to the 'interactive_apps_proxy_settings' table in the 'de' database.
type InteractiveApps struct {
	// The image containing the proxy application that requests to a VICE app flow through.
	ProxyImage string `json:"proxyImage"`
	// The name of the proxy container.
	ProxyName string `json:"proxyName"`
	// The URL the proxy will redirect users to when necessary.
	FrontendUrl string `json:"frontendUrl"`
	// The URL to the CAS instance needed for authentication.
	CasUrl string `json:"casUrl"`
	// The URL to the CAS authentication validation.
	CasValidate string `json:"casValidate"`
	// The path to the SSL cert in the container.
	SslCertPath string `json:"sslCertPath"`
	// The path to the SSL key in the container.
	SslKeyPath string `json:"sslKeyPath"`
	// Unused.
	WebsocketPath string `json:"websocketPath"`
	// Unused.
	WebsocketPort string `json:"websocketPort"`
	// Unused.
	WebsocketProto string `json:"websocketProto"`
	// Unused.
	BackendUrl string `json:"backendUrl"`
}

func (x *InteractiveApps) GetProxyImage() string {
	if x != nil {
		return x.ProxyImage
	}
	return ""
}

func (x *InteractiveApps) GetProxyName() string {
	if x != nil {
		return x.ProxyName
	}
	return ""
}

func (x *InteractiveApps) GetFrontendUrl() string {
	if x != nil {
		return x.FrontendUrl
	}
	return ""
}

func (x *InteractiveApps) GetCasUrl() string {
	if x != nil {
		return x.CasUrl
	}
	return ""
}

func (x *InteractiveApps) GetCasValidate() string {
	if x != nil {
		return x.CasValidate
	}
	return ""
}

func (x *InteractiveApps) GetSslCertPath() string {
	if x != nil {
		return x.SslCertPath
	}
	return ""
}

func (x *InteractiveApps) GetSslKeyPath() string {
	if x != nil {
		return x.SslKeyPath
	}
	return ""
}

func (x *InteractiveApps) GetWebsocketPath() string {
	if x != nil {
		return x.WebsocketPath
	}
	return ""
}

func (x *InteractiveApps) GetWebsocketPort() string {
	if x != nil {
		return x.WebsocketPort
	}
	return ""
}

func (x *InteractiveApps) GetWebsocketProto() string {
	if x != nil {
		return x.WebsocketProto
	}
	return ""
}

func (x *InteractiveApps) GetBackendUrl() string {
	if x != nil {
		return x.BackendUrl
	}
	return ""
}

// A representation of a container for an analysis.
//
// Correlates to the 'container_settings' table in the 'de' database..
type Container struct {
	Id              string           `json:"id"`
	Volumes         []*Volume        `json:"container_volumes"`
	Devices         []*Device        `json:"container_devices"`
	VolumesFrom     []*VolumesFrom   `json:"container_volumes_from"`
	Name            string           `json:"name"`
	NetworkMode     string           `json:"network_mode"`
	CpuShares       ptypes.Int64     `json:"cpu_shares"`
	InteractiveApps *InteractiveApps `json:"interactive_apps"`
	MemoryLimit     ptypes.Int64     `json:"memory_limit"`
	MinMemoryLimit  ptypes.Int64     `json:"min_memory_limit"`
	MaxCpuCores     float32          `json:"max_cpu_cores"`
	MinCpuCores     float32          `json:"min_cpu_cores"`
	MinDiskSpace    ptypes.Int64     `json:"min_disk_space"`
	PidsLimit       ptypes.Int64     `json:"pids_limit"`
	Image           *Image           `json:"image"`
	EntryPoint      string           `json:"entrypoint"`
	WorkingDir      string           `json:"working_directory"`
	Ports           []*Port          `json:"ports"`
	SkipTmpMount    bool             `json:"skip_tmp_mount"`
	Uid             int32            `json:"uid"`
}

func (x *Container) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Container) GetVolumes() []*Volume {
	if x != nil {
		return x.Volumes
	}
	return nil
}

func (x *Container) GetDevices() []*Device {
	if x != nil {
		return x.Devices
	}
	return nil
}

func (x *Container) GetVolumesFrom() []*VolumesFrom {
	if x != nil {
		return x.VolumesFrom
	}
	return nil
}

func (x *Container) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Container) GetNetworkMode() string {
	if x != nil {
		return x.NetworkMode
	}
	return ""
}

func (x *Container) GetCpuShares() ptypes.Int64 {
	if x != nil {
		return x.CpuShares
	}
	return 0
}

func (x *Container) GetInteractiveApps() *InteractiveApps {
	if x != nil {
		return x.InteractiveApps
	}
	return nil
}

func (x *Container) GetMemoryLimit() ptypes.Int64 {
	if x != nil {
		return x.MemoryLimit
	}
	return 0
}

func (x *Container) GetMinMemoryLimit() ptypes.Int64 {
	if x != nil {
		return x.MinMemoryLimit
	}
	return 0
}

func (x *Container) GetMaxCpuCores() float32 {
	if x != nil {
		return x.MaxCpuCores
	}
	return 0
}

func (x *Container) GetMinCpuCores() float32 {
	if x != nil {
		return x.MinCpuCores
	}
	return 0
}

func (x *Container) GetMinDiskSpace() ptypes.Int64 {
	if x != nil {
		return x.MinDiskSpace
	}
	return 0
}

func (x *Container) GetPidsLimit() ptypes.Int64 {
	if x != nil {
		return x.PidsLimit
	}
	return 0
}

func (x *Container) GetImage() *Image {
	if x != nil {
		return x.Image
	}
	return nil
}

func (x *Container) GetEntryPoint() string {
	if x != nil {
		return x.EntryPoint
	}
	return ""
}

func (x *Container) GetWorkingDir() string {
	if x != nil {
		return x.WorkingDir
	}
	return ""
}

func (x *Container) GetPorts() []*Port {
	if x != nil {
		return x.Ports
	}
	return nil
}

func (x *Container) GetSkipTmpMount() bool {
	if x != nil {
		return x.SkipTmpMount
	}
	return false
}

func (x *Container) GetUid() int32 {
	if x != nil {
		return x.Uid
	}
	return 0
}
