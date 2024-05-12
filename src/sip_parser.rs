use super::text_reader::TextReader;
use std::collections::HashMap;

pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn to_string(&self) -> String {
        let mut s = format!("{} {} {}\n", self.method, self.path, self.version);
        for (key, value) in &self.headers {
            s.push_str(&format!("{}: {}\n", key, value));
        }
        s.push_str(&self.body);
        s
    }
}

pub struct SipParser<'a> {
    reader: TextReader<'a>,
}

impl<'a> SipParser<'a> {
    pub fn new(reader: TextReader) -> SipParser {
        SipParser { reader }
    }

    pub fn parse_request(&mut self) -> Request {
        self.consume_whitespace();
        let method = self.consume_token();
        self.consume_whitespace();
        let uri = self.consume_token();
        self.consume_whitespace();
        let version = self.consume_token();
        self.consume_whitespace();

        let headers = self.parse_headers();

        self.consume_whitespace();

        let body = self.parse_body();

        Request {
            method,
            path: uri,
            version,
            headers,
            body,
        }
    }

    fn parse_headers(&mut self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        loop {
            let line = self.reader.read_line();
            if line.trim().is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap().trim().to_string();
            headers.insert(key, value);
        }
        headers
    }

    fn parse_body(&mut self) -> String {
        let body = self.reader.read_to_end();
        body
    }

    fn consume_whitespace(&mut self) {
        loop {
            let ch = self.reader.peek();
            if ch == -1 || !(ch as u8).is_ascii_whitespace() {
                break;
            }
            self.reader.read();
        }
    }

    fn consume_token(&mut self) -> String {
        let mut token = String::new();
        loop {
            let cp = self.reader.peek();
            if cp == -1 {
                break;
            }
            match char::from_u32(cp as u32) {
                Some(ch) => {
                    if ch.is_whitespace() {
                        break;
                    }
                    token.push(ch);
                    self.reader.read();
                }
                None => {
                    break;
                }
            }
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::SipParser;
    use super::TextReader;
    use std::io::BufReader;

    #[test]
    fn parse_basic_invite() {
        let text = r#"INVITE sip:+14155552222@example.pstn.twilio.com SIP/2.0
        Via: SIP/2.0/UDP 192.168.10.10:5060;branch=z9hG4bK776asdhds
        Max-Forwards: 70
        To: "Bob" <sip:+14155552222@example.pstn.twilio.com>
        From: "Alice" <sip:+14155551111@example.pstn.twilio.com>;tag=1
        Call-ID: a84b4c76e66710
        CSeq: 1 INVITE
        Contact: "Alice" <sip:+14155551111@192.168.10.10:5060>
        Diversion: "Sales" <sip:+14155550000@example.pstn.twilio.com>
        P-Asserted-Identity: "Alice" <sip:+14155551111@example.pstn.twilio.com>
        Content-Length: 0
         
       "#;
        let reader = TextReader::new(BufReader::new(text.as_bytes()));
        let mut parser = SipParser::new(reader);
        let request = parser.parse_request();
        assert_eq!(request.method, "INVITE");
        assert_eq!(request.path, "sip:+14155552222@example.pstn.twilio.com");
        assert_eq!(request.version, "SIP/2.0");
        assert_eq!(request.headers["Via"], "SIP/2.0/UDP 192.168.10.10:5060;branch=z9hG4bK776asdhds");
        assert_eq!(request.headers["Max-Forwards"], "70");
        assert_eq!(request.headers["To"], r#""Bob" <sip:+14155552222@example.pstn.twilio.com>"#);
        assert_eq!(request.headers["From"], r#""Alice" <sip:+14155551111@example.pstn.twilio.com>;tag=1"#);
        assert_eq!(request.headers["Call-ID"], "a84b4c76e66710");
        assert_eq!(request.headers["CSeq"], "1 INVITE");
        assert_eq!(request.headers["Contact"], r#""Alice" <sip:+14155551111@192.168.10.10:5060>"#);
        assert_eq!(request.headers["Diversion"], r#""Sales" <sip:+14155550000@example.pstn.twilio.com>"#);
        assert_eq!(request.headers["P-Asserted-Identity"], r#""Alice" <sip:+14155551111@example.pstn.twilio.com>"#);
        assert_eq!(request.headers["Content-Length"], "0");
        assert_eq!(request.body, "");
    }
}
