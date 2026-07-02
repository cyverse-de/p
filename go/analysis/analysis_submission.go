// Message types for the DE analysis messages, formerly generated from analysis_submission.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package analysis

import (
	"github.com/cyverse-de/p/go/containers"
	"github.com/cyverse-de/p/go/ptypes"
)

type HTCondorExtraInfo struct {
	ExtraRequirements string `json:"extraRequirements"`
}

func (x *HTCondorExtraInfo) GetExtraRequirements() string {
	if x != nil {
		return x.ExtraRequirements
	}
	return ""
}

type Extra struct {
	HtCondor *HTCondorExtraInfo `json:"htCondor"`
}

func (x *Extra) GetHtCondor() *HTCondorExtraInfo {
	if x != nil {
		return x.HtCondor
	}
	return nil
}

type FileMetadata struct {
	Attribute string `json:"attribute"`
	Value     string `json:"value"`
	Unit      string `json:"unit"`
}

func (x *FileMetadata) GetAttribute() string {
	if x != nil {
		return x.Attribute
	}
	return ""
}

func (x *FileMetadata) GetValue() string {
	if x != nil {
		return x.Value
	}
	return ""
}

func (x *FileMetadata) GetUnit() string {
	if x != nil {
		return x.Unit
	}
	return ""
}

type Step struct {
	Component   *Step_Component   `json:"component"`
	Config      *Step_Config      `json:"config"`
	Type        string            `json:"type"`
	StdinPath   string            `json:"stdin"`
	StdoutPath  string            `json:"stdout"`
	StderrPath  string            `json:"stderr"`
	LogFile     string            `json:"log-file"`
	Environment map[string]string `json:"environment"`
	Input       []*Step_Input     `json:"input"`
	Output      []*Step_Output    `json:"output"`
}

func (x *Step) GetComponent() *Step_Component {
	if x != nil {
		return x.Component
	}
	return nil
}

func (x *Step) GetConfig() *Step_Config {
	if x != nil {
		return x.Config
	}
	return nil
}

func (x *Step) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *Step) GetStdinPath() string {
	if x != nil {
		return x.StdinPath
	}
	return ""
}

func (x *Step) GetStdoutPath() string {
	if x != nil {
		return x.StdoutPath
	}
	return ""
}

func (x *Step) GetStderrPath() string {
	if x != nil {
		return x.StderrPath
	}
	return ""
}

func (x *Step) GetLogFile() string {
	if x != nil {
		return x.LogFile
	}
	return ""
}

func (x *Step) GetEnvironment() map[string]string {
	if x != nil {
		return x.Environment
	}
	return nil
}

func (x *Step) GetInput() []*Step_Input {
	if x != nil {
		return x.Input
	}
	return nil
}

func (x *Step) GetOutput() []*Step_Output {
	if x != nil {
		return x.Output
	}
	return nil
}

type Step_Component struct {
	Container     *containers.Container `json:"container"`
	Type          string                `json:"type"`
	Name          string                `json:"name"`
	Location      string                `json:"location"`
	Description   string                `json:"description"`
	TimeLimit     int32                 `json:"time_limit_seconds"`
	Restricted    bool                  `json:"restricted"`
	IsInteractive bool                  `json:"interactive"`
}

func (x *Step_Component) GetContainer() *containers.Container {
	if x != nil {
		return x.Container
	}
	return nil
}

func (x *Step_Component) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *Step_Component) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Step_Component) GetLocation() string {
	if x != nil {
		return x.Location
	}
	return ""
}

func (x *Step_Component) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Step_Component) GetTimeLimit() int32 {
	if x != nil {
		return x.TimeLimit
	}
	return 0
}

func (x *Step_Component) GetRestricted() bool {
	if x != nil {
		return x.Restricted
	}
	return false
}

func (x *Step_Component) GetIsInteractive() bool {
	if x != nil {
		return x.IsInteractive
	}
	return false
}

type Step_Param struct {
	Id    string `json:"id"`
	Name  string `json:"name"`
	Value string `json:"value"`
	Order int32  `json:"order"`
	Type  string `json:"type"`
	Path  string `json:"path"`
}

func (x *Step_Param) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Step_Param) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Step_Param) GetValue() string {
	if x != nil {
		return x.Value
	}
	return ""
}

func (x *Step_Param) GetOrder() int32 {
	if x != nil {
		return x.Order
	}
	return 0
}

func (x *Step_Param) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *Step_Param) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

type Step_Config struct {
	Params  []*Step_Param  `json:"params"`
	Inputs  []*Step_Input  `json:"input"`
	Outputs []*Step_Output `json:"output"`
}

