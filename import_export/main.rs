mod macros {
    #[macro_export] macro_rules! X { () => { Y!(); } }
    #[macro_export] macro_rules! Y { () => {} }
}

//#[macro_use] extern crate macs; //only use an external create from the root module

X!(); //only visible if use #[macro_use]

fn main(){
}

/*
 * Macros can be exported from the current crate using #[macro_export]
 * when import macros using #[macro_use], u can import specifics macros like that
 *
 * #[macro_use(X)] extern crate xpto;
 * 
 * */
