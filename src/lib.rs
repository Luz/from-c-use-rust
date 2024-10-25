#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
// There is no C-representation in C for a Vec, so dont use "#[repr(C)]".
// Otherwise, cbindgen would create something uncompileable.
pub struct Path {
    path: Vec<Point>,
}

#[no_mangle]
pub extern "C" fn point(x: i64, y: i64) -> Point {
    println!("Rust: point({}, {})", x, y);
    let p = Point { x: x, y: y };
    return p;
}
#[no_mangle]
pub extern "C" fn newPath() -> *mut Path {
    println!("Rust: newPath()");
    let h = Box::new(Path::default());
    Box::into_raw(h)
}
#[no_mangle]
pub extern "C" fn newPathFromZero() -> *mut Path {
    println!("Rust: newPathFromZero()");
    let p = Point { x: 0, y: 0 };
    let path = Path { path: vec![p] };
    Box::into_raw(Box::new(path))
}
#[no_mangle]
pub extern "C" fn deletePath(h: *mut Path) {
    let owned = unsafe { Box::from_raw(h) };
    println!("Rust: deletePath: {:?}", owned);
    // "drop(owned);" is not necessary:
    // Rust frees the data on the heap as soon as the pointer goes out of scope
}
#[no_mangle]
pub extern "C" fn addPointToPath(handle: *mut Path, p: Point) -> *mut Path {
    if handle.is_null() {
        println!("Received a null pointer :(");
        println!("Giving back a null pointer :)");
        return handle;
    }
    let path: &mut Path;

    // Dereference the raw pointer to get our desired object
    unsafe {
        path = &mut *handle;
    }

    println!("Rust: addPointToPath({:?}, {:?})", path, p);
    println!("{:?}", path);
    path.path.push(p);
    println!("{:?}", path);

    path
}
