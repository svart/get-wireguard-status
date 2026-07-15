use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
struct Interface {
    ifname: String,
    flags: Vec<String>,
    linkinfo: Option<LinkInfo>,
}

#[derive(Deserialize)]
struct LinkInfo {
    info_kind: String,
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
            if linkinfo.info_kind == "wireguard" && iface.flags.iter().any(|flag| flag == "UP") {
                connected.push(iface.ifname);
            }
        }
    }

    if connected.is_empty() {
        println!("ᚷ Disconnected");
    } else {
        println!("🔒 {}", connected.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::Interface;

    #[test]
    fn parses_linkinfo_with_nested_info_data() {
        let input = r#"[{"ifname":"wg0","flags":["POINTOPOINT","UP"],"linkinfo":{"info_kind":"wireguard","info_data":{}}}]"#;

        let interfaces: Vec<Interface> = serde_json::from_str(input).unwrap();

        assert_eq!(
            interfaces[0]
                .linkinfo
                .as_ref()
                .map(|linkinfo| linkinfo.info_kind.as_str()),
            Some("wireguard")
        );
    }
}
