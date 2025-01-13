fn main() {
    fn sing() {
        println!("The Twelve Days of Christmas");
        let birds: [&str; 12] = ["partridge","turtle doves,","French hens,","calling birds",
        "gold rings","geese a-laying","swans a-swimming","maids a-milking","ladies dancing","lords a-leaping","pipers piping","drummers drumming"];
        let mut index:usize = 0; 
        let mut jndex:usize = 0;
        let mut day: usize;
        loop {
            index +=1;
            jndex = index;
            day = index +1; 
            println!("On the {} day of Cristmas my true love sent to me", day); 
            loop {
                if jndex < 12 {
    
                    println!("{jndex} {}", birds[jndex]);
                }
                jndex -=1;
                if jndex == 0 {
                    println!("And one {} in a pear tree\n", birds[0]);
                    break;}
            }
            
            if index == 11 {break;}
            
        };
    }

    sing();

}
