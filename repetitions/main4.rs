macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! vec3 {
    ($($exprs:expr), *) => {
        {
            let size = count!($($exprs)*);
            let mut vec = Vec::with_capacity(size);
            $(
                vec.push($exprs);
            )*
            vec
        }
        
    }
    
}

fn main() {
    let vec = vec3![1,2,3];
    println!("{:?}", vec);
}
