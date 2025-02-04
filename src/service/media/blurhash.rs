use std::{fmt::Display, io::Cursor, path::Path};

use blurhash::encode_image;
use image::{DynamicImage, ImageDecoder, ImageError, ImageFormat, ImageReader};

/// Returns the blurhash or a blurhash error which implements Display.
fn get_blurhash_from_request(data: &[u8], mime: Option<&str>, filename: Option<&str>, config: BlurhashConfig) -> Result<String, BlurhashingError> {
	// Get format image is supposed to be in
	let format = get_format_from_data_mime_and_filename(data, mime, filename)?;
	// Get the image reader for said image format
	let decoder = get_image_decoder_with_format_and_data(format, data)?;
	// Check image size makes sense before unpacking whole image
	if is_image_above_size_limit(&decoder, config) {
		return  Err(BlurhashingError::ImageTooLarge);
	}
	// decode the image finally
	let image = DynamicImage::from_decoder(decoder)?;

	blurhash_an_image(image, config)

}


/// Gets the Image Format value from the data,mime, and filename
/// It first checks if the mime is a valid image format
/// Then it checks if the filename has a format, otherwise just guess based on the binary data
/// Assumes that mime and filename extension won't be for a different file format than file.
fn get_format_from_data_mime_and_filename(
	data: &[u8],
	mime: Option<&str>,
	filename: Option<&str>,
) -> Result<ImageFormat, BlurhashingError> {
	let mut image_format = None;
	if let Some(mime) = mime {
		image_format = ImageFormat::from_mime_type(mime)
	}
	if let (Some(filename), None) = (filename, image_format) {
		if let Some(extension) = Path::new(filename).extension() {
			image_format = ImageFormat::from_mime_type(extension.to_string_lossy())
		}
	}

	if let Some(format) = image_format {
		Ok(format)
	} else {
		image::guess_format(data).map_err(|e| e.into())
	}
}

fn get_image_decoder_with_format_and_data(
	image_format: ImageFormat,
	data: &[u8],
) -> Result<Box<dyn ImageDecoder + '_>, BlurhashingError> {
	let mut image_reader = ImageReader::new(Cursor::new(data));
	image_reader.set_format(image_format);
	Ok(Box::new(image_reader.into_decoder()?))
}

fn is_image_above_size_limit<T: ImageDecoder>(decoder: &T, blurhash_config: BlurhashConfig) -> bool {
	decoder.total_bytes() >= blurhash_config.size_limit
}
#[inline]
fn blurhash_an_image(image: DynamicImage, blurhash_config: BlurhashConfig) -> Result<String, BlurhashingError> {
	Ok(encode_image(blurhash_config.components_x, blurhash_config.components_y, &image.to_rgba8())?)
}
#[derive(Clone, Copy)]
pub struct BlurhashConfig {
	components_x: u32,
	components_y: u32,
	/// size limit in bytes
	size_limit: u64,
}

#[derive(Debug)]
pub(crate) enum BlurhashingError {
	ImageError(Box<ImageError>),
	HashingLibError(Box<blurhash::Error>),
	ImageTooLarge
}
impl From<ImageError> for BlurhashingError {
	fn from(value: ImageError) -> Self {
		Self::ImageError(Box::new(value))
	}
}

impl From<blurhash::Error> for BlurhashingError {
	fn from(value: blurhash::Error) -> Self {
		Self::HashingLibError(Box::new(value))
	}
}

impl Display for BlurhashingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"Blurhash Error:");
		match &self  {
			Self::ImageTooLarge => write!(f,"Image was too large to blurhash"),
			Self::HashingLibError(e) => write!(f, "There was an error with the blurhashing library => {}", e),
	
			Self::ImageError(e) => write!(f, "There was an error with the image loading library => {}", e)
			
		};

		Ok(())
		
	}
}
