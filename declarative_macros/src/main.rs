fn main() {
    println!("Hello, from declarative macro!");

    let v1: Vec<u8> = pec![];
    let v2: Vec<&str> = pec!["a", "b", "c", "d", "e",];

    let v3 = vec![23; 4];

    println!("{:?} {:?} {:?}", v2, v1, v3);

    saturate! {
        struct Phil {
            pub pubkey: [u8; 32],
            pub max_number: [u8; 8]
        }
    };

    fully_saturate! {
       pub struct Pill {
            pub pubkey: [u8; 32],
            pub max_number: [u8; 8]
        }
    };

    println!(
        "u64 value {:?}",
        u64::from_le_bytes([3, 3, 3, 3, 3, 3, 3, 3])
    );

    println!("u64 max value {:?}", u64::MAX);

    let new_phil = Phil {
        pubkey: [2u8; 32],
        max_number: [3u8; 8],
    };

    let new_pill = Pill {
        pubkey: [3u8; 32],
        max_number: [9u8; 8],
    };

    println!("{:?}", new_phil);
    println!("{:?}", new_pill);
    println!("{:?}", new_pill.max());
    println!("{:?}", new_pill.to_bytes());
    let mee = pec![2;32_000_000];
    // println!("pec repeated twos{:?}", pec![2;32_000_000]);

    let name = "Phil";
    let concatenated_string = string_joiner!("Hello ", "friend ", name, ", what say you?");
    let just_str_joiner = str_joiner!("hey ", "what's up");
    println!("{:?}", concatenated_string);
    println!("{:?}", just_str_joiner);
}

#[macro_export]
macro_rules! pec {

    () => {
        Vec::new()
    };

    ($($val: expr),+ $(,)?) => {{
        let mut new_vec = Vec::new();
        $(new_vec.push($val);)*
        new_vec
    }};

    ($val:expr ; $length:expr) => {{
        // let mut empty_vec = Vec::new();
        // for _ in 0..$length {
        //     empty_vec.push($val)
        // }
        // empty_vec

        [$val; $length].to_vec() // we might  have a stack overflow error here if user uses large number for the length..e.g pec![2;32_000_000]
        // as it needs to be created on the stack but doing a `[$val; $length].to_vec()` might help
    }}
}

#[macro_export]
macro_rules! saturate {
    ($st: item) => {
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        #[repr(C)]
        $st

        //    impl ToSized for $st {
        //         pub fn to_bytes(&self) -> Vec<u8> {
        //             core::mem::si
        //         }

        //         pub fn max(&self) -> {
        //             core::mem::size_of::<self>();
        //         }
        //     }
    };
}

#[macro_export]
macro_rules! fully_saturate {
    ($visibility:vis struct $name:ident { $($field_items:tt)+ }) => {

        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
        #[repr(C)]
        $visibility struct $name {
            $($field_items)*
        }

       impl $name {
        pub fn max(&self) -> usize {
            core::mem::size_of::<Self>()
        }

        pub fn to_bytes(&self) -> &[u8] {
            bytemuck::bytes_of(self)
        }
       }
    };
}

#[macro_export]
macro_rules! string_joiner {
    ($($string_:expr),* $(,)?) => {{
        let mut strings = String::new();
        $(strings.push_str($string_);)*
        strings

    }};
}

#[macro_export]
macro_rules! str_joiner {
    ($($s:literal),* $(,)?) => {
        concat!($($s),*)

    };
}

// a vec! macro equivalent
#[macro_export]
macro_rules! slector {
    // empty vec arm slector!();
    () => {
         Vec::new()
    };

    // slector![2,4,5,6,7]
    ($($vec_item:expr),+ $(,)?) => {
        {
            // let mut new_vec = Vec::with_capacity([$($vec_item),*].len());
            let mut new_vec = Vec::with_capacity($crate::count![@COUNT; $($vec_item),*]);


            $(new_vec.push($vec_item);)+
            new_vec
        }
    };

    // with repeating values for x length
    ($repeating_value:literal; $count:literal) => {
        {
            let mut new_vec = Vec::new();
            // for _ in 0..$count {
            //     new_vec.push($repeating_value)
            // }

            new_vec.resize($count, $repeating_value);
            new_vec
        }
    };

    // slector![0;3, 3,4,5, 2;5]
    ($starting_repeat:literal; $s_count:literal, $($arr_item: literal),+ $(;)?  $ending_repeat:literal; $e_count:literal) => {

        {
            // let mut new_vec = Vec::new();
            let total_len = $s_count + [$($arr_item),+].len() + $e_count;
            let mut new_vec = Vec::with_capacity(total_len);

            for _ in 0..$s_count {
                new_vec.push($starting_repeat);
            }

            $(new_vec.push($arr_item);)+

            let current_vec_length = new_vec.len();

            for _ in current_vec_length..current_vec_length + $e_count {
                new_vec.push($ending_repeat);
            }
            new_vec
        }
    };



}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($values:expr),+) => {
        <[()]>::len(&[$($crate::count![@SIZE; $values]),*])
    };

    (@SIZE; $_values:expr) => {
        {()}
    }
}

