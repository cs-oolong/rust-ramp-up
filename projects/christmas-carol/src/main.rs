fn main() {
    let days = [
        "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let first_paragraph = "
On the first day
Of Christmas
My true love
Sent to me
A partridge
In a pear tree
    ";
    println!("{}", first_paragraph);

    let items = [
        "Two turtledoves", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans\nA-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"
    ];

    let mut idx = 0;
    let mut acc = String::from("");    

    for day in days {
        let begin = format!("On the {day} day\nOf Christmas\nMy true love\nSent to me\n");
        let end = "And a partridge\nIn a pear tree";

        if day == "five" {
            acc.push_str(items[idx]);
            acc.push_str("\n");
            acc.push_str(items[idx]);
            acc.push_str("\n");
        }
        else if day == "eleventh" {
            let mut acc_copy = String::from(acc.clone());
            acc_copy.push_str("I sent eleven\nPipers piping");
            acc_copy.push_str("\n");
        }
        else {
            acc.push_str(items[idx]);
            acc.push_str("\n");
            println!("{}", format!("{begin}{acc}{end}\n"));
        }
        idx += 1;
    }
}
