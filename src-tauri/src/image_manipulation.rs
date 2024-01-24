use std::error::Error;
use image::{
    self, 
    io::Reader as ImageReader, 
    ImageBuffer, imageops::FilterType
};



const CHAR_MAPPING: [char; 8] = [
    ' ', 
    '.', 
    ':', 
    '+', 
    'H', 
    'X', 
    '#', 
    '@'
];

pub trait Image {
    type Output;
    
    fn new(width: u32, height: u32) -> Self;
    fn from_path(path: &str) -> Result<Self::Output, Box<dyn Error>>;
    fn dimentions(&self) -> (u32, u32);
}

pub struct ImageWrapper {
    pub buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

impl Image for ImageWrapper {
    type Output = ImageWrapper;
    
    fn new(width: u32, height: u32) -> Self {
        let blank_image = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));
        
        ImageWrapper {
            buffer: blank_image,
            width,
            height,
        }
    }

    fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        let reader = ImageReader::open(path)?;
        let image = reader.decode()?;
        let rgb_image = image.to_rgb8();
        
        Ok(
            ImageWrapper {
                buffer: rgb_image.clone(),
                width: image.width(),
                height: image.height(),
            }
        )
    }

    fn dimentions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl ImageWrapper {
    pub fn prepare_scale(&mut self) {
        // Since monospace font characters are approximately twice as high
        // as they are wide, we should half the image's height for a good looking result.
        let new_height = self.height / 2;
        let new_buffer = image::imageops::resize(
            &self.buffer, 
            self.width, 
            new_height, 
            FilterType::Gaussian
        );

        self.buffer = new_buffer;
    }
}


fn get_pixel_brightness(pixel: &image::Rgb<u8>) -> u32 {
    let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);
    let brightness = 
        0.2126 * red as f32 + 
        0.7152 * green as f32 + 
        0.0722 * blue as f32;
    
    brightness.round() as u32
}

fn pixel_to_char(pixel: &image::Rgb<u8>) -> char {
    match get_pixel_brightness(pixel) {
        0..=31 => CHAR_MAPPING[0],
        32..=62 => CHAR_MAPPING[1],
        63..=93 => CHAR_MAPPING[2],
        94..=124 => CHAR_MAPPING[3],
        125..=155 => CHAR_MAPPING[4],
        156..=186 => CHAR_MAPPING[5],
        187..=217 => CHAR_MAPPING[6],
        218..=248 => CHAR_MAPPING[7],
        _ => CHAR_MAPPING[7],
    }
}

fn convert_to_2d_charmatrix(image_wrapper: &mut ImageWrapper, scale_options: ImageScaleOptions) -> Vec<Vec<char>> {
    
    if let ImageScaleOptions::HalfHeight = scale_options {
        image_wrapper.prepare_scale();
    }

    let pixels = image_wrapper.buffer.pixels();
    
    let mut text_image: Vec<Vec<char>> = Vec::new();
    let mut text_image_row: Vec<char> = Vec::new();
    let mut row_counter: u32 = 0;
    
    for pixel in pixels {
        let pixel_char = pixel_to_char(pixel);
        text_image_row.push(pixel_char);
        
        if row_counter == image_wrapper.width - 1 {
            text_image.push(text_image_row);
            text_image_row = Vec::new();
            row_counter = 0;
        } else {
            row_counter = row_counter + 1;
        }
    }
    
    text_image
}

pub trait ImageConverter {
    type ConvertsTo;
    fn convert(&mut self) -> Self::ConvertsTo;
}

pub struct ImageToTextConverter {
    pub image_wrapper: ImageWrapper,
}

impl ImageConverter for ImageToTextConverter {
    type ConvertsTo = String;

    fn convert(&mut self) -> Self::ConvertsTo {
        let mut image_buffer = String::new();
        let image = convert_to_2d_charmatrix(
            &mut self.image_wrapper, 
            ImageScaleOptions::default()
        );

        for row in image {
            for character in row {
                image_buffer.push(character);
            }
            image_buffer.push('\n');
        }

        image_buffer
    }
}

#[derive(Default)]
pub enum ImageScaleOptions {
    None,
    #[default]
    HalfHeight,
}



#[test]
fn pixel_to_char_test() {
    let pixel_1 = image::Rgb([0, 0, 0]);        
    let pixel_2 = image::Rgb([10, 10, 10]);     
    let pixel_3 = image::Rgb([70, 65, 80]);     
    let pixel_4 = image::Rgb([190, 170, 255]);  
    let pixel_5 = image::Rgb([230, 230, 230]);  
    let pixel_6 = image::Rgb([255, 255, 255]);
    
    let pixel_char_1 = pixel_to_char(&pixel_1);
    let pixel_char_2 = pixel_to_char(&pixel_2);
    let pixel_char_3 = pixel_to_char(&pixel_3);
    let pixel_char_4 = pixel_to_char(&pixel_4);
    let pixel_char_5 = pixel_to_char(&pixel_5);
    let pixel_char_6 = pixel_to_char(&pixel_6);
    
    assert_eq!(pixel_char_1, CHAR_MAPPING[0]);  // -> ' '
    assert_eq!(pixel_char_2, CHAR_MAPPING[0]);  // -> ' '
    assert_eq!(pixel_char_3, CHAR_MAPPING[2]);  // => ':'
    assert_eq!(pixel_char_4, CHAR_MAPPING[5]);  // -> 'X'
    assert_eq!(pixel_char_5, CHAR_MAPPING[7]);  // -> '@'
    assert_eq!(pixel_char_6, CHAR_MAPPING[7]);  // -> '@'
}

#[test]
fn e2e_image_conversion_test() {
    let path = "test/test_image.png";
    let red_pixel_indexes: Vec<u32> = (0..30).collect();
    let green_pixel_indexes: Vec<u32> = (30..70).collect();
    let blue_pixel_indexes: Vec<u32> = (70..100).collect();
    let white_pixel_index: u32 = 22;
    let black_pixel_index: u32 = 56;
    
    let mut image_wrapper = ImageWrapper::from_path(path).unwrap();
    let text_image = convert_to_2d_charmatrix(&mut image_wrapper, ImageScaleOptions::None);
    
    let mut char_counter: u32 = 0;
    for row in text_image {
        for character in row {
            if black_pixel_index == char_counter {
                assert_eq!(character, CHAR_MAPPING[0]); // 0 -> ' '
                
            } else if white_pixel_index == char_counter {
                assert_eq!(character, CHAR_MAPPING[7]); // 7 -> '@'
                
            } else if red_pixel_indexes.contains(&char_counter) {
                assert_eq!(character, CHAR_MAPPING[2]); // 2 => ':'
                
            } else if green_pixel_indexes.contains(&char_counter) {
                assert_eq!(character, CHAR_MAPPING[5]); // 5 -> 'X'
                
            } else if blue_pixel_indexes.contains(&char_counter) {
                assert_eq!(character, CHAR_MAPPING[0]); // 0 -> ' '
                
            } else {
                panic!("char index was out of range!");
            }
            
            char_counter = char_counter + 1;
        }
    }
}
