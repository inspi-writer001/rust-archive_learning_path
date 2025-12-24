fn main() {
    println!("Hello, and welcome to pointers in Rust!");

    let some_var = 24;
    let some_new_var: *const i32 = &some_var;

    let mut some_string: &'static str = "205";
    let some_new_string = &some_string;
    let another_some_new_string = &some_string;

    unsafe {
        println!("{:?}", some_new_var);
    }

    let mut_ptr: *mut &str = &mut some_string as *mut &str;
    let mut_ptr1: *mut &str = &mut some_string as *mut &str;

    unsafe {
        *mut_ptr = "205";
        *mut_ptr1 = "225";
    }

    unsafe {
        let byte_ptr = some_string.as_ptr();
        let first_byte = *byte_ptr;
        println!("First byte as u8: {}", *byte_ptr as u8);
        println!("Second byte as u8: {}", *byte_ptr.offset(1) as u8);
        println!("Third byte as u8: {}", *byte_ptr.offset(2) as u8);

        println!("string 205 as le bytes {:?}", some_string.as_bytes());
        println!(
            "string casted using parse {:?}",
            some_string.parse::<u8>().unwrap()
        );
        println!("number 205 as le bytes {:?}", 205u8.to_le_bytes());
    }

    #[derive(Debug, Clone, Copy)]
    struct Couple {
        v1: u64,
        v2: u64,
    }

    let couple = Couple { v1: 200, v2: 32 };
    let couple_ptr = &couple as *const Couple;

    let couple_ptr_v1 = &couple as *const Couple as *const u64;

    println!("v1: {:?}", unsafe { (*couple_ptr).v1 });
    println!("v2: {:?}", unsafe { (*couple_ptr).v2 });

    println!("v1 over bytes stack: {:?}", unsafe { *couple_ptr_v1 });
    println!("v2 over bytes stack: {:?}", unsafe {
        *couple_ptr_v1.offset(1)
    });
}
