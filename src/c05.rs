#[cfg(test)]
mod test {
	use crate::set01::*;
    #[test]
	fn test_c05() {
		let plaintext = "Burning 'em, if you ain't quick and nimble
		I go crazy when I hear a cymbal";
		let bar = plaintext.as_bytes();
		let key = "ICE";
		let foo = key.as_bytes();

		println!("{:?}", bar);
		println!("{:?}", foo);
	}
}