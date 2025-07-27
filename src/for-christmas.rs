fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves,",
        "three French Hens,",
        "four Calling Birds,",
        "five Gold Rings,",
        "six Geese a Laying,",
        "seven Swans a Swimming,",
        "eight Maids a Milking,",
        "nine Ladies Dancing,",
        "ten Lords a Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,"
    ];

    for day in 0..12 {
        println!(
            "\nOn the {} day of Christmas my true love sent to me:",
            days[day]
        );

        for gift in (0..=day).rev() {
            if gift == 0 && day != 0 {
                println!("and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
    }
}

