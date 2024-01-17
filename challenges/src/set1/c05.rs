#[cfg(test)]
mod tests {
	use xor::repeating_key_xor;
    #[test]
	fn test_c05() {
		let plaintext = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
		let key: &[u8] = b"ICE";
		println!("{:?}", key);


		let res = repeating_key_xor(plaintext, key);

		assert_eq!(hex::encode(res), "0b3637272a2b2e63622c2e69692a23693a2a3c632\
		4202d623d63343c2a26226324272765272a282b2f2\
		0430a652e2c652a3124333a653e2b2027630c692b2\
		0283165286326302e27282f");
	}
}