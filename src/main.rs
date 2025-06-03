fn main() {
    let a = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtle-doves",
        "And a partridge in a pear tree",
    ];

    for day in 1..=12 {
        println!("Day {}:", day);
        for i in (12 - day)..12 {
            println!("{}", a[i]);
        }
        println!();
    }
}

