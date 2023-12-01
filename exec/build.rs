fn main() {
  println!("cargo:rustc-link-lib=lib1");
  println!("cargo:rustc-link-lib=lib2");
  println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
}