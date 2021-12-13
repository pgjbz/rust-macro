macro_rules! replace_expr {
    ($_t:tt => $sub:expr) => {
        $sub
    };
}

macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        (
            $(
                replace_expr!(
                    ($tup_tys) =>
                    Default::default()
                )
            )*
        )
    }
}

fn main() {
    let a: (i32, i32, i32) = tuple_default!(i32);
    println!("{:?}", a);
}
