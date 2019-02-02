use std::error::Error;
use std::io;
use std::process;
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


    let mut row: Vec<String> = vec![];

    for _ in 0..1000 {

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        row.clear();
        row.push(c1_values[rng.gen::<usize>() % c1_values.len()].to_string());
        row.push(format!("{}", c2_values[rng.gen::<usize>() % c1_values.len()]));
        row.push(format!("{}", rng.gen::<i8>()));
        row.push(format!("{}", rng.gen::<i16>()));
        row.push(format!("{}", rng.gen::<i32>()));
        row.push(format!("{}", rng.gen::<i64>()));
        row.push(format!("{}", rng.gen::<u8>()));
        row.push(format!("{}", rng.gen::<u16>()));
        row.push(format!("{}", rng.gen::<u32>()));
        row.push(format!("{}", rng.gen::<u64>()));
        row.push(format!("{}", rng.gen::<f32>()));
        row.push(format!("{}", rng.gen::<f64>()));
        row.push(rand_string);

        wtr.write_record(&row)?;
    }
    wtr.flush()?;

    Ok(())
}