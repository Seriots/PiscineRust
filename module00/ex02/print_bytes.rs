fn print_bytes(s: &str) {
	let bytes = s.bytes();
	for elem in bytes {
		println!("{}", elem);

	}
}

fn main() {
	print_bytes("Déjà Vu\n");
}