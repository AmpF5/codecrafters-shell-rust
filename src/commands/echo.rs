pub struct Echo {}

impl Echo {
    pub fn execute(value: &String) {
        println!("{}", value)
    }
}
