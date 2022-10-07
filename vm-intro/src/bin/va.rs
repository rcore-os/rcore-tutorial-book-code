use std::alloc::{alloc, Layout};

fn main() {
    println!("location of code : {:#x?}", main as usize as *const usize);
    let layout = Layout::new::<[u8; 100]>();
    let ptr = unsafe { alloc(layout) };
    println!("location of heap : {:#x?}", ptr as usize as *const usize);
    let x = 3;
    let ptr = &x as *const i32;
    println!("location of stack : {:#x?}", ptr);
}
