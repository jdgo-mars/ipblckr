use std::process::Command;

pub fn add_2_iptables(ip: &str) {
    let s = &format!("-s {}", ip)[..];
    Command::new("iptables")
        .arg("-A INPUT")
        .arg(s)
        .arg("-j DROP")
        .output()
        .expect("failed to execute iptables command");

    Command::new("iptables-save > /etc/iptables/rules.v4")
        .output()
        .expect("failed to save iptables config");
}
