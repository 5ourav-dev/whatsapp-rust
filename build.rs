fn main() {
    println!("cargo:rerun-if-changed=whatsapp_res.o");
    println!("cargo:rustc-link-arg=whatsapp_res.o");
}
