mod bindings;

fn main() {
    unsafe {
        println!("Loaded libs");
        let (isolate, isolate_thread) = unsafe {
            let mut isolate_ptr: *mut bindings::__graal_isolate_t = std::ptr::null_mut();
            let mut isolate_thread_ptr: *mut bindings::__graal_isolatethread_t =
                std::ptr::null_mut();

            assert!(
                bindings::graal_create_isolate(
                    std::ptr::null_mut(),
                    &mut isolate_ptr,
                    &mut isolate_thread_ptr
                ) == 0
            );
            (isolate_ptr, isolate_thread_ptr)
        };
        println!("created isolate and isolate_thread");
        let a = bindings::Py_IsInitialized(isolate_thread);
        print!("{a}");
        unsafe {
            assert!(bindings::graal_tear_down_isolate(isolate_thread) == 0);
        }
    }
}
