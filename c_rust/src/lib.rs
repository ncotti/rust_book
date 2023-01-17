use std::slice;


pub extern "C" fn call_from_c() {
    println!("Rust code called from C.");
}

// Function with unsafe code.
pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    // Estos dos líneas son iguales
    //let ptr = values.as_mut_ptr();
    let ptr = &mut values[0] as *mut i32;

    assert!(mid <= len);

    // ptr es lo mismo que devolver un puntero a una posición intermedia
    // de memoria (&values[0], &values[mid-1])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}