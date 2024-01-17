#[cfg(test)]
mod tests {
	use serialize::{decode_base64};
	use xor::{hamming_distance, find_repeating_key, transpose_blocks,find_keysize, decrypt_repeating_key_xor};
	use std::path::Path;
	use std::fs;

	#[test]
	fn test_hamming_distance() {
		assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
	}

    #[test]
	fn test_c06() {
		let path = Path::new("data/c06.txt");
		let content = fs::read_to_string(path)
        	.expect("Should have been able to read the file");
		let decoded_content = decode_base64(&content).unwrap();
		let keysize = find_keysize(&decoded_content);
		let key = find_repeating_key(transpose_blocks(&decoded_content, keysize));
		assert_eq!(key, b"Terminator X: Bring the noise");

		let res = decrypt_repeating_key_xor(&decoded_content, &key);
		assert_eq!(&res[0..33], b"I'm back and I'm ringin' the bell");
	}
}