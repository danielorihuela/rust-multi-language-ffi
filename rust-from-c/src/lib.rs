use std::mem;

#[repr(C)]
pub struct InMemoryKeyStore {
    keys: *mut u8,
    len: usize,
    cap: usize,
}

#[no_mangle]
pub extern "C" fn new_keystore() -> InMemoryKeyStore {
    let values = Vec::with_capacity(1);
    let mut values = mem::ManuallyDrop::new(values);
    let ptr = values.as_mut_ptr();
    InMemoryKeyStore {
        keys: ptr,
        len: values.len(),
        cap: values.capacity(),
    }
}

#[no_mangle]
pub extern "C" fn add(ks: &mut InMemoryKeyStore, value: u8) {
    let mut vector;
    unsafe {
        let new_v = Vec::from_raw_parts(ks.keys, ks.len, ks.cap);
        vector = mem::ManuallyDrop::new(new_v);
    }

    vector.push(value);
    ks.len = vector.len();
    ks.cap = vector.capacity();
}
