use std::fs::File;
use std::io;
//The API libraries this program uses.
use csv::ReaderBuilder;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// The struct for the records in the csv file. This is Rust's equivalent of an object except it
/// only stores variables.
pub struct Trip {
    //These are the variables for the object.
    ref_number: String,
    disclosure_group: String,
    title_en: String,
    title_fr: String,
    name: String,
    purpose_en: String,
    purpose_fr: String,
    start_date: String,
    end_date: String,
    destination_en: String,
    destination_fr: String,
    //f64 is a 64 bit floating-point number. Option<> means that this entry could be blank.
    airfare: Option<f64>,
    other_transport: Option<f64>,
    lodging: Option<f64>,
    meals: Option<f64>,
    other_expenses: Option<f64>,
    total: Option<f64>,
    additional_comments_en: String,
    additional_comments_fr: String,
    owner_org: String,
    owner_org_title: String
}
///The methods for Record are stored in an implementation.
impl Trip {
    ///Calculates the duration of the trip in days as a 64 signed integer.
    pub fn calculate_duration(&self) -> i64 {
        let start_date = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap();
        let end_date = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap();
        //Note that there is no "return" statement. It's just the last line that gets returned.
        (end_date - start_date).num_days()
    }
}
/// This is the main method. It will access the csv file, put the first 10 entries into an array,
/// and then print the info of those entries.
fn main() -> Result<(), io::Error> {
    //Print out my name.
    println!("Written by Morgan Bakelmun");
    //Access the csv_file, located in the root folder of this program.
    let csv_file = match File::open("data/travelq.csv") {
        //This is Rust's equivalent of a try-catch block.
        //If it finds the file it returns it. Otherwise it gives the error message.
        Ok(file) => file,
        Err(error) => {
            panic!("Problem accessing the file: {:?}", error);
        }
    };
    //This part reads the content of the csv file.
    let mut reader = ReaderBuilder::new().from_reader(csv_file);
    //Here I'm making an empty Vec that will store records later on.
    //I could have used an array, but a Vec is similar and has more utility.
    let mut trips: Vec<Trip> = Vec::new();

    // Using a for loop to insert the first 10 lines into the records Vec.
    for entry in reader.deserialize().take(10) {
        let trip: Trip = entry?;
        trips.push(trip);
    }

    //Print some info from the Vec. Printing all the info would look messy.
    //That and I wanted to practice accessing variables from objects.
    for (_index, trip) in trips.iter().enumerate() {
        //The ampersands mean that I'm accessing a reference to the value, not the value itself.
        //It's a security thing for Rust. This way I can read the data but cannot modify it.
        //It also means I don't make a copy of these variables in memory, which saves time.
        let ref_num = &trip.ref_number;
        let name = &trip.name;
        let purpose = &trip.purpose_en;
        let destination = &trip.destination_en;
        let duration = &trip.calculate_duration();
        let total = format!("{:.2}",&trip.total.unwrap_or(0.00));
        //Each {} works like a placeholder. I could have done {ref_num} {name} etc., but stuck to conventions.
        println!("Ref Number: {}. Name: {}. Purpose: {}. Destination: {}. Duration: {} days. Total: ${}",
        ref_num,
        name,
        purpose,
        destination,
        duration,
        total);
    }
    println!("\nWritten by Morgan Bakelmun");
    Ok(())
}