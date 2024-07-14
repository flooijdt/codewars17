fn make_looper(string: &str) -> impl FnMut() -> char + '_ {
    fn string.into() -> &str {
        string.chars().next()
    }
}

fn main(){
    let mut abc = make_looper("abc");
    println!("{:?}", abc());
}
