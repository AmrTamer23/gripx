pub struct Params {
    pub word: String,
    pub file: String,
}
impl Params {
    pub fn verify(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let word = args[1].to_string();
        let file = args[2].to_string();
        Ok(Params { word, file })
    }
}
