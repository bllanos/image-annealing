use bpaf::Bpaf;

// Note that extra whitespace at the start of some comment lines below is intentional,
// as it is required to preserve line breaks.
// See https://docs.rs/bpaf/latest/bpaf/parsers/struct.NamedArg.html#method.help
#[derive(Bpaf, Clone, Debug)]
#[bpaf(options, generate(make_option_parser), version)]
/// Run algorithms on image files.
///  (Pass `--help --help` or `-hh` to see full argument documentation)
///  
///  ## File formats
///  
///  This section describes the file formats used for reading and writing data types.
///  The semantics of the data types themselves are described in the project's README.
///  
///  ### Maps
///  
///  Maps are represented as (non-animated) PNG files with Red, Green, Blue, and Alpha channels,
/// where each pixel has 8-bits per channel.
/// Therefore, the file has the truecolor and alpha color type, not the indexed color type.
/// See <https://en.wikipedia.org/wiki/PNG#Pixel_format> for more information on PNG file formats.
///  
///  Channels values for a pixel represent the following parts of the map vector with its origin at that pixel:
///  
///  1. The Red channel stores the most significant byte of the `x`-component
///  2. The Green channel stores the least significant byte of the `x`-component
///  3. The Blue channel stores the most significant byte of the `y`-component
///  4. The Alpha channel stores the least significant byte of the `y`-component
///  
///  To convert image files to this format, you can use [ImageMagick](https://imagemagick.org/), for example, by running the following command:
///  
///  ```bash
///  convert "${input_image_filename}.${extension}" PNG32:"${output_image_filename}.png"
///  ```
///  
///  To inspect the format of an image, run the following [ImageMagick](https://imagemagick.org/) command:
///  
///  ```bash
///  identify -verbose "${input_image_filename}.${extension}"
///  ```
///  
///  ### Data
///  
///  Image data uses the same file format as [maps](#maps) except that there are 16-bits per channel in order to accommodate high-precision colors.
/// (Data that users do not intend to represent images also must be stored as PNG files for consistency.)
///  
///  In GPU shader code, 16-bit values from the PNG file's channels are interpreted as big-endian values.
/// This is the interpretation used by the [PNG decoder](https://docs.rs/png/latest/png/struct.Reader.html#method.next_frame).
///  
///  To convert image files to this format, you can use [ImageMagick](https://imagemagick.org/), for example, by running the following command:
///  
///  ```bash
///  convert "${input_image_filename}.${extension}" -endian MSB PNG64:"${output_image_filename}.png"
///  ```
///  
///  If you want to convert output images back to 8-bit color depth, you can use the conversion command given [above](#maps) (assuming you want to keep an alpha channel).
///  
///  ### Uniform buffers
///  
///  The contents of uniform buffers are represented as plain binary files.
/// The entire contents of a file will be loaded as-is into a uniform buffer.
///  
///  ## Command-line interface
pub struct Options {}

#[cfg(test)]
mod tests;
