use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessEventColumns<'a> {
    /// "uid": "0",
    pub uid: &'a str,

    /// "time": "1527895541",
    pub time: &'a str,

    /// "pid": "30219",
    pub pid: &'a str,

    /// "path": "/usr/bin/curl",
    pub path: &'a str,

    /// "auid": "1000",
    pub auid: &'a str,

    /// "cmdline": "curl google.com",
    pub cmdline: &'a str,

    /// "ctime": "1503452096",
    pub ctime: &'a str,

    /// "cwd": "",
    pub cwd: &'a str,

    /// "egid": "0",
    pub egid: &'a str,

    /// "euid": "0",
    pub euid: &'a str,

    /// "gid": "0",
    pub gid: &'a str,

    /// "parent": ""
    pub parent: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessEvent<'a> {
    ///   "action": "added" OR "removed",
    pub action: &'a str,

    /// "unixTime": 1527895550,
    #[serde(alias = "unixTime")]
    pub unix_time: u64,

    /// "columns": {//see ProcessEventColumns},
    pub columns: ProcessEventColumns<'a>,

    /// "hostIdentifier": "vagrant",
    #[serde(alias = "hostIdentifier")]
    pub host_identifier: &'a str,

    #[serde(borrow)]
    pub decorations: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Serialize, Deserialize)]
pub struct SocketEventColumns<'a> {
    pub time: &'a str,
    pub success: &'a str,
    pub remote_port: &'a str,
    pub action: &'a str,
    pub auid: &'a str,
    pub family: &'a str,
    pub local_address: &'a str,
    pub local_port: &'a str,
    pub path: &'a str,
    pub pid: &'a str,
    pub remote_address: &'a str,
    #[serde(borrow)]
    pub decorations: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Serialize, Deserialize)]
pub struct SocketEvent<'a> {
    pub columns: SocketEventColumns<'a>,

    #[serde(alias = "unixTime")]
    pub unix_time: u64,
    #[serde(alias = "hostIdentifier")]
    pub host_identifier: &'a str,
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Event<'a> {
    #[serde(rename = "process_events")]
    #[serde(borrow)]
    ProcessEvent(ProcessEvent<'a>),

    #[serde(rename = "socket_events")]
    #[serde(borrow)]
    SocketEvent(SocketEvent<'a>),
}

#[cfg(test)]
mod tests {
    use super::{SocketEvent, Event};
    use serde_json::json;

    #[test]
    fn it_works() {
        let log = json!(
            {
              "action": "added",
              "columns": {
                "time": "1527895541",
                "success": "1",
                "remote_port": "80",
                "action": "connect",
                "auid": "1000",
                "family": "2",
                "local_address": "",
                "local_port": "0",
                "path": "/usr/bin/curl",
                "pid": "30220",
                "remote_address": "172.217.164.110"
              },
              "unixTime": 1527895545,
              "hostIdentifier": "vagrant",
              "name": "socket_events"
            }
        ).to_string();

        let _: Event = serde_json::from_str(&log)
            .expect("Could not deserialize SocketEvent");


    }


}
