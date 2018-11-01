/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PolicyV1beta1PodSecurityPolicySpec : PodSecurityPolicySpec defines the policy enforced.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyV1beta1PodSecurityPolicySpec {
  /// allowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.
  #[serde(rename = "allowPrivilegeEscalation")]
  allow_privilege_escalation: Option<bool>,
  /// allowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both allowedCapabilities and requiredDropCapabilities.
  #[serde(rename = "allowedCapabilities")]
  allowed_capabilities: Option<Vec<String>>,
  /// allowedFlexVolumes is a whitelist of allowed Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the \"volumes\" field.
  #[serde(rename = "allowedFlexVolumes")]
  allowed_flex_volumes: Option<Vec<::models::PolicyV1beta1AllowedFlexVolume>>,
  /// allowedHostPaths is a white list of allowed host paths. Empty indicates that all host paths may be used.
  #[serde(rename = "allowedHostPaths")]
  allowed_host_paths: Option<Vec<::models::PolicyV1beta1AllowedHostPath>>,
  /// AllowedProcMountTypes is a whitelist of allowed ProcMountTypes. Empty or nil indicates that only the DefaultProcMountType may be used. This requires the ProcMountType feature flag to be enabled.
  #[serde(rename = "allowedProcMountTypes")]
  allowed_proc_mount_types: Option<Vec<String>>,
  /// allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to whitelist all allowed unsafe sysctls explicitly to avoid rejection.  Examples: e.g. \"foo/_*\" allows \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" allows \"foo.bar\", \"foo.baz\", etc.
  #[serde(rename = "allowedUnsafeSysctls")]
  allowed_unsafe_sysctls: Option<Vec<String>>,
  /// defaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capability in both defaultAddCapabilities and requiredDropCapabilities. Capabilities added here are implicitly allowed, and need not be included in the allowedCapabilities list.
  #[serde(rename = "defaultAddCapabilities")]
  default_add_capabilities: Option<Vec<String>>,
  /// defaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process.
  #[serde(rename = "defaultAllowPrivilegeEscalation")]
  default_allow_privilege_escalation: Option<bool>,
  /// forbiddenSysctls is a list of explicitly forbidden sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of forbidden sysctls. Single * means all sysctls are forbidden.  Examples: e.g. \"foo/_*\" forbids \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" forbids \"foo.bar\", \"foo.baz\", etc.
  #[serde(rename = "forbiddenSysctls")]
  forbidden_sysctls: Option<Vec<String>>,
  /// fsGroup is the strategy that will dictate what fs group is used by the SecurityContext.
  #[serde(rename = "fsGroup")]
  fs_group: ::models::PolicyV1beta1FsGroupStrategyOptions,
  /// hostIPC determines if the policy allows the use of HostIPC in the pod spec.
  #[serde(rename = "hostIPC")]
  host_ipc: Option<bool>,
  /// hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
  #[serde(rename = "hostNetwork")]
  host_network: Option<bool>,
  /// hostPID determines if the policy allows the use of HostPID in the pod spec.
  #[serde(rename = "hostPID")]
  host_pid: Option<bool>,
  /// hostPorts determines which host port ranges are allowed to be exposed.
  #[serde(rename = "hostPorts")]
  host_ports: Option<Vec<::models::PolicyV1beta1HostPortRange>>,
  /// privileged determines if a pod can request to be run as privileged.
  #[serde(rename = "privileged")]
  privileged: Option<bool>,
  /// readOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.
  #[serde(rename = "readOnlyRootFilesystem")]
  read_only_root_filesystem: Option<bool>,
  /// requiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.
  #[serde(rename = "requiredDropCapabilities")]
  required_drop_capabilities: Option<Vec<String>>,
  /// runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.
  #[serde(rename = "runAsUser")]
  run_as_user: ::models::PolicyV1beta1RunAsUserStrategyOptions,
  /// seLinux is the strategy that will dictate the allowable labels that may be set.
  #[serde(rename = "seLinux")]
  se_linux: ::models::PolicyV1beta1SeLinuxStrategyOptions,
  /// supplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
  #[serde(rename = "supplementalGroups")]
  supplemental_groups: ::models::PolicyV1beta1SupplementalGroupsStrategyOptions,
  /// volumes is a white list of allowed volume plugins. Empty indicates that no volumes may be used. To allow all volumes you may use '*'.
  #[serde(rename = "volumes")]
  volumes: Option<Vec<String>>
}

