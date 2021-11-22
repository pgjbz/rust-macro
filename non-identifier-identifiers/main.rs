macro_rules! what_is {
    (self) => {
        "the keyword is 'self'"
    };
    ($i:ident) => {format!("the idenfifier '{}'", stringify!($i)) }
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {
        $c!($i) 
    };
}

fn main(){
    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
    /*
     *  call_with_ident require a identifier, match self
     *  and replace with self keyword when call what_is macro
     * */
}
