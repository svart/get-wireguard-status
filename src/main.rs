use std::collections::HashMap;
use std::process::Command;

use serde::Deserialize;


#[derive(Deserialize)]
struct Interface {
    ifname: String,
    flags: Vec<String>,
    linkinfo: Option<HashMap<String, String>>,
}


fn main() {
    let cmd_result = Command::new("ip")
        .args(["-j", "-d", "addr"])
        .output()
        .expect("failed to get interfaces");

    let iface_list: Vec<Interface> = serde_json::from_slice(&cmd_result.stdout).unwrap();

    let mut connected = Vec::new();

    for iface in iface_list {
        if let Some(linkinfo) = iface.linkinfo {
            match linkinfo.get("info_kind") {
                Some(x) if x == "wireguard" => {
                    if iface.flags.contains(&"UP".to_string()) {
                        connected.push(iface.ifname);
                    }
                }
                None | Some(_) => {}
            }
        }
    }

    if connected.is_empty() {
        println!("ᚷ Disconnected");
    } else {
        println!("🔒 {}", connected.join(" "));
    }
}