impl PolicyV1beta1PodSecurityPolicySpec {
  /// PodSecurityPolicySpec defines the policy enforced.
  pub fn new(fs_group: ::models::PolicyV1beta1FsGroupStrategyOptions, run_as_user: ::models::PolicyV1beta1RunAsUserStrategyOptions, se_linux: ::models::PolicyV1beta1SeLinuxStrategyOptions, supplemental_groups: ::models::PolicyV1beta1SupplementalGroupsStrategyOptions) -> PolicyV1beta1PodSecurityPolicySpec {
    PolicyV1beta1PodSecurityPolicySpec {
      allow_privilege_escalation: None,
      allowed_capabilities: None,
      allowed_flex_volumes: None,
      allowed_host_paths: None,
      allowed_proc_mount_types: None,
      allowed_unsafe_sysctls: None,
      default_add_capabilities: None,
      default_allow_privilege_escalation: None,
      forbidden_sysctls: None,
      fs_group: fs_group,
      host_ipc: None,
      host_network: None,
      host_pid: None,
      host_ports: None,
      privileged: None,
      read_only_root_filesystem: None,
      required_drop_capabilities: None,
      run_as_user: run_as_user,
      se_linux: se_linux,
      supplemental_groups: supplemental_groups,
      volumes: None
    }
  }

  pub fn set_allow_privilege_escalation(&mut self, allow_privilege_escalation: bool) {
    self.allow_privilege_escalation = Some(allow_privilege_escalation);
  }

  pub fn with_allow_privilege_escalation(mut self, allow_privilege_escalation: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.allow_privilege_escalation = Some(allow_privilege_escalation);
    self
  }

  pub fn allow_privilege_escalation(&self) -> Option<&bool> {
    self.allow_privilege_escalation.as_ref()
  }

  pub fn reset_allow_privilege_escalation(&mut self) {
    self.allow_privilege_escalation = None;
  }

  pub fn set_allowed_capabilities(&mut self, allowed_capabilities: Vec<String>) {
    self.allowed_capabilities = Some(allowed_capabilities);
  }

  pub fn with_allowed_capabilities(mut self, allowed_capabilities: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.allowed_capabilities = Some(allowed_capabilities);
    self
  }

  pub fn allowed_capabilities(&self) -> Option<&Vec<String>> {
    self.allowed_capabilities.as_ref()
  }

  pub fn reset_allowed_capabilities(&mut self) {
    self.allowed_capabilities = None;
  }

  pub fn set_allowed_flex_volumes(&mut self, allowed_flex_volumes: Vec<::models::PolicyV1beta1AllowedFlexVolume>) {
    self.allowed_flex_volumes = Some(allowed_flex_volumes);
  }

  pub fn with_allowed_flex_volumes(mut self, allowed_flex_volumes: Vec<::models::PolicyV1beta1AllowedFlexVolume>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.allowed_flex_volumes = Some(allowed_flex_volumes);
    self
  }

  pub fn allowed_flex_volumes(&self) -> Option<&Vec<::models::PolicyV1beta1AllowedFlexVolume>> {
    self.allowed_flex_volumes.as_ref()
  }

  pub fn reset_allowed_flex_volumes(&mut self) {
    self.allowed_flex_volumes = None;
  }

  pub fn set_allowed_host_paths(&mut self, allowed_host_paths: Vec<::models::PolicyV1beta1AllowedHostPath>) {
    self.allowed_host_paths = Some(allowed_host_paths);
  }

  pub fn with_allowed_host_paths(mut self, allowed_host_paths: Vec<::models::PolicyV1beta1AllowedHostPath>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.allowed_host_paths = Some(allowed_host_paths);
    self
  }

  pub fn allowed_host_paths(&self) -> Option<&Vec<::models::PolicyV1beta1AllowedHostPath>> {
    self.allowed_host_paths.as_ref()
  }

  pub fn reset_allowed_host_paths(&mut self) {
    self.allowed_host_paths = None;
  }

  pub fn set_allowed_proc_mount_types(&mut self, allowed_proc_mount_types: Vec<String>) {
    self.allowed_proc_mount_types = Some(allowed_proc_mount_types);
  }

  pub fn with_allowed_proc_mount_types(mut self, allowed_proc_mount_types: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.allowed_proc_mount_types = Some(allowed_proc_mount_types);
    self
  }

  pub fn allowed_proc_mount_types(&self) -> Option<&Vec<String>> {
    self.allowed_proc_mount_types.as_ref()
  }

