#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

use std::{io::Read, path::PathBuf};

use rand::random;
use tangram_client as tg;
use tangram_error::Result;
const READ_SZ: usize = 65536;

pub async fn new_file(tg: &dyn tg::Handle) -> Result<tg::file::Id> {
	let mut contents: Vec<u8> = Vec::with_capacity(READ_SZ);
	for _ in 0..READ_SZ {
		contents.push(random());
	}
	let contents = tg::Blob::with_reader(tg, contents.as_slice()).await?;
	let object = tg::file::Object {
		contents,
		executable: false,
		references: Vec::new()
	};
	let file = tg::File::with_object(object);
	file.id(tg).await.cloned()
}

pub async fn create_files(tg: &dyn tg::Handle, count: usize) -> Result<Vec<String>> {
	let mut output = Vec::with_capacity(count);
	for _ in 0..count {
		let id = new_file(tg).await?;
		output.push(id.to_string());
	}
	Ok(output)
} 

pub fn bench_read(id: &str) -> Vec<u8> {
	let path = PathBuf::from("/Users/mikedorf/.tangram/artifacts").join(id);
	let mut file = std::fs::File::open(path).unwrap();
	let mut output = Vec::with_capacity(READ_SZ);
	file.read_to_end(&mut output).unwrap();
	output	
}
