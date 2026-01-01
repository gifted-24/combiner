use std::env;

#[derive(Debug)]
pub struct Args {
    pub image1: String,
    pub image2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        let mut args: env::Args = env::args();
        Args {
            image1: args.nth(1).unwrap(),
            image2: args.next().unwrap(),
            output: args.next().unwrap(),
        }
    }
}
