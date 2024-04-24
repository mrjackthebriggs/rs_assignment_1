use std::{fs::File, io::{Read, Write}};

//Word Type Enums for categorising words
#[derive(PartialEq)]    // SO == can be used on the wrapped enums
pub enum WordType{
Noun,
Verb,
Adv,
Adj,
Prep,
Misc,
PropNoun,
NounVerb,
}

//Converts a String to a WordType
pub fn string_to_type(str_in:impl ToString) -> Result<WordType,String>
{
    let input = str_in.to_string();

    if input == "n"{
        return Ok(WordType::Noun);
    }
    else if input == "v"{
        return Ok(WordType::Verb);
    }
    else if input == "adv"{
        return Ok(WordType::Adv);
    }
    else if input == "adj"{
        return Ok(WordType::Adj);
    }
    else if input == "prep"{
        return Ok(WordType::Prep);
    }
    else if input == "pn"{
        return Ok(WordType::PropNoun);
    }
    else if input == "n_and_v"{
        return Ok(WordType::NounVerb);
    }
    else if input == "misc"{
        return Ok(WordType::Misc);
    }
    else {
        return Err(String::from(format!("{} not valid, please input a valid type",input)));
    }

    

}

//Converts WordType to String
pub fn type_to_string(type_in:&WordType) -> String
{
    match type_in{
        WordType::Noun => String::from("noun"),
        WordType::Verb => String::from("verb"),
        WordType::Adv => String::from("adverb"),
        WordType::Adj => String::from("adjective"),
        WordType::Prep => String::from("preposition"),
        WordType::Misc => String::from("miscellaneous"),
        WordType::PropNoun => String::from("proper noun"),
        WordType::NounVerb => String::from("noun and verb"),
    }
}

/*
    Word structure for holding word information
    Holds Options because there are points the word object will be blank, needed for loading words.
*/
pub struct Word{
    pub word:Option<String>,
    pub word_type:Option<WordType>,
    pub def:Option<String>
}

impl Word{
    pub fn print_word(&self){
        if self.word.is_some(){
            println!("\n{}\n[{}]\n{}\n",self.word.as_ref().unwrap(),type_to_string(self.word_type.as_ref().unwrap()),self.def.as_ref().unwrap());
        }
    }

    pub fn new(word_in:impl ToString, word_type_in:impl ToString, def_in:impl ToString) -> Self{
        let str_word_in = word_in.to_string().to_lowercase();
        let str_word_type = word_type_in.to_string().to_lowercase();
        let str_word_def = def_in.to_string();
        Self{
            word:Some(str_word_in),
            word_type:Some(string_to_type(str_word_type).expect("Invalid input")),
            def:Some(str_word_def)
        }
    }

    pub fn new_blank() -> Self{
        Self{
            word:None,
            word_type:None,
            def:None
        }
    }
}

/*
Function for loading the words from a text document and turning it into a vector of Word objects
takes a reference to the "dictionary" and the file location, returns a boolean.
Notes:
- Reads each word object
- Had issues with reading each line and constructing an object
Used Some/None and checked if it had None to assign a value
*/
pub fn load_dictionary(dict:& mut Vec<Word>, load_loc:impl ToString) -> Result<i32,String>
{
    let load_str = load_loc.to_string();

    let mut file_ob = match File::open(load_str){
        Ok(the_file) => the_file,
        Err(_) => return Err(String::from("File could not be loaded"))
    };

    //Read lines
    let mut file_str:String = String::new();
    file_ob.read_to_string(& mut file_str);
    let mut word_buf:Word = Word::new_blank();
    for lin in file_str.lines()
    {     
        
        if lin == "<word>"
        {
            word_buf = Word::new_blank();
            continue;
        }

        else if lin == "</word>"
        {
            dict.push(word_buf);
            word_buf = Word::new_blank();   //needs to be here for memeory safety
            continue;
        }

        //checks if blank, if so, overwrites with actual value
        if word_buf.word == None
        {
            word_buf.word = Some(String::from(lin));
            continue;
        }

        else if word_buf.def == None
        {
            word_buf.def = Some(String::from(lin));
            continue;
        }  

        else if word_buf.word_type == None
        {
            word_buf.word_type = Some(string_to_type(lin).expect("Invalid input from .txt doc"));
            continue;
        }     
    }

    //if no words were loaded, it probably failed
    if dict.len() > 0 {
        Ok(dict.len().try_into().unwrap())
    }
    else {
        Err(String::from("No objects loaded"))
    }   
}

/* 
Finds the word using linear searching, unless char is remapped, binary searching is not possible
Returns a result
Notes:
- learned the use of as_ref, dunno why I can't just use '&' here
*/
pub fn find_word(dict:& Vec<Word>, search: impl ToString) -> Result<&Word,String>
{
    let search_str = search.to_string().to_lowercase();

    for dict_word in dict
    {
        if dict_word.word.as_ref().unwrap() == &search_str
        {
            return Ok(dict_word);
        }
    }

    return Err(String::from("Word not found"));
}

/*
Returns a vector of words in the dictionary that contains more than three z's
Notes:
- Rusts built in methods are a life saver, so easy to work with!
*/
pub fn mt3z_words(dict:& Vec<Word>) -> Vec<String>
{
    let mut ret_vec:Vec<String> = Vec::new();

    for dict_word in dict
    {
        let mut z_count:u16 = 0;
        for str_char in dict_word.word.as_ref().unwrap().chars()
        {
            if str_char == 'z'
            {
                z_count += 1;
            }
        }

        if z_count > 3
        {
            ret_vec.push(dict_word.word.as_ref().unwrap().clone());
        }
    }

    return ret_vec;
}

/*
Adds a word object to the dictionary and saves it as a .txt doc
Requires a mut ref for the dict, a word obj to add and a destination to save the .txt file.
Notes:
- as_ref.unwrap().clone().etc. These long method chains are cool, and efficient and all, wish there were better ways to do this
- enuming the word types came in clutch here
*/
pub fn add_word(dict:& mut Vec<Word>, in_word:Word, dest: impl ToString) -> Result<String, String>
{
    //does word already exist?
    match find_word(dict, in_word.word.as_ref().unwrap())
    {
        Ok(_wrd) => return Err(String::from("Word already exists")),
        Err(_er) => dict.push(in_word)
    }

    //now new word added, prepare string to write
    let mut write_str:String = String::new();

    for wrd in dict
    {
        write_str.push_str("<word>\n");

        write_str.push_str(wrd.word.as_ref().unwrap().clone().as_str());
        write_str.push_str("\n");

        write_str.push_str(wrd.def.as_ref().unwrap().clone().as_str());
        write_str.push_str("\n");

        let type_word:&str = match wrd.word_type.as_ref().unwrap()
        {
            WordType::Noun => "n",
            WordType::Verb => "v",
            WordType::Adv => "adv",
            WordType::Adj => "adj",
            WordType::Prep => "prep",
            WordType::Misc => "misc",
            WordType::PropNoun => "pn",
            WordType::NounVerb => "n_and_v",
        };
        write_str.push_str(type_word);
        write_str.push_str("\n");

        write_str.push_str("</word>\n");
    }

    let mut file_dest = dest.to_string();

    let mut file = match File::create(file_dest.clone())
    {
        Ok(fl) => fl,
        Err(_er_str) => return Err(String::from("Failed to create new dict file"))
    };

    file.write_all(write_str.as_bytes());

    Ok(file_dest)
}