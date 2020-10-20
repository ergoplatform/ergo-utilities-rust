/// This file holds functions related to saving/accessing local data
/// related to interacting with an ergo node. (Ip/Port/Api Key)
use crate::node_interface::{NodeError, NodeInterface, Result};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

/// The basic yaml of a `node_interface.config`.
fn barebones_yaml() -> Yaml {
    let s = r#"
        # IP Address of the node (default is local, edit if yours is different)
        node_ip: "0.0.0.0"
        # Port that the node is on (default is 9053, edit if yours is different)
        node_port: "9053"
        # API key for the node (edit if yours is different)
        node_api_key: "hello"
    "#;

    YamlLoader::load_from_str(&s).unwrap()[0].clone()
}

/// Uses the config yaml provided to create a new `NodeInterface`
pub fn new_interface_from_yaml(config: Yaml) -> Result<NodeInterface> {
    let ip = config["node_ip"].as_str().ok_or(NodeError::YamlError(
        "`node_ip` is not specified in the provided Yaml".to_string(),
    ))?;
    let port = config["node_port"].as_str().ok_or(NodeError::YamlError(
        "`node_port` is not specified in the provided Yaml".to_string(),
    ))?;
    let api_key = config["node_api_key"].as_str().ok_or(NodeError::YamlError(
        "`node_api_key` is not specified in the provided Yaml".to_string(),
    ))?;
    Ok(NodeInterface::new(api_key, ip, port))
}

/// Opens a local `node_interface.config` file and uses the
/// data inside to create a `NodeInterface`
pub fn new_interface_from_local_config() -> Result<NodeInterface> {
    todo!()
}
