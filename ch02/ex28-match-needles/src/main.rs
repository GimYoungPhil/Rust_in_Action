fn main() {
    // let needle = 42;
    let haystack: [u16; 10] = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

      if result == "hit!" {
          println!("{}: {}", item, result);
      }
    }

    for item in &haystack {
        match item {
            1 => { println!("{}: odd", item); },
            2 => { println!("{}: even", item); },
            _ => { println!("{}: some", item); },
        }
    }

    for item in &haystack {
        if *item == 1 {
            println!(". {}", item);
        } else {
            println!("_ {}", item);
        }
    }
}
