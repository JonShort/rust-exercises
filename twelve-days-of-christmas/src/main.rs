fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings, badam-pam-pam",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for (idx, &day) in days.iter().enumerate() {
        println!("On the {} day of Christmas", day);
        println!("My true love gave to me");

        if idx == 0 {
            println!("A partridge in a pear tree");
        } else {
            let gifts_current_day = &gifts[..idx + 1];

            for &gift in gifts_current_day.iter().rev() {
                println!("{}", gift);
            }
        }

        println!("\r\n");
    }
}
