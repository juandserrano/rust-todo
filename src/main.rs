use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify and item");

    let mut todo = Todo::new().expect("Initialization of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("And error occurred: {}", why)
        }
    }
    std::process::Command::new("cat").arg("db.txt").status().expect("Could not cat");
}

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>
}

impl Todo {
    fn insert(&mut self, key: String){
        //insert a new item into out map
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let content = std::fs::read_to_string("db.txt").expect("Unable to open the file. Check permissions and try again");
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), std::str::FromStr::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }
}
