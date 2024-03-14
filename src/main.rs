use word_maker::word_maker::Word;

fn main() {
    let mut my_word = Word {
        body: String::new(),
    };

    my_word.set_body(String::from("hello"));

    println!("{}", my_word.body);
}
