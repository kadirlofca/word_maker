pub mod word_maker {
    pub struct Word {
        pub body: String,
    }

    impl Word {
        pub fn set_body(&mut self, new_body: String) {
            self.body = new_body;
        }

        pub fn body(&self) -> &String {
            &self.body
        }
    }
}
