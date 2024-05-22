//use std::{error::Error, ptr::NonNull};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::{thread, time};
use fontdue;

const LENGTH: usize = 16;
const WIDTH: usize = 16;
const BYTES_PER_LED: usize = 3;
const _SOF: u8 = 0x72; 

const _PATH: &[u8] = include_bytes!("/usr/share/fonts/truetype/freefont/FreeSansBold.ttf") as &[u8];

pub fn generate_font(size: f32) -> fontdue::Font{
    // Setup the configuration for how the font will be parsed.
    let settings = fontdue::FontSettings {
        scale: size,
        ..fontdue::FontSettings::default()
    };

    //  Get font structure for future configuration
    let font = fontdue::Font::from_bytes(_PATH, settings).unwrap();
    return font;
}

pub struct Matrix{
    _spi: Spi, 
    data_arr: [u8; LENGTH * WIDTH * BYTES_PER_LED + 1],
    size: f32,
    font: fontdue::Font
}

impl Matrix{
    pub fn update(&mut self){
        match self._spi.write(&self.data_arr){
            Ok(_s)=> return, 
            Err(_e)=> panic!("Could not transfer buffers through spi!")
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8){
        // Out of bounds for our matrices
        if (x >= 16) | (y >= 16){
            return; 
        }
        let pos = ((16 * y + x) as usize) * 3 + 1; 

        self.data_arr[pos] = r; 
        self.data_arr[pos + 1] = g; 
        self.data_arr[pos + 2] = b; 
    }

    pub fn _print_char(&mut self, character: char, x_offset: u8, y_offset: u8) -> usize{
        let (metrics, bitmap) = self.font.rasterize_subpixel(character, self.size);
        let width = metrics.width; 
        let length = metrics.height; 

        for y in 0..length {
            for x in 0..width  {
                // Bitmap generates RGB.. we are greyscale for now so we only need this
                let val = bitmap[x*3 + y * width*3];
                self.set_pixel(x as u32 + x_offset as u32, y as u32 + y_offset as u32, val, val, val);
            }
        }

        return width; 
    }

    pub fn print_char(&mut self, character: char, x_offset: u8, y_offset: u8){
        let _n = self._print_char(character, x_offset, y_offset); 
    }

    pub fn clear_matrix(&mut self){
        for n in 0..(LENGTH * WIDTH * BYTES_PER_LED){
            self.data_arr[n + 1] = 0; 
        }
    }

    pub fn set_matrix(&mut self, r: u8, g: u8, b: u8){
        for y in 0..LENGTH {
            for x in 0..WIDTH  {
                self.set_pixel(x as u32, y as u32, r, g, b);
            }
        }

        self.update();
    }

    pub fn print_string(&mut self, string: String){
        
        let mut x_offset: usize = 0; 
        let x_width: usize = 11 * string.chars().count() as usize; 
        let y_width: usize = 12;
        let mut string_map_arr = vec![vec![0; x_width]; y_width];
    
        // Font Rending into Bitmap used for animations 
        for character in string.chars(){
            if character == ' ' {
                x_offset = x_offset + 10; 
            }
            else{
                let (metrics, bitmap) = self.font.rasterize_subpixel(character, self.size);
                for y in 0..metrics.height {
                    let y_offset = y + (y_width - metrics.height); 
                    for x in 0..metrics.width {
                        // Bitmap generates RGB.. we are greyscale for now so we only need this
                        let val = bitmap[x*3 + y * metrics.width*3];
                        string_map_arr[y_offset][x + x_offset] = val; 
                    }
                }
                x_offset = x_offset + metrics.width + 1; 
            }
        }
    

        for offset in -12..(x_width as i32){
            self.clear_matrix();
            // Render matrix frame, not entire bitmap. 
            for y in 0..y_width{
                for x in 0..16{
                    let x_map_pos = x + offset as i32;
                    // Ensure no out of bounds array accessing.
                    if (x_map_pos < x_width as i32) && (x_map_pos > 0) {
                        let val = string_map_arr[y][x_map_pos as usize];
                        self.set_pixel(x as u32, y as u32, val, val, val);
                    }
                    // Out of bounds rending is null pixel. 
                    else{
                        self.set_pixel(x as u32, y as u32, 0, 0, 0);
                    }
                }
            }
            self.update();
            let ten_millis = time::Duration::from_millis(27);
            thread::sleep(ten_millis);
        }
    }

}

pub fn matrix_setup(font_size: f32)-> Matrix{
    let mut _spi: Spi; 
    let mut _matrix: Matrix; 
    let font = generate_font(font_size); 
    // TODO: Test 10MHZ spi speed. 
    match Spi::new(Bus::Spi0, SlaveSelect::Ss0, 10_000_000, Mode::Mode0){
        Ok(_s)=> {
            _spi = _s; 
            _matrix = Matrix{
                _spi: _spi, 
                data_arr: [0; (LENGTH * WIDTH * BYTES_PER_LED + 1)], 
                size: font_size,
                font 
            }; 
            _matrix.data_arr[0] = _SOF; 
            return _matrix
        }, 
        Err(_e)=> {
            panic!("Could not setup SPI, shutting program down")
        }
    }
}