use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashSet;
use std::io::Cursor;

pub async fn get_data_from_url(nozomi_address: String) -> HashSet<i32> {
    let response = match reqwest::get(&nozomi_address).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HashSet::new();
        }
    };

    let body = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading response: {}", e);
            return HashSet::new();
        }
    };

    let mut nozomi = HashSet::new();
    let mut cursor = Cursor::new(body);

    while let Ok(value) = cursor.read_i32::<BigEndian>() {
      print!("{}", value);
        nozomi.insert(value);
    }

    return nozomi;
}
