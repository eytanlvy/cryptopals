use std::str;

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
