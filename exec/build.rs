fn main() {
  println!("cargo:rustc-link-search=native=/Users/mkind/Dev/Uni/RE23/graal/sdk/mxbuild/darwin-aarch64/libpythonvm.dylib.image");
  println!("cargo:rustc-link-search=native=/Users/mkind/Dev/Uni/RE23/graal/sdk/mxbuild/darwin-aarch64/PYTHON_JAVA_STANDALONE_JAVA22/graalpy-community-24.0.0-dev-macos-aarch64/lib/graalpy24.0");
  println!("cargo:rustc-link-lib=pythonvm");
  println!("cargo:rustc-link-lib=python-native");
}