func (x *Step_Config) GetParams() []*Step_Param {
	if x != nil {
		return x.Params
	}
	return nil
}

func (x *Step_Config) GetInputs() []*Step_Input {
	if x != nil {
		return x.Inputs
	}
	return nil
}

func (x *Step_Config) GetOutputs() []*Step_Output {
	if x != nil {
		return x.Outputs
	}
	return nil
}

type Step_Input struct {
	Id           string `json:"id"`
	Ticket       string `json:"ticket"`
	Multiplicity string `json:"multiplicity"`
	Name         string `json:"name"`
	Property     string `json:"property"`
	Retain       bool   `json:"retain"`
	Type         string `json:"type"`
	Value        string `json:"value"`
}

func (x *Step_Input) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Step_Input) GetTicket() string {
	if x != nil {
		return x.Ticket
	}
	return ""
}

func (x *Step_Input) GetMultiplicity() string {
	if x != nil {
		return x.Multiplicity
	}
	return ""
}

func (x *Step_Input) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Step_Input) GetProperty() string {
	if x != nil {
		return x.Property
	}
	return ""
}

func (x *Step_Input) GetRetain() bool {
	if x != nil {
		return x.Retain
	}
	return false
}

func (x *Step_Input) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *Step_Input) GetValue() string {
	if x != nil {
		return x.Value
	}
	return ""
}

type Step_Output struct {
	Multiplicity string `json:"multiplicity"`
	Name         string `json:"name"`
	Property     string `json:"property"`
	QualId       string `json:"qual-id"`
	Retain       bool   `json:"retain"`
	Type         string `json:"type"`
}

func (x *Step_Output) GetMultiplicity() string {
	if x != nil {
		return x.Multiplicity
	}
	return ""
}

func (x *Step_Output) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Step_Output) GetProperty() string {
	if x != nil {
		return x.Property
	}
	return ""
}

func (x *Step_Output) GetQualId() string {
	if x != nil {
		return x.QualId
	}
	return ""
}

func (x *Step_Output) GetRetain() bool {
	if x != nil {
		return x.Retain
	}
	return false
}

func (x *Step_Output) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

type AnalysisSubmission struct {
	AppDescription     string            `json:"app_description"`
	AppId              string            `json:"app_id"`
	AppName            string            `json:"app_name"`
	ArchiveLogs        bool              `json:"archive_logs"`
	BatchId            string            `json:"batch_id"`
	CondorId           string            `json:"condor_id"`
	CondorLogPath      string            `json:"condor_log_path"`
	CreateOutputSubdir bool              `json:"create_output_subdir"`
	DateSubmitted      *ptypes.Timestamp `json:"date_submitted"`
	DateStarted        *ptypes.Timestamp `json:"date_started"`
	DateCompleted      *ptypes.Timestamp `json:"date_completed"`
	Description        string            `json:"description"`
	Email              string            `json:"email"`
	Extra              *Extra            `json:"extra"`
	ExecutionTarget    string            `json:"execution_target"`
	ExitCode           int32             `json:"exit_code"`
	FailureCount       ptypes.Int64      `json:"failure_count"`
	FailureThreshold   ptypes.Int64      `json:"failure_threshold"`
	FileMetadata       []*FileMetadata   `json:"file-metadata"`
	FilterFiles        []string          `json:"filter_files"`
	Group              string            `json:"group"`
	InputPathListFile  string            `json:"input_path_list"`
	InputTicketsFile   string            `json:"input_ticket_list"`
	InvocationId       string            `json:"invocation_id"`
	IrodsBase          string            `json:"irods_base"`
	Name               string            `json:"name"`
	NfsBase            string            `json:"nfs_base"`
	Notify             bool              `json:"notify"`
	NowDate            string            `json:"now_date"`
	OutputDir          string            `json:"output_dir"`
	OutputDirTicket    string            `json:"output_dir_ticket"`
	OutputTicketFile   string            `json:"output_ticket_file"`
	RequestType        string            `json:"request_type"`
	RunOnNfs           bool              `json:"run-on-nfs"`
	SkipParentMetadata bool              `json:"skip-parent-meta"`
	Steps              []*Step           `json:"steps"`
	SubmissionDate     string            `json:"submission_date"`
	Submitter          string            `json:"username"`
	Type               string            `json:"type"`
	UserId             string            `json:"user_id"`
	UserGroups         []string          `json:"user_groups"`
	UserHome           string            `json:"user_home"`
	WikiUrl            string            `json:"wiki_url"`
	ConfigFile         string            `json:"config_file"`
	MountDataStore     bool              `json:"mount_data_store"`
}

