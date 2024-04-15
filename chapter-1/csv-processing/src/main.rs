// executable projects require a main() function
fn main() {
    /* escapes the trailing newline character */
    let peng_data = "\
        common name, length (cm)
        Little Penguin, 20
        Green-eyed Penguin, 35
        Sumgaitian Penguin, 50
        Invalid, data";

    let records = peng_data.lines(); // returns an iterator

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            /* skip the header and empty lines */
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, length: {}cm", name, length);
        }
    }
}
