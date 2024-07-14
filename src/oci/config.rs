use anyhow::anyhow;
use serde::Deserialize;
use std::{
  fs,
  path::{Path, PathBuf},
};
use strum_macros::EnumString;

#[derive(Deserialize)]
pub struct Config {
  // Version of the Open Container Initiative Runtime Specification with which the bundle complies.
  ociVersion: String,

  // The container's root filesystem.
  root: Root,

  // Additional mounts beyond root. The runtime MUST mount entries in the listed order. For Linux,
  // the parameters are as documented in mount(2) system call man page.
  mounts: Vec<Mount>,

  // The container process.
  process: Option<Process>,

  // Container's hostname as seen by processes running inside the container. On Linux, for example,
  // this will change the hostname in the container UTS namespace. Depending on your namespace
  // configuration, the container UTS namespace may be the runtime UTS namespace.
  hostname: Option<String>,

  linux: Option<Linux>,

  // For configuring custom actions related to the lifecycle of the container.
  hooks: Option<Hooks>,
}

impl TryFrom<&Path> for Config {
  type Error = anyhow::Error;

  fn try_from(configJSONFilePath: &Path) -> Result<Self, Self::Error> {
    let configAsString = fs::read_to_string(configJSONFilePath)
      .map_err(|error| anyhow!("Failed reading config.json file : {:?}", error))?;

    let config: Config = serde_json::from_str(&configAsString)
      .map_err(|error| anyhow!("Failed parsing config.json file : {:?}", error))?;
    Ok(config)
  }
}

#[derive(Deserialize)]
struct Root {
  // Path to the root filesystem for the container. On POSIX platforms, it's either an absolute path
  // or a relative path to the bundle. A directory MUST exist at the path declared by the field.
  path: PathBuf,

  // If true then the root filesystem MUST be read-only inside the container. Defaults to false.
  readonly: Option<bool>,
}

#[derive(Deserialize)]
struct Mount {
  // Destination of mount point: path inside container.
  // Linux: This value SHOULD be an absolute path. For compatibility with old tools and
  // configurations, it MAY be a relative path, in which case it MUST be interpreted as relative to
  // "/". Relative paths are deprecated.
  destination: String,

  // A device name, but can also be a file or directory name for bind mounts or a dummy. Path values
  // for bind mounts are either absolute or relative to the bundle. A mount is a bind mount if it
  // has either bind or rbind in the options.
  source: Option<String>,

  options: Option<Vec<MountOptions>>,
}

#[derive(EnumString, Deserialize)]
enum MountOptions {
  // Bind mount.
  #[strum(serialize = "bind")]
  Bind,

  // Recursive bind mount.
  #[strum(serialize = "rbind")]
  Rbind,
}

#[derive(Deserialize)]
struct Process {
  /*
    Whether a terminal is attached to the process, defaults to false.

    As an example, if set to true on Linux a pseudoterminal pair is allocated for the process and
    the pseudoterminal pty is duplicated on the process's standard streams (stdin(3)).

    In some operating systems, including Unix-like systems, a pseudoterminal, pseudotty, or PTY is a
    pair of pseudo-device endpoints (files) which establish asynchronous, bidirectional
    communication (IPC) channel (with two ports) between two or more processes.
    Read more here : https://en.wikipedia.org/wiki/Pseudoterminal.
  */
  terminal: Option<bool>,

  // Console size in characters of the terminal.
  consoleSize: Option<ConsoleSize>,

  // Working directory that will be set for the executable.
  cwd: String,
  env: Option<Vec<String>>,
  args: Option<Vec<String>>,

  // Allows specific control over which user the process runs as.
  user: User,

  // Allows setting resource limits for the process.
  rlimits: Option<Vec<Rlimits>>,
}

#[derive(Deserialize)]
struct ConsoleSize {
  height: usize,
  width: usize,
}

#[derive(Deserialize)]
struct Rlimits {
  // The platform resource being limited. The runtime MUST generate an error for any values which
  // cannot be mapped to a relevant kernel interface.
  // Linux: valid values are defined in the getrlimit(2) man page.
  r#type: String,

  // Value of the limit enforced for the corresponding resource.
  soft: u64,

  // Ceiling for the soft limit that could be set by an unprivileged process. Only a privileged
  // process can raise a hard limit.
  hard: u64,
}

#[derive(Deserialize)]
struct User {
  // User ID in the container namespace.
  uid: i64,

  // Group ID in the container namespace.
  gid: i64,
  additionalGids: Option<Vec<i64>>,

  // (user file-creation mode mask) is used by UNIX-based systems to set default permissions for
  // newly created files and directories. It does this by masking or subtracting these permissions.
  // Read more here : https://www.liquidweb.com/blog/what-is-umask-and-how-to-use-it-effectively/.
  umask: Option<i64>,
}

#[derive(Deserialize)]
struct Linux {}

#[derive(Deserialize)]
struct Hooks {}
