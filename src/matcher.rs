use regex::bytes::{Match, Regex};

pub fn get_ip(line: &Vec<u8>) -> Option<String> {
    let check_line = Regex::new(r"(?m)^[^\r\n]*(Disconnected from invalid user)[^\r\n]*$").unwrap();
    let ipv4_re = Regex::new(r"([0-9]{1,3}(\.[0-9]{1,3}){3})").unwrap();

    if !check_line.is_match(line) {
        return None;
    }

    let ip: Match = ipv4_re.captures(line).unwrap().get(0).unwrap();
    Option::from(String::from_utf8(ip.as_bytes().to_vec()).unwrap())
}

#[test]
fn it_returns_ip() {
    let line = "Mar  3 12:24:23 vmi537036 sshd[12035]: Disconnected from invalid user root1 144.91.78.208 port 44936 [preauth]".as_bytes().to_vec();
    assert_eq!("144.91.78.208".to_owned(), get_ip(&line).unwrap());
}

#[test]
fn no_match() {
    let line =
        "Mar  3 12:28:35 vmi537036 sshd[12037]: Invalid user root2 from 144.91.78.208 port 59980"
            .as_bytes()
            .to_vec();
    assert_eq!(None, get_ip(&line));
}