func (x *AnalysisSubmission) GetAppDescription() string {
	if x != nil {
		return x.AppDescription
	}
	return ""
}

func (x *AnalysisSubmission) GetAppId() string {
	if x != nil {
		return x.AppId
	}
	return ""
}

func (x *AnalysisSubmission) GetAppName() string {
	if x != nil {
		return x.AppName
	}
	return ""
}

func (x *AnalysisSubmission) GetArchiveLogs() bool {
	if x != nil {
		return x.ArchiveLogs
	}
	return false
}

func (x *AnalysisSubmission) GetBatchId() string {
	if x != nil {
		return x.BatchId
	}
	return ""
}

func (x *AnalysisSubmission) GetCondorId() string {
	if x != nil {
		return x.CondorId
	}
	return ""
}

func (x *AnalysisSubmission) GetCondorLogPath() string {
	if x != nil {
		return x.CondorLogPath
	}
	return ""
}

func (x *AnalysisSubmission) GetCreateOutputSubdir() bool {
	if x != nil {
		return x.CreateOutputSubdir
	}
	return false
}

func (x *AnalysisSubmission) GetDateSubmitted() *ptypes.Timestamp {
	if x != nil {
		return x.DateSubmitted
	}
	return nil
}

func (x *AnalysisSubmission) GetDateStarted() *ptypes.Timestamp {
	if x != nil {
		return x.DateStarted
	}
	return nil
}

func (x *AnalysisSubmission) GetDateCompleted() *ptypes.Timestamp {
	if x != nil {
		return x.DateCompleted
	}
	return nil
}

func (x *AnalysisSubmission) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *AnalysisSubmission) GetEmail() string {
	if x != nil {
		return x.Email
	}
	return ""
}

func (x *AnalysisSubmission) GetExtra() *Extra {
	if x != nil {
		return x.Extra
	}
	return nil
}

func (x *AnalysisSubmission) GetExecutionTarget() string {
	if x != nil {
		return x.ExecutionTarget
	}
	return ""
}

func (x *AnalysisSubmission) GetExitCode() int32 {
	if x != nil {
		return x.ExitCode
	}
	return 0
}

func (x *AnalysisSubmission) GetFailureCount() ptypes.Int64 {
	if x != nil {
		return x.FailureCount
	}
	return 0
}

func (x *AnalysisSubmission) GetFailureThreshold() ptypes.Int64 {
	if x != nil {
		return x.FailureThreshold
	}
	return 0
}

func (x *AnalysisSubmission) GetFileMetadata() []*FileMetadata {
	if x != nil {
		return x.FileMetadata
	}
	return nil
}

func (x *AnalysisSubmission) GetFilterFiles() []string {
	if x != nil {
		return x.FilterFiles
	}
	return nil
}

func (x *AnalysisSubmission) GetGroup() string {
	if x != nil {
		return x.Group
	}
	return ""
}

func (x *AnalysisSubmission) GetInputPathListFile() string {
	if x != nil {
		return x.InputPathListFile
	}
	return ""
}

func (x *AnalysisSubmission) GetInputTicketsFile() string {
	if x != nil {
		return x.InputTicketsFile
	}
	return ""
}

func (x *AnalysisSubmission) GetInvocationId() string {
	if x != nil {
		return x.InvocationId
	}
	return ""
}

func (x *AnalysisSubmission) GetIrodsBase() string {
	if x != nil {
		return x.IrodsBase
	}
	return ""
}

func (x *AnalysisSubmission) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *AnalysisSubmission) GetNfsBase() string {
	if x != nil {
		return x.NfsBase
	}
	return ""
}

func (x *AnalysisSubmission) GetNotify() bool {
	if x != nil {
		return x.Notify
	}
	return false
}

func (x *AnalysisSubmission) GetNowDate() string {
	if x != nil {
		return x.NowDate
	}
	return ""
}

func (x *AnalysisSubmission) GetOutputDir() string {
	if x != nil {
		return x.OutputDir
	}
	return ""
}

func (x *AnalysisSubmission) GetOutputDirTicket() string {
	if x != nil {
		return x.OutputDirTicket
	}
	return ""
}

func (x *AnalysisSubmission) GetOutputTicketFile() string {
	if x != nil {
		return x.OutputTicketFile
	}
	return ""
}

func (x *AnalysisSubmission) GetRequestType() string {
	if x != nil {
		return x.RequestType
	}
	return ""
}

