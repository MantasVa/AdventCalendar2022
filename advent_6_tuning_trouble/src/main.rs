use std::{fs, vec};
use std::collections::HashSet;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()> {
    let communication_packet = fs::read_to_string("input.txt")?;

    for (index, char_14) in communication_packet.char_indices().skip(13) {
        let mut uniq = HashSet::new();

        let char_1 = communication_packet.chars().nth(index - 13).unwrap();
        let char_2 = communication_packet.chars().nth(index - 12).unwrap();
        let char_3 = communication_packet.chars().nth(index - 11).unwrap();
        let char_4 = communication_packet.chars().nth(index - 10).unwrap();
        let char_5 = communication_packet.chars().nth(index - 9).unwrap();
        let char_6 = communication_packet.chars().nth(index - 8).unwrap();
        let char_7 = communication_packet.chars().nth(index - 7).unwrap();
        let char_8 = communication_packet.chars().nth(index - 6).unwrap();
        let char_9 = communication_packet.chars().nth(index - 5).unwrap();
        let char_10 = communication_packet.chars().nth(index - 4).unwrap();
        let char_11 = communication_packet.chars().nth(index - 3).unwrap();
        let char_12 = communication_packet.chars().nth(index - 2).unwrap();
        let char_13 = communication_packet.chars().nth(index - 1).unwrap();

        let mut are_inserted: Vec<bool> = Vec::new();
        are_inserted.push(uniq.insert(char_1));
        are_inserted.push(uniq.insert(char_2));
        are_inserted.push(uniq.insert(char_3));
        are_inserted.push(uniq.insert(char_4));
        are_inserted.push(uniq.insert(char_5));
        are_inserted.push(uniq.insert(char_6));
        are_inserted.push(uniq.insert(char_7));
        are_inserted.push(uniq.insert(char_8));
        are_inserted.push(uniq.insert(char_9));
        are_inserted.push(uniq.insert(char_10));
        are_inserted.push(uniq.insert(char_11));
        are_inserted.push(uniq.insert(char_12));
        are_inserted.push(uniq.insert(char_13));
        are_inserted.push(uniq.insert(char_14));

        if are_inserted.iter().all(|b| *b) {
            let identifier: String = vec![char_1, char_2, char_3, char_4, char_5, char_6, char_7, char_8, char_9, char_10, char_11,char_12,char_13,char_14].into_iter().collect();
            let index = communication_packet.find(&identifier).unwrap();

            println!("Marker identifiers are: {}, index: {}", identifier, index + 14);
            break;
        }
    }

    return Ok(());
}
