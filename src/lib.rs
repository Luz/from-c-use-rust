// Notes to my future self:
//
// 1) Vec in Rust has no defined ABI:
// Link: https://stackoverflow.com/questions/58311426/how-do-i-use-cbindgen-to-return-and-free-a-boxvec
// As we can not pass Vec over to C, we have the option to not use Vec at all (when passing it)
//
// 2) Passing raw pointers:
// We may want to pass handles of Rust objects to C and later have them back for further processing
// bindgen shall create a struct, as we do not want to use void pointers all over the place in C
// By just omitting the "#[repr(C)]", as seen for the Path structure, a C struct is created
// So if possible, prefer this over the c_void (dont "use std::os::raw::c_void")

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
// There is no C-represantation in C for a Vec, so dont use "#[repr(C)]"
// here, as cbindgen creates something uncompileable
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
    // A drop here is not necessary, as heap stuff goes out of scope and is deleted
    // drop(owned);
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
    println!("      {:?}", path);
    path.path.push(p);
    println!("      {:?}", path);

    path
}
