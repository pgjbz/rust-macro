macro_rules! using_a {
    ($a:ident, $e:expr) => {
        {
            let $a = 42;
            $e
        }
    }
}

fn main() {
    
    /*
     * a is a identifier will be used inside macro
     * by parameter $a:ident
     * */
    let four = using_a!(a, a / 10);

    

    /*
     * Expansion become
     * let four = {
     *      let a = 42;
     *      a / 10;
     * }
     * */
    println!("{}", four);
}
