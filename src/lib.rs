// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/nlg-system/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod generator {
  const COULD_NOT_OPEN: &'static str = "Could not open";
  const SPLIT_FILE: &'static str = " ";  
  const SPLIT_INPUT: &'static str = " ";
  const OUTPUT_SEPARATOR: &'static str = " ";
  const OUTPUT_REPLACE: &'static str = " \n";
  const OUTPUT_END: &'static str = "\n";
  const PARSE_ERROR: &'static str = "parse error";
  const EMPTY_WORD: &'static str = "";
  extern crate std;

  /// The `dictionary` structure is database of synonym.
  pub struct Dictionary {
    types: [std::collections::HashMap<usize, String>; 255],
  }

  impl Dictionary {
    pub fn from_files (
      files: &Vec<String>,
    ) -> Self {
      use std::io::BufRead;
      let lines = open(&files);
      let mut types: [std::collections::HashMap<usize, String>; 255] = unsafe {
        std::mem::uninitialized()
      };
      for i in 0..255 {
        unsafe {
          std::ptr::write(&mut types[i], std::collections::HashMap::new());
        }
      }

      for mut content in lines {
        let mut line = String::new();

        while content.read_line(&mut line).is_ok()
        && !line.is_empty() {
          let (word, dictionaries):(String, Vec<usize>) = parse(&line);

          for dictionary in dictionaries {
            let index = types[dictionary].len();

            types[dictionary].insert(index, word.clone());
          }
          line.clear();
        }
      }
      Dictionary {
        types: types,
      }
    }

    /// The `display` function returns a collection of words.
    fn get_value (
      &self,
      types: usize,
      missing: usize,
    ) -> String {
      match self.types[types].get(&missing) {
        Some(word) => word.clone(),
        None => EMPTY_WORD.to_string(),
      }
    }

    /// The `get_word` function returns the word according to
    // a number of dictionary and a number of index.
    fn get_word (
      &self,
      line: &String,
    ) -> String {
      let left:String = line.chars().take_while(|d|
        d.is_numeric()
      ).collect();
      let right:String = line.chars().skip_while(|d|
        d.is_numeric()
      ).skip(1).take_while(|d|
        d.is_numeric()
      ).collect();

      self.get_value(match left.parse() {
        Ok(digit) => digit,
        Err(_) => 0,
      }, match right.parse() {
        Ok(digit) => digit,
        Err(_) => 0,
      })
    }

    /// The `get_words` function returns a collection of words.
    pub fn get_words (
      &self,
      lines: &String,
    ) -> Vec<String> {
      let mut words:Vec<String> = Vec::new();

      for line in lines.split(SPLIT_INPUT) {
          words.push(self.get_word(&line.to_string()));
      }
      words
    }

    /// The `display` function returns a formated text.
    pub fn display (
      lines: &Vec<String>,
    ) -> String {
      let mut out:String = String::new();

      for line in lines {
        out.push_str(line);
        out.push_str(OUTPUT_SEPARATOR);
      }
      out.push_str(OUTPUT_END);
      out.replace(OUTPUT_REPLACE, OUTPUT_END);
      out
    }
  }

  /// The `parse` function returns the key and value from a line.
  fn parse (
    line: &str,
  ) -> (String, Vec<usize>) {
    let mut split = line.split(SPLIT_FILE);
    let mut value = Vec::new();
    let key = match split.next() {
      Some(word) => word.to_string(),
      None => panic!(PARSE_ERROR)
    };

    loop {
      value.push(match split.next() {
        Some(digit) => digit.to_string().trim().parse().unwrap(),
        None => break ,
      });
    }
    return (key, value);
  }

  /// The `open` function returns a file's opened list for a file's list.
  fn open (
    files: &Vec<String>,
  ) -> Vec<std::io::BufReader<std::fs::File>> {
    let mut lines: Vec<std::io::BufReader<std::fs::File>> = Vec::with_capacity(
      files.capacity()
    );
    for file in files {
      let path = std::path::PathBuf::from(file);

      match std::fs::File::open(&path) {
        Err(why) => panic!("{} {}: {:?}", COULD_NOT_OPEN, path.display(), why),
        Ok(file) => {
          lines.push(std::io::BufReader::new(file))
        },
      }
    }
    lines
  }
}
