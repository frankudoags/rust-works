use std::collections::HashMap;
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(unused_mut)]

pub(crate) fn main() {
    //Hashmap with keys of i32 and values of i32
    let mut map: HashMap<i32, i32> = HashMap::new();
    // writing to the hashmap. key is 1 and value is 2
    map.insert(1, 2);
    // updating the value of key 1 to 3
    map.insert(1, 3);
    // removing the key 1 from the hashmap
    map.remove(&1);
    // adding a new key value pair to the hashmap
    map.insert(2, 4);
    // reading the value of key 2 from the hashmap
    let value = map.get(&2);

    // Adding a Key and Value Only If a Key Isn’t Present
    map.entry(3).or_insert(7);
    println!("Hashmap is: {:?}", map); // Hashmap is: {3: 7, 2: 4}
    
    //value of key 3 is 7, so the value of key 3 is not updated
    map.entry(3).or_insert(8);
    println!("Hashmap is: {:?}", map); // Hashmap is: {3: 7, 2: 4}


    let text = "hello world wonderful world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) to the value for this key.
        // to use this mutable reference, we first have to dereference it using the asterisk (*)
        // this basically means that we are assigning the value of the mutable reference to the count variable
        // then we increment the value of count by 1, but to do so,
        // we have to dereference count again using the asterisk (*)
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
}

pub fn complex_hashmap_with_structs() {
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: (name.to_string()),
                country: (country.to_string()),
            }
        }
    }
    //Hashmap with keys of type Viking  and values of i32
    let mut vikings: HashMap<Viking, i32> = HashMap::from([
        (Viking::new("Ragna Lobtrakit", "Norway"), 99),
        (Viking::new("Ivar The Boneless", "Norway"), 54),
        (Viking::new("Thor, Thunder God", "Asgard"), 200),
        (Viking::new("Loki, Deceit God", "Asgard"), 170),
    ]);

    vikings.insert(Viking::new("Odin, god of gods", "Asgard"), 1000);

    for (viking, health) in &vikings {
        println!(
            "{} from {}, has a {}% health factor.\n",
            viking.name, viking.country, health
        );
    }
}
