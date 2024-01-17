use std::path::Path;
use std::{fs, str};

static CHARS: &'static[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/*
	Conversion functions
 */

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

fn u8_from_base64(c: char) -> Result<Option<u8>> {
    match c {
        'A'..='Z' => Ok(Some(c as u8 - b'A')),
        'a'..='z' => Ok(Some(26 + (c as u8 - b'a'))),
        '0'..='9' => Ok(Some(52 + (c as u8 - b'0'))),
        '+' => Ok(Some(62)),
        '/' => Ok(Some(63)),
        '\n' | '=' => Ok(None), // Skip '\n' or '='
        _ => Err(format!("invalid character {}", c).into()),
    }
}


pub fn decode_base64(s: &str) -> Result<Vec<u8>>  {
	if s.len() % 4 != 0 {
        return Err("input length needs to be multiple of 4".into());
    }

    let mut n = s.len();
    if s.as_bytes()[n - 1] == b'=' {
        if s.as_bytes()[n - 2] == b'=' {
            n -= 1;
        }
        n -= 1;
    }

    let mut digits = Vec::with_capacity(n);
    for c in s.chars().take(n) {
        digits.push(u8_from_base64(c)?);
    }
	let digits: Vec<u8> = digits.into_iter().filter_map(|x| x).collect();

    let mut u = Vec::with_capacity(3 * s.len() / 4);
    for b in digits.chunks(4) {
        u.push((b[0] << 2) + (b[1] >> 4));
        if b.len() == 2 {
            if b[1] << 4 != 0 {
                return Err("input not padded with zero".into());
            }
            break;
        }

        u.push((b[1] << 4) + (b[2] >> 2));
        if b.len() == 3 {
            if b[2] << 6 != 0 {
                return Err("input not padded with zero".into());
            }
            break;
        }

        u.push((b[2] << 6) + b[3]);
    }
    Ok(u)
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

pub fn bytes_to_hex(x: &[u8]) -> String {
	return x.iter().map( |y| format!("{:02x}", y) ).collect();
}

pub fn bytes_to_string(x: &[u8]) -> String {
	return str::from_utf8(x).unwrap().to_string();
}

/* 
	XOR functions
 */

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
	assert!(a.len() == b.len());
	return a.iter().zip(b.iter()).map( |(x, y)| x ^ y ).collect();
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

fn score(x: &[u8]) -> i32 {
	let mut score = 0;
	for c in x {
		if *c == b'E' || *c == b'e' || *c == b't' || *c == b'T' || *c == b'A' || *c == b'a' || *c == b'o' || *c == b'O' || *c == b'I' || *c == b'i' || *c == b'N' || *c == b'n' || *c == b' ' || *c == b'S' || *c == b's' || *c == b'H' || *c == b'h' || *c == b'R' || *c == b'r' || *c == b'D' || *c == b'd' || *c == b'L' || *c == b'l' || *c == b'U' || *c == b'u' {
			score += 1;
		}
	}
	return score;
}

pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
	let mut res = Vec::new();
	for i in 0..plaintext.len() {
		res.push(plaintext[i] ^ key[i % key.len()]);
	}
	return res;
}

pub fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> usize {
    assert_eq!(bytes1.len(), bytes2.len());

	let len = bytes1.len();

	let mut distance = 0;
	for i in 0..len {
		let mut x = bytes1[i] ^ bytes2[i];
		while x > 0 {
			distance += 1;
			x &= x - 1;
		}
	}
	distance
}

pub fn find_keysize(ciphertext: &[u8]) -> usize {
    let mut best_keysize = 0;
    let mut best_score = f64::INFINITY;

    for keysize in 2..40 {
        let mut total_score = 0.0;
        let num_blocks = 10;
        for i in 0..num_blocks {
            let block1 = &ciphertext[i * keysize..(i + 1) * keysize];
            for j in (i + 1)..num_blocks {
                let block2 = &ciphertext[j * keysize..(j + 1) * keysize];
                total_score += hamming_distance(block1, block2) as f64 / keysize as f64;
            }
        }
        let score = total_score / num_blocks as f64;
        if score < best_score {
            best_score = score;
            best_keysize = keysize;
        }
    }
    best_keysize
}


pub fn transpose_blocks(ciphertext: &[u8], keysize: usize) -> Vec<Vec<u8>> {
	let mut blocks = Vec::new();
	for _ in 0..keysize {
		blocks.push(Vec::new());
	}
	for i in 0..ciphertext.len() {
		blocks[i % keysize].push(ciphertext[i]);
	}
	return blocks;
}

pub fn find_repeating_key(ciphertext: Vec<Vec<u8>>) -> Vec<u8> {
	let mut key = Vec::new();
	for block in ciphertext {
		let (k, _) = single_bytes_xor_cypher(&block);
		key.push(k);
	}
	return key;
}

pub fn decrypt_repeating_key_xor(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
	let mut res = Vec::new();
	for i in 0..ciphertext.len() {
		res.push(ciphertext[i] ^ key[i % key.len()]);
	}
	return res;
}