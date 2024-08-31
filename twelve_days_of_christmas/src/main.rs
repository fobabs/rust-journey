fn main() {
    twelve_days_of_xmas();
}

fn twelve_days_of_xmas() {
    const LYRICS: [&str; 12] = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese A-Laying",
        "Seven Swans A-Swimming",
        "Eight Maids A-Milking",
        "Nine Ladies Dancing",
        "Ten Lords A-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    for (i, lyric) in LYRICS.iter().enumerate() {
        let suffix = match i {
            0 => "st",
            1 => "nd",
            2 => "rd",
            _ => "th"
        };
        println!("On the {}{} day of Christmas, my true love gave to me,", i + 1, suffix);

        if i == 0 {
            println!("{}.", lyric);
        } else {
            for x in (0..(i + 1)).rev() {
                if let 0 = x {
                    println!("And {}", LYRICS[0]);
                } else {
                    println!("{}", LYRICS[x]);
                }
            }
        }
    }
}
