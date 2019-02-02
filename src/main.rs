use std::error::Error;
use std::io;
use std::result::Result;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    gen_aggregate_data().unwrap();
}


fn gen_aggregate_data() -> Result<(), Box<Error>> {

    let mut wtr = csv::Writer::from_writer(io::stdout());

    let mut rng = rand::thread_rng();


    let c1_values = vec!["a", "b", "c", "d", "e"];
    let c2_values = vec![1, 2, 3, 4, 5];


    let mut header: Vec<String> = vec![];
    for i in 1..=13 {
        header.push(format!("c{}", i));
    }
    wtr.write_record(&header)?;

    let mut row: Vec<String> = vec![];
    for _ in 0..100 {

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        row.clear();
        row.push(c1_values[rng.gen::<usize>() % c1_values.len()].to_string()); //c1
        row.push(format!("{}", c2_values[rng.gen::<usize>() % c1_values.len()])); //c2
        row.push(format!("{}", rng.gen::<i8>())); // c3
        row.push(format!("{}", rng.gen::<i16>())); // c4
        row.push(format!("{}", rng.gen::<i32>())); // c5
        row.push(format!("{}", rng.gen::<i64>())); // c6
        row.push(format!("{}", rng.gen::<u8>()));  // c7
        row.push(format!("{}", rng.gen::<u16>())); // c8
        row.push(format!("{}", rng.gen::<u32>())); // c9
        row.push(format!("{}", rng.gen::<u64>()));  // c10
        row.push(format!("{}", rng.gen::<f32>())); //c11
        row.push(format!("{}", rng.gen::<f64>())); //c12
        row.push(rand_string); //c13

        wtr.write_record(&row)?;
    }
    wtr.flush()?;

    Ok(())
}