//! TODO: https://github.com/fuzzstati0n/fuzzgoat/tree/master
//! Although this may have been built for Linux

use std::process::Command;

extern "C" {
	//fn main(num: i32, args: &[&[char]]) -> i32;
	fn test() -> i32;
}

#[test]
pub fn start() {
	/*Command::new("./build.sh")
		.status()
		.unwrap();*/

	fuzz();
}

pub fn execute(args: Vec<String>) {

}

pub fn fuzz() {
	unsafe {
		println!("{}", test());	
	}
}

