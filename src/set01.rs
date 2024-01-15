use std::path::Path;
use std::fs;

static CHARS: &'static[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64(x: &[u8]) -> String {
	let mut s = Vec::with_capacity(x.len() * 4 / 3);
	for b in x.chunks(3) {
        let y = match b.len() {
            3 => ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32),
            2 => ((b[0] as u32) << 10) | ((b[1] as u32) << 2),
            1 => (b[0] as u32) << 4,
            _ => panic!("invalid chunk size"),
        };
		for i in 0..b.len()+1 {
			s.push(CHARS[((y >> (6 * (3 - i))) & 0x3F) as usize]);
		}
		for _ in 0..3-b.len() {
			s.push(b'=');
		}
    }
	return String::from_utf8(s).unwrap();
}

pub fn hex_to_bytes(x: &str) -> Vec<u8> {
	assert!(x.len() % 2 == 0);
	pub fn convert(x: u8) -> u8 {
		match x {
			b'0'..=b'9' => x - b'0',
			b'a'..=b'f' => x - b'a' + 10,
			b'A'..=b'F' => x - b'A' + 10,
			_ => panic!("Invalid hex character: {}", x),
		}
	}
	return x.as_bytes().chunks(2).map( |y| { convert(y[0]) << 4 | convert(y[1]) }).collect();
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
	assert!(a.len() == b.len());
	return a.iter().zip(b.iter()).map( |(x, y)| x ^ y ).collect();
}

pub fn bytes_to_hex(x: &[u8]) -> String {
	return x.iter().map( |y| format!("{:02x}", y) ).collect();
}

pub fn single_bytes_xor_cypher(x: &[u8]) -> (u8, Vec<u8>) {
	let mut best_score = 0;
	let mut best_key  = 0;
	let mut best_res = Vec::new();
	for i in 0..255 {
		let res = xor(x, &[i; 1].repeat(x.len()));
		let score = score(&res);
		if score > best_score {
			best_score = score;
			best_key = i;
			best_res = res;
		}
	}
	return (best_key as u8, best_res);
}

pub fn detect_sc_xor(file_path: &Path) -> (u8, Vec<u8>) {
	let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
	let mut best_score = 0;
	let mut best_key  = 0;
	let mut best_res = Vec::new();
	for line in contents.lines() {
		let (key, res) = single_bytes_xor_cypher(&hex_to_bytes(line));
		let score = score(&res);
		if score > best_score {
			best_score = score;
			best_key = key;
			best_res = res;
		}
	}
	return (best_key, best_res);
}

pub fn score(x: &[u8]) -> i32 {
	let mut score = 0;
	for c in x {
		if *c == b'E' || *c == b'e' || *c == b't' || *c == b'T' || *c == b'A' || *c == b'a' || *c == b'o' || *c == b'O' || *c == b'I' || *c == b'i' || *c == b'N' || *c == b'n' || *c == b' ' || *c == b'S' || *c == b's' || *c == b'H' || *c == b'h' || *c == b'R' || *c == b'r' || *c == b'D' || *c == b'd' || *c == b'L' || *c == b'l' || *c == b'U' || *c == b'u' {
			score += 1;
		}
	}
	return score;
}

pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> { //to test
	let mut res = Vec::new();
	for i in 0..plaintext.len() {
		res.push(plaintext[i] ^ key[i % key.len()]);
	}
	return res;
}