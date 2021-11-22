//macro_rules! X {
//    () => {};
//}
//defined on all mods

//    mod a {
//        X!(); 
//    }
//    
//    macro_rules! X {() => {};} //defined only belowi
//    
//    mod b{
//    //    macro_rules! X {() => {};} //defined only this mod
//        X!(); //definifed
//    }
//    
//    mod c{
//        X!(); 
//    }

fn main() {
    macro_rules! X {
    () => { Y!() }};
    fn a() {
        macro_rules! Y { () => {"Hi!"} }
        assert_eq!(X!(), "Hi!");
        {
            assert_eq!(X!(), "Hi!");
            macro_rules! Y { () => {"Bye!"} }
            assert_eq!(X!(), "Bye!");
        }
        assert_eq!(X!(), "Hi!");
    }
    
    fn b() {
        macro_rules! Y { () => {"One more"} }
        assert_eq!(X!(), "One more");
    }
    println!("Hello World!");
    a();
    b();
}
