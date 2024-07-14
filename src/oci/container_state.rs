use super::OCI_VERSION;
use crate::TEMP_DIR;
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
  fs,
  io::Write,
  path::{Path, PathBuf},
};

// NOTE: The state MAY include additional properties.
#[derive(Serialize)]
pub struct ContainerState {
  // Version of the Open Containers Initiative Runtime Specification with which the state complies.
  ociVersion: String,

  // Container's ID. This MUST be unique across all containers on this host. There is no requirement
  // that it be unique across hosts.
  id: String,

  // Runtime state of the container.
  status: Status,

  /*
    ID of the container process. For hooks executed in the runtime namespace, it is the pid as seen
    by the runtime. For hooks executed in the container namespace, it is the pid as seen by the
    container.

    Container Namespace : On Linux, the namespaces in which the configured process executes.

    Linux Namespace : A namespace wraps a global system resource in an abstraction that makes it
    appear to the processes within the namespace that they have their own isolated instance of the
    global resource. There are different types of namespaces like Cgroups / IPC / Network / UTS etc.
  */
  pid: i64,

  /*
    Absolute path to the container's bundle directory. This is provided so that consumers can find
    the container's configuration and root filesystem on the host.

    Filesystem Bundle, is a set of files organized in a certain way, and containing all the necessary
    data and metadata for any compliant runtime to perform all standard operations against it. This
    includes :

    (1) config.json : contains configuration data. This REQUIRED file MUST reside in the root of the
      bundle directory.

    (2) container's root filesystem: the directory referenced by root.path, if that property is set
      in config.json.
  */
  bundle: PathBuf,
}

impl ContainerState {
  pub fn new(id: String, bundle: &str) -> Self {
    Self {
      ociVersion: OCI_VERSION.to_string(),
      id: id.clone(),
      pid: 0,
      status: Status::Creating,
      bundle: Path::new(bundle).canonicalize().expect(&format!(
        "Failed determining absolute filesystem bundle path for container-id = {} and bundle = {}",
        id, bundle
      )),
    }
  }

  pub fn save(&self) -> Result<()> {
    let containerStateDir = Path::new(TEMP_DIR).join(&self.id);
    fs::create_dir_all(containerStateDir.clone()).map_err(|error| {
      anyhow!("Failed creating container state dir {:?} : {}", containerStateDir.clone(), error)
    })?;

    let marshalledContainerState = serde_json::to_string(self).map_err(|error| {
      anyhow!("Failed JSON marshalling state of container {} : {}", self.id, error)
    })?;

    let mut containerStateFile = fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(containerStateDir.join("state.json"))
      .map_err(|error| {
        anyhow!(
          "Failed opening container state file in {:?} : {}",
          containerStateDir.clone(),
          error
        )
      })?;

    containerStateFile
      .write_all(marshalledContainerState.as_bytes())
      .map_err(|error| {
        anyhow!(
          "Failed persisting container state in state.json in {:?} : {}",
          containerStateDir,
          error
        )
      })
  }
}

// NOTE: Additional values MAY be defined by the runtime.
#[derive(Serialize)]
enum Status {
  Creating,

  // The runtime has finished the create operation, and the container process has neither exited nor
  // executed the user-specified program.
  Created,

  // The container process has executed the user-specified program but has not exited.
  Running,

  // The container process has exited.
  Stopped,
}