  pub fn reset_allowed_proc_mount_types(&mut self) {
    self.allowed_proc_mount_types = None;
  }

  pub fn set_allowed_unsafe_sysctls(&mut self, allowed_unsafe_sysctls: Vec<String>) {
    self.allowed_unsafe_sysctls = Some(allowed_unsafe_sysctls);
  }

  pub fn with_allowed_unsafe_sysctls(mut self, allowed_unsafe_sysctls: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.allowed_unsafe_sysctls = Some(allowed_unsafe_sysctls);
    self
  }

  pub fn allowed_unsafe_sysctls(&self) -> Option<&Vec<String>> {
    self.allowed_unsafe_sysctls.as_ref()
  }

  pub fn reset_allowed_unsafe_sysctls(&mut self) {
    self.allowed_unsafe_sysctls = None;
  }

  pub fn set_default_add_capabilities(&mut self, default_add_capabilities: Vec<String>) {
    self.default_add_capabilities = Some(default_add_capabilities);
  }

  pub fn with_default_add_capabilities(mut self, default_add_capabilities: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.default_add_capabilities = Some(default_add_capabilities);
    self
  }

  pub fn default_add_capabilities(&self) -> Option<&Vec<String>> {
    self.default_add_capabilities.as_ref()
  }

  pub fn reset_default_add_capabilities(&mut self) {
    self.default_add_capabilities = None;
  }

  pub fn set_default_allow_privilege_escalation(&mut self, default_allow_privilege_escalation: bool) {
    self.default_allow_privilege_escalation = Some(default_allow_privilege_escalation);
  }

  pub fn with_default_allow_privilege_escalation(mut self, default_allow_privilege_escalation: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.default_allow_privilege_escalation = Some(default_allow_privilege_escalation);
    self
  }

  pub fn default_allow_privilege_escalation(&self) -> Option<&bool> {
    self.default_allow_privilege_escalation.as_ref()
  }

  pub fn reset_default_allow_privilege_escalation(&mut self) {
    self.default_allow_privilege_escalation = None;
  }

  pub fn set_forbidden_sysctls(&mut self, forbidden_sysctls: Vec<String>) {
    self.forbidden_sysctls = Some(forbidden_sysctls);
  }

  pub fn with_forbidden_sysctls(mut self, forbidden_sysctls: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.forbidden_sysctls = Some(forbidden_sysctls);
    self
  }

  pub fn forbidden_sysctls(&self) -> Option<&Vec<String>> {
    self.forbidden_sysctls.as_ref()
  }

  pub fn reset_forbidden_sysctls(&mut self) {
    self.forbidden_sysctls = None;
  }

  pub fn set_fs_group(&mut self, fs_group: ::models::PolicyV1beta1FsGroupStrategyOptions) {
    self.fs_group = fs_group;
  }

  pub fn with_fs_group(mut self, fs_group: ::models::PolicyV1beta1FsGroupStrategyOptions) -> PolicyV1beta1PodSecurityPolicySpec {
    self.fs_group = fs_group;
    self
  }

  pub fn fs_group(&self) -> &::models::PolicyV1beta1FsGroupStrategyOptions {
    &self.fs_group
  }


  pub fn set_host_ipc(&mut self, host_ipc: bool) {
    self.host_ipc = Some(host_ipc);
  }

  pub fn with_host_ipc(mut self, host_ipc: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.host_ipc = Some(host_ipc);
    self
  }

  pub fn host_ipc(&self) -> Option<&bool> {
    self.host_ipc.as_ref()
  }

  pub fn reset_host_ipc(&mut self) {
    self.host_ipc = None;
  }

  pub fn set_host_network(&mut self, host_network: bool) {
    self.host_network = Some(host_network);
  }

  pub fn with_host_network(mut self, host_network: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.host_network = Some(host_network);
    self
  }

  pub fn host_network(&self) -> Option<&bool> {
    self.host_network.as_ref()
  }

  pub fn reset_host_network(&mut self) {
    self.host_network = None;
  }

  pub fn set_host_pid(&mut self, host_pid: bool) {
    self.host_pid = Some(host_pid);
  }

  pub fn with_host_pid(mut self, host_pid: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.host_pid = Some(host_pid);
    self
  }

  pub fn host_pid(&self) -> Option<&bool> {
    self.host_pid.as_ref()
  }

  pub fn reset_host_pid(&mut self) {
    self.host_pid = None;
  }

