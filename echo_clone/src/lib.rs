pub fn default_echo(vec: Vec<String>) {
    for arg in vec {
        print!("{} ", arg);
    }

    println!("");
}

pub fn interpretate_echo(vec: Vec<String>) {
    let mut sentence = String::new();

    for word in vec {
        if !word.contains("-") {

            for (index, char) in word.chars().enumerate() {
                let mut last_char_index = 0;

                println!("{:?}", (index, char));

                if char != ' ' {
                    last_char_index = index;
                } 

                sentence.push(char);
            }
        }
    }

    println!("{:?}", sentence);
}

pub fn check_for_option(vec: Vec<String>) -> Vec<String> {
    let mut options: Vec<String> = Vec::new();

    for arg in vec {
        if arg.contains("-") {
            options.push(arg);
        }
    }

    options
}

pub fn remove_first<T>(vec: &mut Vec<T>) {

    if vec.is_empty() {
        return ();
    }

    vec.remove(0);
}
