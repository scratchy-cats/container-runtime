use crate::oci::{config::Config, container_state::ContainerState};
use std::path::Path;

struct Container {}

impl Container {
  pub fn create(containerID: String, bundle: String) {
    // Read and parse config.json in the bundle.
    let configJsonFilePath = Path::new(&bundle).join("config.json");
    let config = Config::try_from(configJsonFilePath.as_path())
      .map_err(|error| panic!("Failed parsing config from bundle : {}", error));

    // Construct and persist the container state.
    let containerState = ContainerState::new(containerID, &bundle);
    if let Err(error) = containerState.save() {
      panic!("Failed saving container state : {}", error);
    }

    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn createContainer() {
    Container::create("hello-world".to_string(), "./tests/bundles/hello-world".to_string())
  }
}
