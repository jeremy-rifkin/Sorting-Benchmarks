fn main() {
	// gcc -c src/calgos/insertionsort.c -o src/calgos/insertionsort.o
	println!("cargo:rustc-link-search=all=object/");
	//println!("cargo:rustc-link-lib=libgcc")
}