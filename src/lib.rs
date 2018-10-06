#[cfg(test)]
pub mod chapel {
    pub fn init() {
        println!("Chapel, constructed!");
    }
}

mod tests {

    #[test]
    fn build() {
        super::chapel::init()
    }
}
