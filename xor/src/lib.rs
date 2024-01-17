use std::path::Path;
use std::fs;
use serialize::hex_to_bytes;

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