use image_annealing::config;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let parse_result = config::parse_args(env::args());
    if let Err(err) = parse_result {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    }
    let (filename, rect) = parse_result.unwrap();

    rect.iter()
        .for_each(|x| println!("Image rectangle is {:?}", x));

    let filepath = config::check_input_path(&filename)?;
    let output_path = config::make_output_filepath(&filepath)?;
    let img = image::open(filepath)?;

    // let prepare_result = annealing::prepare_image(&img, rect.as_ref());
    // if let Err(err) = prepare_result {
    //     eprintln!(
    //         "Problem preparing the image file {} for processing: {}",
    //         filename, err
    //     );
    //     process::exit(1);
    // }
    // let prepared_img = prepare_result.unwrap();

    // let process_result = annealing::process_image(&prepared_img);
    // if let Err(err) = process_result {
    //     eprintln!("Problem processing the image file {}: {}", filename, err);
    //     process::exit(1);
    // }
    // let processed_img = process_result.unwrap();

    println!("Saving image to: {}", output_path.to_str().unwrap());
    img.save(output_path)?;

    Ok(())
}
