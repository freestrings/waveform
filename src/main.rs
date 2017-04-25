#[macro_use]
extern crate clap;
extern crate image;
extern crate imageproc;
extern crate simplemad;

use std::f64;
use std::i64;
use std::path::{Path, PathBuf};
use std::fs::{self, File};

use std::error::Error;

use clap::App;
use image::{Rgb, RgbImage};
use imageproc::{drawing, rect};
use simplemad::{Decoder, MadFixed32};

///https://en.wikipedia.org/wiki/Root_mean_square
///http://m.blog.naver.com/pkw00/220226903866
fn into_rms(samples: &Vec<MadFixed32>) -> f64 {
    let sum = samples.iter().fold(0.0, |_, sample| (sample.to_raw() as f64).powi(2));
    let len = samples.len() as f64;
    (sum / len).sqrt()
}

fn samples(file: File) -> Vec<f64> {
    Decoder::decode(file)
        .unwrap()
        .filter_map(|r| match r {
            Ok(f) => Some(f),
            _ => None,
        })
        .map(|f| [into_rms(&f.samples[0]), into_rms(&f.samples[1])]) //rms
        .map(|s| (s[0] + s[1]) / 2.0) //mono
        .collect()
}

fn hexadecimal_to_decimal(hex: &str) -> u8 {
    match i64::from_str_radix(hex, 16) {
        Ok(v) => v as u8,
        _ => 0,
    }
}

fn hexcolor_to_rgb(hex: &str) -> Rgb<u8> {
    let hex = if hex.starts_with("#") { &hex[1..] } else { hex };
    let r = hexadecimal_to_decimal(&hex[0..2]);
    let g = hexadecimal_to_decimal(&hex[2..4]);
    let b = hexadecimal_to_decimal(&hex[4..6]);
    Rgb([r, g, b])
}

fn build_output_dir(output: &str) {
    if output.is_empty() {
        return;
    }

    if Path::new(output).exists() {
        return;
    }

    match fs::create_dir_all(output) {
        Ok(_) => {}
        Err(_) => panic!("Cannot create the output directory: \"{}\"", output),
    };
}

fn target_path(output: &str, path: &str) -> String {
    if output.is_empty() {
        // create in the same directory of the path
        return format!("{}.png", path);
    }

    let mut path_buf = PathBuf::from(output);

    let file_name = match Path::new(path).file_name() {
        Some(file_name) => file_name.to_str().unwrap_or(""),
        None => "",
    };

    if file_name.is_empty() {
        return String::new();
    }

    path_buf.push(file_name);

    match path_buf.as_path().to_str() {
        Some(p) => format!("{}.png", p),
        None => String::new(),
    }
}

fn main() {

    let matches = App::new("waveform")
        .version("0.1")
        .author("Changseok Han <freestrings@gmail.com>")
        .args_from_usage(concat!("<INPUT>... 'mp3 file pathes. ex) ./waveform file1 file2'\n",
                                 // width
                                 "-w --width=[WIDTH] 'The image width. the default \
                                  width is 512 pixel'\n",
                                 // height
                                 "-h --height=[HEIGTH] 'The image height. the default \
                                  height is 120 pixel'\n",
                                 // background color
                                 "-b --background=[BACKGROUND] 'The background hex \
                                  color. the default value is #000000'\n",
                                 // foreground color
                                 "-f --foreground=[FOREGROUND] 'The foreground hex \
                                  color. the default value is #ffffff'\n",
                                 "-o --output=[OUTPUT] 'The output directory'\n",
                                 // verbose
                                 "-v --verbose 'Sets the level of verbosity'"))
        .get_matches();

    let image_width = value_t!(matches.value_of("width"), u32).unwrap_or(512 as u32);
    let image_height = value_t!(matches.value_of("height"), u32).unwrap_or(120 as u32);
    //
    // the color of canvas
    //
    let background = hexcolor_to_rgb(matches.value_of("background").unwrap_or("#000000"));
    //
    // the color of wave
    //
    let foreground = hexcolor_to_rgb(matches.value_of("foreground").unwrap_or("#ffffff"));
    //
    // the output path
    //
    let output = matches.value_of("output").unwrap_or("");
    build_output_dir(output);

    let pathes: Vec<_> = matches.values_of("INPUT").unwrap().collect();

    let mut count = 0;
    let total_pathes = pathes.len();

    for path in pathes {

        count += 1;

        match File::open(path) {
            Ok(file) => {
                let samples = samples(file);
                let max_sample = samples.iter().cloned().fold(f64::MIN, f64::max);

                let mut image = RgbImage::new(image_width, image_height);

                drawing::draw_filled_rect_mut(&mut image,
                                              rect::Rect::at(0_i32, 0_i32)
                                                  .of_size(image_width as u32,
                                                           image_height as u32),
                                              background);

                let sample_len = samples.len();
                for n in 0..sample_len {
                    let half_height = samples[n] * image_height as f64 / max_sample / 2_f64;
                    let x = (image_width as usize * n / sample_len) as i32;
                    let y = (image_height as f64 / 2_f64 - half_height) as i32;
                    let h = (half_height * 2_f64) as u32;
                    let h = if h <= 0 { 1 } else { h };

                    drawing::draw_filled_rect_mut(&mut image,
                                                  rect::Rect::at(x, y).of_size(1_u32, h),
                                                  foreground);

                }

                let target = target_path(output, path);

                if target.is_empty() {
                    println!("Target is empty {}/{} \"{}\"", count, total_pathes, path);
                    continue;
                }

                let result = image.save(&target);

                if matches.is_present("verbose") {
                    match result {
                        Ok(_) => {
                            println!("Done {}/{} \"{}\"", count, total_pathes, path);
                        }
                        Err(e) => {
                            println!("Save error {}/{} {}(\"{}\")",
                                     count,
                                     total_pathes,
                                     e.description(),
                                     target);
                        }
                    }
                }
            }

            Err(e) => {
                println!("Cannot open {}/{} {}(\"{}\")",
                         count,
                         total_pathes,
                         e.description(),
                         path);
            }
        }
    }

}


#[cfg(test)]
mod tests {

    use std::f64;
    use std::i64;

    #[test]
    fn max() {
        let vector = vec![0_f64, 1_f64, 2_f64];
        let max = vector.iter().cloned().fold(f64::MIN, f64::max);
        assert_eq!(max, 2_f64);
    }

    #[test]
    fn hex_to_dec() {
        assert_eq!(i64::from_str_radix("ff", 16).unwrap(), 255);
        assert_eq!(i64::from_str_radix("00", 16).unwrap(), 0);
        assert_eq!(i64::from_str_radix("01", 16).unwrap(), 1);
    }

    #[test]
    fn empty() {
        assert!("".is_empty())
    }

    #[test]
    fn target() {
        assert_eq!(//
                   super::target_path("", ""),
                   ".png".to_string());
        assert_eq!(//
                   super::target_path("", "/a/b.mp3"),
                   "/a/b.mp3.png".to_string());
        assert_eq!(//
                   super::target_path("", "a/b.mp3"),
                   "a/b.mp3.png".to_string());
        assert_eq!(//
                   super::target_path("output.png", "/a/b.mp3"),
                   "output.png/b.mp3.png".to_string());
        assert_eq!(//
                   super::target_path("/output", "/a/b.mp3"),
                   "/output/b.mp3.png".to_string());
        assert_eq!(//
                   super::target_path("/output/", "/a/b.mp3"),
                   "/output/b.mp3.png".to_string());
    }
}