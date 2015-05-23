// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/synonym/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate nlglib;

fn main() {
  let files = vec!["./data/man.word".to_string()];
  let dictionary = nlglib::generator::Dictionary::from_files(&files);
  let mut line:String = String::new();
  loop {
    match std::io::stdin().read_line(&mut line) {
      Ok(_) => {
        let out:Vec<String> = dictionary.get_words(&line.chars().take_while(|x|
          *x != '\n'
        ).collect());
        match std::io::Write::write(
          &mut std::io::stderr(),
          &nlglib::generator::Dictionary::display(&out).into_bytes()
        ) {
          Ok(_) => line.clear(),
          Err(why) => panic!("Unable to write to stderr: {}", why),
        }
      },
      Err(_) => break ,
    }
  }
}
