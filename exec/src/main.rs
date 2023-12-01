
extern "C" {
    fn Py_IsInitialized() -> u32;
}

fn main() {
    unsafe {
        println!("Loaded libs");
        let a = Py_IsInitialized();
        print!("{a}");
    }
}
