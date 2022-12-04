fn main() {
    let mut gifts: String = "".to_owned();
    let mut gift; 
    
    for day in 1..13 {
        if day == 1 {
            println!("On the first day of Christmas");
        } else if day == 2 {
            println!("On the second day of Christmas");
        } else if day == 3 {
            println!("On the third day of Christmas");
        } else if day == 4 {
            println!("On jhe fourth day of Christmas");
        } else if day == 5 {
            println!("On the fifth day of Christmas");
        } else if day == 6 {
            println!("On the sixth day of Christmas");
        } else if day == 7 {
            println!("On the seventh day of Christmas");
        } else if day == 8 {
            println!("On the eighth day of Christmas");
        } else if day == 9 {
            println!("On the ninth day of Christmas");
        } else if day == 10 {
            println!("On the tenth day of Christmas");
        } else if day == 11 {
            println!("On the eleventh day of Christmas");
        } else if day == 12 {
            println!("On the twelfth day of Christmas");
        }
        println!("my true love sent to me:");
        if day == 1 {
            println!("A partridge in a Pear Tree");
            gift = format!("and a partridge in a Pear Tree");
            gifts.insert_str(0, &gift);
        } else {
            if day == 12 {
                gift = format!("{} Drummers Drumming\n", day);
            } else if day == 11 {
                gift = format!("{} Pipers Piping\n", day);
            } else if day == 10 {
                gift = format!("{} Lords a Leaping\n", day);
            } else if day == 9 {
                gift = format!("{} Ladies Dancing\n", day);
            } else if day == 8 {
                gift = format!("{} Maids a Milking\n", day);
            } else if day == 7 {
                gift = format!("{} Swans a Swimming\n", day);
            } else if day == 6 {
                gift = format!("{} Geese a Laying\n", day);
            } else if day == 5 {
                gift = format!("{} Golden Rings\n", day);
            } else if day == 4 {
                gift = format!("{} Calling Birds\n", day);
            } else if day == 3 {
                gift = format!("{} French Fries\n", day);
            } else {
                gift = format!("{} Turtle Doves\n", day);
            }
            gifts.insert_str(0, &gift);
            println!("{gifts}");
        }
        println!("");
    }
}
