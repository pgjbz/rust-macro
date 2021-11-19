macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}


macro_rules! match_tokens {
    ($a:tt + $b:tt) => {"got an addition"};
    (($i:ident)) => {"got an identifier"};
    ($($other:tt)*) => {"got something else"};
}

fn main() {
    println!("{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5));
    // By parsing the input into an AST node, the substituted result becomes un-destructible; U
    // cannot examine the contnts or match agains it ever again
    println!("{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5));
}