func (x *AnalysisSubmission) GetRunOnNfs() bool {
	if x != nil {
		return x.RunOnNfs
	}
	return false
}

func (x *AnalysisSubmission) GetSkipParentMetadata() bool {
	if x != nil {
		return x.SkipParentMetadata
	}
	return false
}

func (x *AnalysisSubmission) GetSteps() []*Step {
	if x != nil {
		return x.Steps
	}
	return nil
}

func (x *AnalysisSubmission) GetSubmissionDate() string {
	if x != nil {
		return x.SubmissionDate
	}
	return ""
}

func (x *AnalysisSubmission) GetSubmitter() string {
	if x != nil {
		return x.Submitter
	}
	return ""
}

func (x *AnalysisSubmission) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *AnalysisSubmission) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

func (x *AnalysisSubmission) GetUserGroups() []string {
	if x != nil {
		return x.UserGroups
	}
	return nil
}

func (x *AnalysisSubmission) GetUserHome() string {
	if x != nil {
		return x.UserHome
	}
	return ""
}

func (x *AnalysisSubmission) GetWikiUrl() string {
	if x != nil {
		return x.WikiUrl
	}
	return ""
}

func (x *AnalysisSubmission) GetConfigFile() string {
	if x != nil {
		return x.ConfigFile
	}
	return ""
}

func (x *AnalysisSubmission) GetMountDataStore() bool {
	if x != nil {
		return x.MountDataStore
	}
	return false
}

// Since protocol buffers don't have a way to alias messages, we're copying the Analysis definition into Job and deprecating it.
//
// Deprecated: Marked as deprecated in analysis_submission.proto.
type Job struct {
	AppDescription     string            `json:"app_description"`
	AppId              string            `json:"app_id"`
	AppName            string            `json:"app_name"`
	ArchiveLogs        bool              `json:"archive_logs"`
	BatchId            string            `json:"batch_id"`
	CondorId           string            `json:"condor_id"`
	CondorLogPath      string            `json:"condor_log_path"`
	CreateOutputSubdir bool              `json:"create_output_subdir"`
	DateSubmitted      *ptypes.Timestamp `json:"date_submitted"`
	DateStarted        *ptypes.Timestamp `json:"date_started"`
	DateCompleted      *ptypes.Timestamp `json:"date_completed"`
	Description        string            `json:"description"`
	Email              string            `json:"email"`
	Extra              *Extra            `json:"extra"`
	ExecutionTarget    string            `json:"execution_target"`
	ExitCode           int32             `json:"exit_code"`
	FailureCount       ptypes.Int64      `json:"failure_count"`
	FailureThreshold   ptypes.Int64      `json:"failure_threshold"`
	FileMetadata       []*FileMetadata   `json:"file-metadata"`
	FilterFiles        []string          `json:"filter_files"`
	Group              string            `json:"group"`
	InputPathListFile  string            `json:"input_path_list"`
	InputTicketsFile   string            `json:"input_ticket_list"`
	InvocationId       string            `json:"invocation_id"`
	IrodsBase          string            `json:"irods_base"`
	Name               string            `json:"name"`
	NfsBase            string            `json:"nfs_base"`
	Notify             bool              `json:"notify"`
	NowDate            string            `json:"now_date"`
	OutputDir          string            `json:"output_dir"`
	OutputDirTicket    string            `json:"output_dir_ticket"`
	OutputTicketFile   string            `json:"output_ticket_file"`
	RequestType        string            `json:"request_type"`
	RunOnNfs           bool              `json:"run-on-nfs"`
	SkipParentMetadata bool              `json:"skip-parent-meta"`
	Steps              []*Step           `json:"steps"`
	SubmissionDate     string            `json:"submission_date"`
	Submitter          string            `json:"username"`
	Type               string            `json:"type"`
	UserId             string            `json:"user_id"`
	UserGroups         []string          `json:"user_groups"`
	UserHome           string            `json:"user_home"`
	WikiUrl            string            `json:"wiki_url"`
	ConfigFile         string            `json:"config_file"`
	MountDataStore     bool              `json:"mount_data_store"`
}

func (x *Job) GetAppDescription() string {
	if x != nil {
		return x.AppDescription
	}
	return ""
}

func (x *Job) GetAppId() string {
	if x != nil {
		return x.AppId
	}
	return ""
}

func (x *Job) GetAppName() string {
	if x != nil {
		return x.AppName
	}
	return ""
}

func (x *Job) GetArchiveLogs() bool {
	if x != nil {
		return x.ArchiveLogs
	}
	return false
}