// a macro for mojo resolving and Deriving on impl
// pub struct Flash { pub name: [u8; 4],}
#[macro_export]
macro_rules! mojo {
    ($visibility:vis struct $name:ident { $($struct_content:tt)* }) => {

        #[repr(C)]
        #[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        $visibility struct $name   {
            $($struct_content)*
        }

        impl $name  {
             fn to_bytes(&self) -> &[u8] {
                bytemuck::bytes_of(self)
            }

             fn max(&self) -> usize {
                core::mem::size_of::<Self>()
            }
        }
    };
}
// a string joiner macro

#[cfg(test)]
mod test {

    #[test]
    fn test_slector() {
        let first_use: Vec<u8> = slector![];

        let second_use = slector![1, 3, 5, 6,];

        let third_use = slector![2; 4];

        let fourth_use = slector! [0;200_000_000, 3,4,5; 2;30_000_000];
        let duplicate_fourth_use = {
            let mut first_vec = vec![0; 200_000_000];
            let mut second_vec = vec![3, 4, 5];
            let mut third_vec = vec![2; 30_000_000];

            first_vec.append(&mut second_vec);
            first_vec.append(&mut third_vec);

            first_vec
        };

        // println!(
        //     "{:?} {:?} {:?} {:?}",
        //     &first_use, &second_use, &third_use, &fourth_use
        // );
        assert_eq!(first_use, vec![]);
        assert_eq!(second_use, vec![1, 3, 5, 6]);
        assert_eq!(third_use, vec![2, 2, 2, 2]);
        assert_eq!(fourth_use, duplicate_fourth_use);
    }

    #[test]
    fn test_mojo() {
        mojo! {
            pub struct MyFlash {
            pub name: [u8; 8],
            pub age: [u8; 1],

        }}

        let new_flash = MyFlash {
            age: 23u8.to_le_bytes(),
            name: {
                let mut free = [0u8; 8];
                let name = b"Flower";
                free[..name.len()].copy_from_slice(name);
                free
            },
        };

        println!("{:?}", new_flash);
        println!("{:?}", new_flash.to_bytes());
        println!("{:?}", new_flash.max());
    }
}

#[macro_export]
macro_rules! expand_vector {
    () => {
        Vec::new()
    };
    ($($val:expr),+ $(,)?) => {
        {
            let mut new_vec = Vec::new();

            $(new_vec.push($val);)*
            new_vec
        }
    };
    ($val:expr; $count:literal) => {
        {
            let mut new_vec = Vec::with_capacity($count);
            new_vec.resize($count, $val);
            new_vec
        }
    }
}

// #[repr(C)]
// #[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug)]
// pub struct Wtf {
//     pub amount: [u8; 8],

//     pub address: [u8; 32],
// }

// impl Wtf {
//     pub const SIZE: usize = core::mem::size_of::<Self>();

//     pub fn fetch_address(&self) -> &[u8; 32] {
//         &self.address
//     }

//     pub fn size(&self) -> usize {
//         core::mem::size_of::<Self>()
//     }

//     pub fn to_bytes(&self) -> &[u8; Self::SIZE] {
//         let bytes = bytemuck::bytes_of(self);
//         bytes.try_into().unwrap()
//         // let mut max_size = [0u8; Self::SIZE];
//         // max_size[..Self::SIZE].copy_from_slice(bytes);
//         // max_size
//     }
// }

#[macro_export]
macro_rules! third_mojo {
    ($visibility:vis struct $struct_name:ident {$($token_visibility:vis $token_name:ident : $struct_tokens:ty),* $(,)?}) => {
        #[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug, PartialEq)]
        #[repr(C)]
        $visibility struct $struct_name {
            // $($struct_tokens)*,
            $($token_visibility $token_name: $struct_tokens),*
        }

       paste::paste!{
        impl $struct_name {
             pub const SIZE: usize = core::mem::size_of::<Self>();

            // pub fn fetch_address(&self) -> &[u8; 32] {
            //     &self.address
            // }

        //    $( pub fn [<fetch_ $token_name>](&self) -> $struct_tokens {
        //         &self.$token_name
        //     })*

            pub fn size(&self) -> usize {
                Self::SIZE
            }

            pub fn to_bytes(&self) -> &[u8; Self::SIZE] {
                let bytes = bytemuck::bytes_of(self);
                bytes.try_into().unwrap()

            }

            $(
                 pub fn [ < fetch_ $token_name > ](&self) -> &$struct_tokens {
                        &self.$token_name
                    }
                )*
        }}
    };
}

#[cfg(test)]
mod tpest {
    third_mojo! { pub struct Wtf {
        pub amount: [u8; 8],
        pub address: [u8; 32],
    }}

    #[test]
    fn test_expand() {
        let first_vector: Vec<u8> = expand_vector!();

        let second_vector = expand_vector![2; 10];

        let third_vector = expand_vector![2, 4, 6, 7, 6, 8];

        let new_guy = Wtf {
            address: [3u8; 32],
            amount: {
                let value: u64 = 30_000_000;
                value.to_le_bytes()
            },
        };

        println!("new guy bytes: {:?}", new_guy.to_bytes());
        // println!("new guy address: {:?}", new_guy.fetch_address());
        println!("new guy size: {:?}", new_guy.size());
        println!("new guy fetch_address {:?}", new_guy.fetch_address());
        println!("new guy fetch_amount {:?}", new_guy.fetch_amount());

        println!("{:?} {:?} {:?}", first_vector, second_vector, third_vector);
    }
}
