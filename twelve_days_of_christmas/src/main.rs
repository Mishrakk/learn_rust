use std::io::stdout;

fn main() {
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day_index in 0..12 {
        clearscreen::clear().unwrap();
        let mut verse = format!(
            "On the {} day of Christmas my true love sent to me\n",
            days[day_index]
        );
        for j in (0..day_index + 1).rev() {
            let mut line = format!("{}\n", lyrics[j]);
            if day_index > 0 && j == 0 {
                format_last_line(&mut line);
            }
            verse.push_str(line.as_str());
        }
        verse.push_str("\n");
        ferris_says::say(verse.as_bytes(), 60, &mut stdout()).unwrap();
        press_btn_continue::wait("Press any key to continue...\n").unwrap();
    }
}

fn format_last_line(verse: &mut String) {
    *verse = verse.remove(0).to_lowercase().to_string() + &*verse;
    *verse = "And ".to_owned() + &*verse;
}
