fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let gifts = [
        "A partrigde in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let mut first_verse = true;

    for number in 0..12 {
        let day = days[number];
        println!("On the {day} day of Christmas, my true love sent to me");

        for number in (0..(number + 1)).rev() {
            let gift = gifts[number];
            if gift == gifts[0] && !first_verse {
                println!("And a {gift}")
            } else if gift == gifts[0] && first_verse {
                println!("{gift}");
                first_verse = false;
            } else {
                println!("{gift},");
            }
        }

        println!("");
    }
}
