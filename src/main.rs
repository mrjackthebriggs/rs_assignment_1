use std::io;
use rs_assignment_1::source;

fn main() {

    let mut dict:Vec<source::Word> = Vec::new(); 
    let mut current_file:String = String::new();
    const DEFAULT:&str = "dictionary_2023S1.txt";

    println!("--- Welcome to: Dictionary Manager! ---");

    'main:loop {
        println!("    Input:");
        println!("\t1) Load Dictionary");
        println!("\t2) Search for Word");
        println!("\t3) Words with more than 3 Zs");
        println!("\t4) Add word to Dictionary");
        println!("\t0) To Exit");

        //Parsing is a lot better with writing in the domain 
        //PARSING
        let mut inp:String = String::new();
        io::stdin().read_line(&mut inp).expect("Invalid Input");
        let parsed_inp:u16= match inp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue 'main
        };

        print!("{}: ", parsed_inp);

        // Inputs are processed and Functions are called here
        match parsed_inp {
            1 => {  //Load Dictionary
                    println!("Load Dictionary");
                    let mut load_loc:String = String::new();

                    'input:loop{   //Process input
                    println!("What dictionary file would you like to load?:\n(leave blank for default dictionary) ");

                        if io::stdin().read_line(& mut load_loc).is_ok(){
                            load_loc = load_loc.trim().to_string();
                            
                            if load_loc == ""{
                                load_loc = DEFAULT.to_string();
                            }

                            current_file = load_loc.clone();

                            break 'input;}  
                    } 

                    match source::load_dictionary(& mut dict, load_loc){
                            Ok(cnt) => println!("Dictionary loaded {} words successfully\n", cnt),
                            Err(msg) => println!("ERROR: {}\n",msg)  
                    }
            },
            2 => { // Search for word
                println!("Search for word");
                
                if !dict.is_empty(){
                    let mut search_word:String = String::new();

                    'input:loop{   //Process input
                        println!("Enter a word to search for: \t");
                            
                            if io::stdin().read_line(& mut search_word).is_ok(){
                                search_word = search_word.trim().to_string();
                                break 'input;}  
                    } 

                    match source::find_word(& dict, search_word){
                        Ok(wrd) => wrd.print_word(),
                        Err(msg) => println!("{}\n",msg)
                    }
                }
                else{
                    println!("ERROR: Please load a dictionary file first");
                }
            },
            3 => {  //Print words with more than three z's
                println!("Words with more than 3 Z's");
                if !dict.is_empty() {
                    println!("Words with more than 3 'Z's found:");
                    for zzzword in source::mt3z_words(& dict){
                        println!("\t{}",zzzword);
                    }
                    println!("");
                }
                else{
                    println!("ERROR: Please load a dictionary file first");
                }
            },
            4 => {  // Add a word to dictionary
                println!("Add a word to dictionary");

                if !dict.is_empty() {
                    // get input for word
                    println!("\nWhat word would you like to add?: ");
                    let mut word_str:String = String::new();
                    if io::stdin().read_line(&mut word_str).is_ok() {
                        word_str = word_str.trim().to_string();
                    }

                    // check if it already exists
                    if source::find_word(&dict, word_str.clone()).is_ok() {
                        println!("ERROR: Word already exists, elevated privledges needed.\n");
                        continue 'main;
                    }

                    //get input for type and description
                    let mut type_str:String;
                    'type_loop:loop{
                        type_str = String::new();

                        println!("\nWhat type of word is your new word?:");
                        println!("(v for verb)");
                        println!("(n for noun)");
                        println!("(adv for adverb)");
                        println!("(adj for adjective)");
                        println!("(prep for preposition)");
                        println!("(pn for proper noun)");
                        println!("(n_and_v for noun and a verb)");
                        println!("(misc for other words)");

                        if io::stdin().read_line(& mut type_str).is_ok() {
                            type_str = type_str.trim().to_string();
                        }

                        match source::string_to_type(&type_str){
                            Ok(_tp) => {
                                break 'type_loop;
                            },
                            Err(_er) => {
                                println!("\nERROR: Invalid type, please try again")
                            }
                        }
                    }

                    // Input word description
                    println!("\nWhat does the word mean?;");
                    let mut word_def:String = String::new();
                    io::stdin().read_line(& mut word_def).expect("Description invalid, please try again");
                    word_def = word_def.trim().to_string();

                    let new_word:source::Word = source::Word::new(word_str,type_str, word_def);

                    'decisonloop:loop{
                        new_word.print_word();
                        println!("... will be added, are you. is everything correct?(y/n)");
                        let mut decision:String = String::new();
                        io::stdin().read_line(& mut decision).expect("Description invalid, please try again");
                        decision = decision.trim().to_lowercase().to_string();
                        if &decision == "n"{
                            println!("Word addition cancelled, please try again");
                            continue 'main;
                        }

                        else if &decision == "y"{
                            println!("\nAdding {} to the loaded dictionary", new_word.word.as_ref().unwrap());
                            break 'decisonloop;
                        }

                        else{
                            println!("\nplease only input 'y' or 'n'")
                        }
                    }


                    
                    // input file name
                    let mut file_name:String = String::new();
                    println!("\nWhat would you like to name the new dictionary file?\n(leave blank to overwrite current)");
                    io::stdin().read_line(& mut file_name).expect("File name invalid, please try again");
                    file_name = file_name.trim().to_string();
                    if &file_name == ""
                    {
                        file_name = current_file.clone();
                    }
                    
                    match source::add_word(& mut dict, new_word, file_name)
                    {
                        Ok(msg) => println!("File successfully written at {}. Word has been added to loaded dictionary", msg),
                        Err(msg) => println!("ERROR:{}",msg)
                    }
                }
                else{
                    println!("ERROR: Please load a dictionary file first");
                }
            },
            0 => break 'main,   //number not between 1 and 4
            _other => println!("Input a number between 0 to 4!")
        }



        
    }

    println!("Thanks for using dictionary manager!");
}
