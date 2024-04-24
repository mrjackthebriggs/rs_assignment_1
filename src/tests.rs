#[cfg(test)]
mod tests{
    use crate::source::{self, find_word};
    #[test]
    fn word_full_construct(){
        let word_name = String::from("Test");
        let word_type = String::from("n");
        let word_def = String::from("Where something is tested");

        let new_word = source::Word::new(word_name,word_type,word_def);

        assert_eq!(new_word.word.unwrap().as_str(),"test");
        assert_eq!(source::type_to_string(new_word.word_type.as_ref().unwrap()),"noun");
        assert_eq!(new_word.def.unwrap().as_str(),"Where something is tested");
    }

    #[test]
    fn word_print(){
        let word_name = String::from("Test");
        let word_type = String::from("n");
        let word_def = String::from("Where something is tested");

        let new_word = source::Word::new(word_name,word_type,word_def);

        new_word.print_word();
    }

    #[test]
    fn read_file(){
        let mut test_dict:Vec<source::Word> = Vec::new();
        assert!(source::load_dictionary(& mut test_dict, "dictionary_2023S1.txt").is_ok());
        println!("At position 5 is:");
        test_dict.get(5).unwrap().print_word();
    }

    #[test]
    fn search_word_success(){
        let mut test_dict:Vec<source::Word> = Vec::new();
        assert!(source::load_dictionary(& mut test_dict, "dict1.txt").is_ok());

        let search = "abide";
        let found = match source::find_word(&test_dict, search){
            Ok(str) => str.word.as_ref().unwrap().clone(),
            Err(er) => er
        };

        assert_eq!(search, found);
    }

    #[test]
    fn search_word_fail(){
        let mut test_dict:Vec<source::Word> = Vec::new();
        assert!(source::load_dictionary(& mut test_dict, "dict1.txt").is_ok());

        let search = "bannana";
        let found = match source::find_word(&test_dict, search){
            Ok(str) => str.word.as_ref().unwrap().clone(),
            Err(er) => er
        };

        assert_ne!(search, found);
    }

    #[test]
    fn z3word_test(){
        let mut test_vec:Vec<source::Word> = Vec::new();
        assert!(source::load_dictionary(& mut test_vec, "dictionary_2023S1.txt").is_ok());

        let zzzret = source::mt3z_words(&test_vec);

        assert_eq!(zzzret.len(),1);
        assert_eq!(zzzret.get(0).unwrap(),"razzamatazz");
    }

    #[test]
    fn file_write_test(){
        let mut test_dict:Vec<source::Word> = Vec::new();
        assert!(source::load_dictionary(& mut test_dict, "dict1.txt").is_ok());

        let new_word = source::Word::new("Pissing", "v", "Where you piss");
        source::add_word(& mut test_dict, new_word, "dict2.txt");

        let mut new_dict:Vec<source::Word> = Vec::new();
        assert!(source::load_dictionary(& mut new_dict, "dict2.txt").is_ok());

        let found = find_word(&new_dict, "Pissing").unwrap();
        assert_eq!(found.word.as_ref().unwrap(),"pissing");
    }
}
