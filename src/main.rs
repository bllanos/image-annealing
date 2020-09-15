use image_annealing::config;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let parse_result = config::parse_args();
    if let Err(err) = parse_result {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }

    let (filename, rect) = parse_result.unwrap();

    rect.iter()
        .for_each(|x| println!("Image rectangle is {:?}", x));

    let filepath = config::check_input_path(&filename)?;
    let output_path = config::make_output_filepath(&filepath)?;
    let img = image::open(filepath)?;

    println!("Saving image to: {}", output_path.to_str().unwrap());
    img.save(output_path)?;

    Ok(())
}
