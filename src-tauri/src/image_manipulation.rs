use std::error::Error;
use image::{
    self, 
    io::Reader as ImageReader, 
    ImageBuffer
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
    pub buffer: ImageBuffer<image::Rgb<f32>, Vec<f32>>,
    pub width: u32,
    pub height: u32,
}

impl Image for ImageWrapper {
    type Output = ImageWrapper;
    
    fn new(width: u32, height: u32) -> Self {
        let blank_image = ImageBuffer::from_pixel(width, height, image::Rgb([0.0, 0.0, 0.0]));
        
        ImageWrapper {
            buffer: blank_image,
            width,
            height,
        }
    }

    fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        let reader = ImageReader::open(path).unwrap();
        let image = reader.decode().unwrap();
        let rgb_image = image.as_rgb32f().unwrap();
        
        Ok(
            ImageWrapper {
                buffer: rgb_image.clone(),
                width: rgb_image.width(),
                height: rgb_image.height(),
            }
        )
    }

    fn dimentions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}


fn get_pixel_brightness(pixel: &image::Rgb<f32>) -> u32 {
    let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);
    let brightness = 0.2126 * red + 0.7152 * green + 0.0722 * blue;
    
    brightness.round() as u32
}

fn pixel_to_char(pixel: &image::Rgb<f32>) -> char {
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

pub fn convert_to_char_image(image_wrapper: ImageWrapper) -> Vec<Vec<char>> {
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
        }
        
        row_counter = row_counter + 1;
    }
    
    text_image
}



#[test]
fn pixel_to_char_test() {
    let pixel_1 = image::Rgb([0.0, 0.0, 0.0]);        
    let pixel_2 = image::Rgb([10.0, 10.0, 10.0]);     
    let pixel_3 = image::Rgb([70.0, 65.0, 80.0]);     
    let pixel_4 = image::Rgb([190.0, 170.0, 255.0]);  
    let pixel_5 = image::Rgb([230.0, 230.0, 230.0]);  
    let pixel_6 = image::Rgb([255.0, 255.0, 255.0]);
    
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
