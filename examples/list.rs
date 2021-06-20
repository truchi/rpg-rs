use std::fs::read;

fn main() {
    let file = "./examples/list.txt";

    let list = String::from_utf8(read(file).unwrap()).unwrap();
    let mut new_list = String::new();
    let mut anims = String::new();

    for line in list.lines() {
        if line.is_empty() {
            new_list.push('\n');
            continue;
        }

        let parts = line.split(' ').collect::<Vec<_>>();
        let is_anim = parts[0].ends_with("_anim");

        debug_assert!(parts.len() == 5 || parts.len() == 6 && is_anim);
        debug_assert!(parts[1].len() < 4);
        debug_assert!(parts[2].len() < 4);
        debug_assert!(parts[3].len() < 4);
        debug_assert!(parts[4].len() < 4);

        let name = parts[0].to_uppercase();
        let x: u16 = parts[1].parse().unwrap();
        let y: u16 = parts[2].parse().unwrap();
        let w: u16 = parts[3].parse().unwrap();
        let h: u16 = parts[4].parse().unwrap();

        if !is_anim {
            new_list.push_str(&format!(
                "{:<50} {:>3} {:>3} {:>3} {:>3}\n",
                name, x, y, w, h
            ));
        } else {
            let frames: u16 = parts[5].parse().unwrap();

            anims.push_str(&format!("{} {}\n", frames, name));

            for i in 0..frames {
                let name = format!("{}_{}", name, i);
                let x = x + i * w;
                anims.push_str(&format!(
                    "    {:<46} {:>3} {:>3} {:>3} {:>3}\n",
                    name, x, y, w, h
                ));
            }
        }
    }

    println!("{}", new_list.trim());
    println!("==================================================================");
    println!("{}", anims.trim());
}
