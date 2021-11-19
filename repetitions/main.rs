macro_rules! vec_strs {
    (
        //start repetition
        $(
            $element:expr
        )
        , //seperated by commas...
        * //zero or more times, like Regex
    ) => {
        //Enclose the expansion in a block so that we can use
        //multiple staements.
        {
            let mut v: Vec<String> = Vec::new();
            //start a repetition:
            $(
                // Each repeat will contain the follwing statement, with
                // $element replaced with the corresponding expression
                v.push(format!("{}", $element));
            )*
            v
        }
    };

}

fn main() {
    let my_vec = vec_strs![1, 3, 2];
    println!("{:?}", my_vec);
}
