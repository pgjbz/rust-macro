macro_rules! make_mutable {
    ($i:ident) => {
        let mut $i = self
    };
}

macro_rules! double_method {
    ($self_:ident, $body:expr) => {
        fn double(mut $self_) -> Dummy {
            $body
        }  
    };
}

#[derive(Debug)]
struct Dummy(i32);


impl Dummy {
    // fn double(self) -> Dummy {
    //     make_mutable!(self);
    //     self.0 *= 2;
    //     self
    // }
    double_method!{self, {
        self.0 *= 2;
        self
    }}
}

fn main() {
    let dum = Dummy(2);
    let dum = dum.double();
    println!("{:?}", dum)
}

