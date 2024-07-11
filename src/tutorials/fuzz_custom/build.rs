fn main() {
	cc::Build::new().file("src/vulnerable.c").compile("vulnerable");
}
