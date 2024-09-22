#![allow(dead_code)]

use std::error::Error;

use crate::cli::{ssh::SshPass, tty::DynTty};


pub struct ShellOptions {
    shell: Option<String>,
}

pub struct SerialOptions {
    port: String,
    baud: u32,
}

pub struct SshOptions {
    host: String,
    port: u16,
    user: String,
    pass: SshPass,
}

pub enum NodeType {
    SHELL(
        ShellOptions
    ),
    SERIAL(
        SerialOptions
    ),
    SSH(
        SshOptions
    ), // May add more types in the future, like SSH tunnel, etc.
}

pub trait DeviceAbst {
    fn fnode(&mut self) -> Option<&dyn DeviceAbst>;
    fn node_type(&self) -> &NodeType;

    // For this to work, seems we have to define a "must" toolset
    // for each device, or better, provide a way to define a toolset
    fn connect(&mut self) -> Result<DynTty, Box<dyn Error>>;
}

pub struct Device<'a> {
    __fnode: Option<&'a dyn DeviceAbst>,
    __node_type: NodeType,
}

impl<'a> Device<'a> {
    pub fn new(node_type: NodeType, f_node: Option<&'a dyn DeviceAbst>) -> Device<'a> {
        Device {
            __fnode: f_node,
            __node_type: node_type,
        }
    }
}

impl<'a> DeviceAbst for Device<'a> {
    fn fnode(&mut self) -> Option<&dyn DeviceAbst> {
        self.__fnode
    }
    fn node_type(&self) -> &NodeType {
        &self.__node_type
    }
    fn connect(&mut self) -> Result<DynTty, Box<dyn Error>> {
        todo!(); 
        
        // for here, we neet to do recursive call to connect to the device
        // that is, connect from the root node to the this node to build a chain
    }
}
