macro_rules! vec2 {
    ($($exprs:expr,)*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($exprs);
            )*
            vec
        }
    }
}

macro_rules! vec3 {
    ($($exprs:expr), *) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($exprs);
            )*
            vec
        }
    };
}

fn main() {
    let vec =vec2![1, 2, 3,]; //in this case comma is necessary
    vec.iter().for_each(|i| println!("{}", i));
    let vec = vec3![1,2,3]; //in this case comma is not necessary
    vec.iter().for_each(|i| println!("{}", i));
}
