use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Read given file and return its content as byte vector
pub fn read_file(name : &str) -> Vec<u8> {
	let mut file = match File::open(Path::new(name)) {
		Err(why) => panic!("could not open {}: {}", name, Error::description(&why)),
		Ok(file) => file,
	};
	let mut content = Vec::new();
	return match file.read_to_end(&mut content) {
		Err(why) => panic!("could not read {}: {}", name, Error::description(&why)),
		Ok(_) => content,
	};
}

// Write given content to given file
pub fn write_file(content : &[u8], name : &str) {
	let mut file = match File::create(Path::new(name)) {
		Err(why) => panic!("could not open {}: {}", name, Error::description(&why)),
		Ok(file) => file,
	};
	match file.write_all(content) {
		Err(why) => panic!("could not write {}: {}", name, Error::description(&why)),
		Ok(ok) => ok,
	};
}