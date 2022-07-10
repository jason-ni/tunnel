fn main() {
    println!(r"cargo:rustc-link-search=gomodule");
    println!(r"cargo:rustc-link-lib=static=gomodule");
    println!(r"cargo:rerun-if-changed=gomodule/lib.go");
}