  pub fn set_host_ports(&mut self, host_ports: Vec<::models::PolicyV1beta1HostPortRange>) {
    self.host_ports = Some(host_ports);
  }

  pub fn with_host_ports(mut self, host_ports: Vec<::models::PolicyV1beta1HostPortRange>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.host_ports = Some(host_ports);
    self
  }

  pub fn host_ports(&self) -> Option<&Vec<::models::PolicyV1beta1HostPortRange>> {
    self.host_ports.as_ref()
  }

  pub fn reset_host_ports(&mut self) {
    self.host_ports = None;
  }

  pub fn set_privileged(&mut self, privileged: bool) {
    self.privileged = Some(privileged);
  }

  pub fn with_privileged(mut self, privileged: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.privileged = Some(privileged);
    self
  }

  pub fn privileged(&self) -> Option<&bool> {
    self.privileged.as_ref()
  }

  pub fn reset_privileged(&mut self) {
    self.privileged = None;
  }

  pub fn set_read_only_root_filesystem(&mut self, read_only_root_filesystem: bool) {
    self.read_only_root_filesystem = Some(read_only_root_filesystem);
  }

  pub fn with_read_only_root_filesystem(mut self, read_only_root_filesystem: bool) -> PolicyV1beta1PodSecurityPolicySpec {
    self.read_only_root_filesystem = Some(read_only_root_filesystem);
    self
  }

  pub fn read_only_root_filesystem(&self) -> Option<&bool> {
    self.read_only_root_filesystem.as_ref()
  }

  pub fn reset_read_only_root_filesystem(&mut self) {
    self.read_only_root_filesystem = None;
  }

  pub fn set_required_drop_capabilities(&mut self, required_drop_capabilities: Vec<String>) {
    self.required_drop_capabilities = Some(required_drop_capabilities);
  }

  pub fn with_required_drop_capabilities(mut self, required_drop_capabilities: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.required_drop_capabilities = Some(required_drop_capabilities);
    self
  }

  pub fn required_drop_capabilities(&self) -> Option<&Vec<String>> {
    self.required_drop_capabilities.as_ref()
  }

  pub fn reset_required_drop_capabilities(&mut self) {
    self.required_drop_capabilities = None;
  }

  pub fn set_run_as_user(&mut self, run_as_user: ::models::PolicyV1beta1RunAsUserStrategyOptions) {
    self.run_as_user = run_as_user;
  }

  pub fn with_run_as_user(mut self, run_as_user: ::models::PolicyV1beta1RunAsUserStrategyOptions) -> PolicyV1beta1PodSecurityPolicySpec {
    self.run_as_user = run_as_user;
    self
  }

  pub fn run_as_user(&self) -> &::models::PolicyV1beta1RunAsUserStrategyOptions {
    &self.run_as_user
  }


  pub fn set_se_linux(&mut self, se_linux: ::models::PolicyV1beta1SeLinuxStrategyOptions) {
    self.se_linux = se_linux;
  }

  pub fn with_se_linux(mut self, se_linux: ::models::PolicyV1beta1SeLinuxStrategyOptions) -> PolicyV1beta1PodSecurityPolicySpec {
    self.se_linux = se_linux;
    self
  }

  pub fn se_linux(&self) -> &::models::PolicyV1beta1SeLinuxStrategyOptions {
    &self.se_linux
  }


  pub fn set_supplemental_groups(&mut self, supplemental_groups: ::models::PolicyV1beta1SupplementalGroupsStrategyOptions) {
    self.supplemental_groups = supplemental_groups;
  }

  pub fn with_supplemental_groups(mut self, supplemental_groups: ::models::PolicyV1beta1SupplementalGroupsStrategyOptions) -> PolicyV1beta1PodSecurityPolicySpec {
    self.supplemental_groups = supplemental_groups;
    self
  }

  pub fn supplemental_groups(&self) -> &::models::PolicyV1beta1SupplementalGroupsStrategyOptions {
    &self.supplemental_groups
  }


  pub fn set_volumes(&mut self, volumes: Vec<String>) {
    self.volumes = Some(volumes);
  }

  pub fn with_volumes(mut self, volumes: Vec<String>) -> PolicyV1beta1PodSecurityPolicySpec {
    self.volumes = Some(volumes);
    self
  }

  pub fn volumes(&self) -> Option<&Vec<String>> {
    self.volumes.as_ref()
  }

  pub fn reset_volumes(&mut self) {
    self.volumes = None;
  }

}



