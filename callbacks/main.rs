macro_rules! call_with_larch {
    ($callback:ident) => {
        $callback!(larch)
    };
}

macro_rules! expand_to_larch {
    () => {
        larch
    };
}

macro_rules! recognise_tree {
    (larch) => { println!("#1, the Larch.") };
    (redwood) => { println!("#2, the Mighty Redwood.") };
    (fir) => { println!("#3, the Fir.") };
    (chestnut) => { println!("#4, the Horse Checstnut") };
    (pine) => { println!("#5, the Scots Pine.") };
    ($($other:tt)*) => { println!("I don't know; some kind of birch maybe?") };
}

macro_rules! callback{
    ($callback:ident($($args:tt)*)) => {
        $callback!($($args)*)
    };
}

fn main() {
    recognise_tree!(expand_to_larch!()); 
    /*
     *  recognise_tree expand to another match
     *  {
     *      println!("I don't know;....")
     *  }
     *
     *  when call expand_to_larch macro this expand to
     *  {
     *      larch
     *  }
     *
     *  but this identifier don't has larch, ^mind blow^
     * */
    call_with_larch!(recognise_tree); //pass macro in another macro
    callback!(callback(println("Yes, this *was* unnecessary.")));
    //all
}