func (x *Job) GetBatchId() string {
	if x != nil {
		return x.BatchId
	}
	return ""
}

func (x *Job) GetCondorId() string {
	if x != nil {
		return x.CondorId
	}
	return ""
}

func (x *Job) GetCondorLogPath() string {
	if x != nil {
		return x.CondorLogPath
	}
	return ""
}

func (x *Job) GetCreateOutputSubdir() bool {
	if x != nil {
		return x.CreateOutputSubdir
	}
	return false
}

func (x *Job) GetDateSubmitted() *ptypes.Timestamp {
	if x != nil {
		return x.DateSubmitted
	}
	return nil
}

func (x *Job) GetDateStarted() *ptypes.Timestamp {
	if x != nil {
		return x.DateStarted
	}
	return nil
}

func (x *Job) GetDateCompleted() *ptypes.Timestamp {
	if x != nil {
		return x.DateCompleted
	}
	return nil
}

func (x *Job) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Job) GetEmail() string {
	if x != nil {
		return x.Email
	}
	return ""
}

func (x *Job) GetExtra() *Extra {
	if x != nil {
		return x.Extra
	}
	return nil
}

func (x *Job) GetExecutionTarget() string {
	if x != nil {
		return x.ExecutionTarget
	}
	return ""
}

func (x *Job) GetExitCode() int32 {
	if x != nil {
		return x.ExitCode
	}
	return 0
}

func (x *Job) GetFailureCount() ptypes.Int64 {
	if x != nil {
		return x.FailureCount
	}
	return 0
}

func (x *Job) GetFailureThreshold() ptypes.Int64 {
	if x != nil {
		return x.FailureThreshold
	}
	return 0
}

func (x *Job) GetFileMetadata() []*FileMetadata {
	if x != nil {
		return x.FileMetadata
	}
	return nil
}

func (x *Job) GetFilterFiles() []string {
	if x != nil {
		return x.FilterFiles
	}
	return nil
}

func (x *Job) GetGroup() string {
	if x != nil {
		return x.Group
	}
	return ""
}

func (x *Job) GetInputPathListFile() string {
	if x != nil {
		return x.InputPathListFile
	}
	return ""
}

func (x *Job) GetInputTicketsFile() string {
	if x != nil {
		return x.InputTicketsFile
	}
	return ""
}

func (x *Job) GetInvocationId() string {
	if x != nil {
		return x.InvocationId
	}
	return ""
}

func (x *Job) GetIrodsBase() string {
	if x != nil {
		return x.IrodsBase
	}
	return ""
}

func (x *Job) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Job) GetNfsBase() string {
	if x != nil {
		return x.NfsBase
	}
	return ""
}

func (x *Job) GetNotify() bool {
	if x != nil {
		return x.Notify
	}
	return false
}

func (x *Job) GetNowDate() string {
	if x != nil {
		return x.NowDate
	}
	return ""
}

func (x *Job) GetOutputDir() string {
	if x != nil {
		return x.OutputDir
	}
	return ""
}

func (x *Job) GetOutputDirTicket() string {
	if x != nil {
		return x.OutputDirTicket
	}
	return ""
}

func (x *Job) GetOutputTicketFile() string {
	if x != nil {
		return x.OutputTicketFile
	}
	return ""
}

func (x *Job) GetRequestType() string {
	if x != nil {
		return x.RequestType
	}
	return ""
}

func (x *Job) GetRunOnNfs() bool {
	if x != nil {
		return x.RunOnNfs
	}
	return false
}

func (x *Job) GetSkipParentMetadata() bool {
	if x != nil {
		return x.SkipParentMetadata
	}
	return false
}

func (x *Job) GetSteps() []*Step {
	if x != nil {
		return x.Steps
	}
	return nil
}

func (x *Job) GetSubmissionDate() string {
	if x != nil {
		return x.SubmissionDate
	}
	return ""
}

func (x *Job) GetSubmitter() string {
	if x != nil {
		return x.Submitter
	}
	return ""
}

func (x *Job) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *Job) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

func (x *Job) GetUserGroups() []string {
	if x != nil {
		return x.UserGroups
	}
	return nil
}

func (x *Job) GetUserHome() string {
	if x != nil {
		return x.UserHome
	}
	return ""
}

func (x *Job) GetWikiUrl() string {
	if x != nil {
		return x.WikiUrl
	}
	return ""
}

func (x *Job) GetConfigFile() string {
	if x != nil {
		return x.ConfigFile
	}
	return ""
}

func (x *Job) GetMountDataStore() bool {
	if x != nil {
		return x.MountDataStore
	}
	return false
}
