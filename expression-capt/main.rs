macro_rules! mult_five {
    ($nm:expr) => { 5 * $nm }
    /*
     * nm is a name of capturing expression
     * expr is mode to tell to compiler "Hey capture this expression"
     * */
}

macro_rules! mult_add {
    ($mult: expr, $ad1:expr, $ad2:expr) => { 
        $mult * ( $ad1 + $ad2 )
        /*
         * basicaly receive three expressions
         * a expression to multiply
         * maybe 2 * 2
         * others two expressions what will to be added
         * and multiply by first expressions
         * */
    }
}

fn main() {
    let mult_five = mult_five!(10 + 5);
    println!("{}", mult_five);
    let mult_add = mult_add![2 * 2, 10, 5];
    println!("{}", mult_add);
}
