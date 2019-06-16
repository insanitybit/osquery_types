use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ProcessEventColumns<'a> {
    /// "uid": "0",
    uid: &'a str,

    /// "time": "1527895541",
    time: &'a str,

    /// "pid": "30219",
    pid: &'a str,

    /// "path": "/usr/bin/curl",
    path: &'a str,

    /// "auid": "1000",
    auid: &'a str,

    /// "cmdline": "curl google.com",
    cmdline: &'a str,

    /// "ctime": "1503452096",
    ctime: &'a str,

    /// "cwd": "",
    cwd: &'a str,

    /// "egid": "0",
    egid: &'a str,

    /// "euid": "0",
    euid: &'a str,

    /// "gid": "0",
    gid: &'a str,

    /// "parent": ""
    parent: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ProcessEvent<'a> {
    ///   "action": "added" OR "removed",
    action: &'a str,

    /// "unixTime": 1527895550,
    #[serde(alias = "unixTime")]
    unix_time: u64,

    /// "columns": {//see ProcessEventColumns},
    columns: ProcessEventColumns<'a>,

    /// "hostIdentifier": "vagrant",
    #[serde(alias = "hostIdentifier")]
    host_identifier: &'a str,

    #[serde(borrow)]
    decorations: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Serialize, Deserialize)]
struct SocketEventColumns<'a> {
    time: &'a str,
    success: &'a str,
    remote_port: &'a str,
    action: &'a str,
    auid: &'a str,
    family: &'a str,
    local_address: &'a str,
    local_port: &'a str,
    path: &'a str,
    pid: &'a str,
    remote_address: &'a str,
    #[serde(borrow)]
    decorations: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Serialize, Deserialize)]
struct SocketEvent<'a> {
    columns: SocketEventColumns<'a>,

    #[serde(alias = "unixTime")]
    unix_time: u64,
    #[serde(alias = "hostIdentifier")]
    host_identifier: &'a str,
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "name")]
enum Event<'a> {
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
