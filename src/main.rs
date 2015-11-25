mod io;

// Print compressed output in readable form? 
const PRINT:  bool = false;

// Store compressed output to files?
const STORE:  bool = false;

fn main() {
	// Run with different files as input.
	run("input_lipsum.txt");
	run("input_random.txt");
	run("input_repetitive.txt");
	run("README.md");
	
	// Hey, lets compress our own sourcecode :)
	run("src/main.rs");
	run("src/io.rs");
}

fn run(name : &str) {
	// Read input file
	let input = io::read_file(&name);
	
	// Compress and decompress input
	let compressed = compress(&input);
	let decompressed = decompress(&compressed);
	
	// Write compressed to file
	if STORE {
		let mut outfile = String::from(name);
		outfile.push_str(".compressed");
		io::write_file(&compressed, &outfile);
	}
	
	// Display analysis
	println!("{}: compressed from {} to {}, decompresed content {}",
		name, input.len(), compressed.len(),
		if input == decompressed {
			"matched."
		} else {
			"DID NOT MATCH!"
	});
}

// The idea is as follows:
// We look at the input as a stream of bytes (unsigned 8bit integer).
// When the current charactar can be found the already processed text, we look
// for the longest matching sequence and output its offset plus length.
// Offset and length have to be <= 255 to fit into a single byte.
// If we cannot find the current character within the previous 255 symbols,
// then we output offset 0 and the character itself.
fn compress(data : &[u8]) -> Vec<u8> {
	let mut output = Vec::new();
	let mut pos = 0;
	while pos < data.len() {
		let (offset, len) = find_longest_match(data, pos);
		output.push(offset);
		if offset == 0 {
			if PRINT { println!("('{}')", data[pos] as char) }
			output.push(data[pos]);
			pos += 1;
		} else {
			output.push(len);
			pos = pos + (len as usize);
			if PRINT { println!("({}, {})", offset, len) }
		}
	}
	return output;
}

// pos is current position in input value
fn find_longest_match(data : &[u8], pos : usize) -> (u8, u8) {
	let mut best_offset = 0u8;
	let mut best_len = 0u8;
	let start = if pos > 255 {
		pos - 255
	} else {
		0
	};
	
	for offset in start..pos {
		let len = matcher(data, offset, pos);
		if len > best_len {
			best_offset = (pos - (offset as usize)) as u8;
			best_len = len;
		}
	}
	return (best_offset, best_len);
}

fn matcher(data : &[u8], offset : usize, end : usize) -> u8 {
	let mut offset = offset;
	let mut pos = end;
	let mut len = 0u8;
	
	// Here I had to add len < 255 for repetitive inputs!
	while offset < pos && pos < data.len() && data[offset] == data[pos] && len < 255 {
		offset += 1;
		pos += 1;
		len += 1;
	}
	return len;
}

fn decompress(data : &[u8]) -> Vec<u8> {
	let mut output = Vec::new();
	let mut pos = 0;
	while pos + 1 < data.len() {
		let header = data[pos];
		let item = data[pos + 1];
		pos += 2;
		if header == 0 {
			output.push(item);
		} else {
			let offset = header as usize;
			let len = item as usize;
			let start = output.len() - offset;
			for i in start..(start+len) {
				let char = output[i];
				output.push(char);
			}
		}
	}
	return output;	
}