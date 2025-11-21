use crate::structure::ParrotData;

pub fn run(data: &ParrotData) {
    for (tag, commands) in &data.recordings {
        println!("Tag: {}", tag);
        for (i, entry) in commands.iter().enumerate() {
            println!("  Entry {}: {}", i + 1, entry);
        }
    }
}
