pub mod dependency;

pub mod secret {
    fn f2() -> u32 {
        return 42;
    }

    pub fn f1() -> u32 {
        return f2();
    }
}

pub fn f3() -> u32 {
    // return secret::f2(); --> secrett fn
    return secret::f1();
}
