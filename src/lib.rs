use std::{error::Error, fs::File, path::Path};
use gif::Encoder;
use opencv::{videoio, prelude::*, core, imgproc};
use clap::Parser;

/// Generate GIFs of animes!
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The threshold for determining whether a scene constitutes a transition or not.
    #[arg(long, default_value_t = 85.0)]
    threshold: f64,

    /// The path to the file
    #[arg(long)]
    filepath: String,

    /// The width of the GIF
    #[arg(long, default_value_t = 500)]
    width: u32,

    /// The height of the GIF
    #[arg(long, default_value_t = 270)]
    height: u32,

    /// Whether to print the difference between frames
    #[arg(long, default_value_t = false)]
    printdiff: bool
}

struct Frame {
    data: Mat,
    count: i32
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut video = open_video(&config.filepath)?;

    let mut current_frame: Mat = Mat::default();
    let mut last_frame: Mat = Mat::default();
    let mut gif: Vec<Frame> = Vec::new();

    while video.read(&mut current_frame).unwrap() {
        if last_frame.empty() {
            last_frame = current_frame.clone();
            continue;
        }

        let frame_count: i32 =  video.get(videoio::CAP_PROP_POS_FRAMES).unwrap() as i32;

        let frame = Frame{data: current_frame.clone(), count: frame_count};

        let difference: f64 = calculate_difference(&current_frame, &last_frame, config.printdiff).unwrap();

        if difference > config.threshold && gif.len() > 24 {
            println!("Saving gif..." );

            gif = create_gif(gif, &config).unwrap();

            gif.clear();
            gif.push(frame);

            println!("Gif saved!" );
            continue;  
        }
        
        gif.push(frame);
        last_frame = current_frame.clone();
    }

    Ok(())
}

fn open_video(path: &String) -> Result<videoio::VideoCapture, Box<dyn Error>> {
    if !Path::new(path).exists() { return Err("The provided file does not exist".into())};

    let video: videoio::VideoCapture = videoio::VideoCapture::from_file(path, videoio::CAP_ANY)?;

    if !video.is_opened()? { return Err("The provided file could not be opened".into())};

    Ok(video)
}

fn calculate_difference(current_frame: &Mat, last_frame: &Mat, printdff: bool) -> Result<f64, opencv::Error> {
    let mut current_frame_gray: Mat = current_frame.clone();
    let mut last_frame_gray: Mat = last_frame.clone();

    imgproc::cvt_color(&current_frame, &mut current_frame_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    imgproc::cvt_color(&last_frame, &mut last_frame_gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut diff: Mat = Mat::default();

    opencv::core::absdiff(&current_frame_gray, &last_frame_gray, &mut diff)?;

    let mut diff_u8: Mat = Mat::default();
    diff.convert_to(&mut diff_u8, core::CV_8U, 1.0, 0.0)?;

    let non_zero_count: f64 = core::count_non_zero(&diff_u8)? as f64;
    
    let percentage: f64 = (non_zero_count * 100.0) / (diff_u8.size()?.area() as f64);
    
    if printdff { println!("Difference between frames is: {:.2}%", percentage) }

    Ok(percentage)
}

fn create_gif(gif: Vec<Frame>, config: &Config) -> Result<Vec<Frame>, Box<dyn Error>>{
    let mut file = create_file(&gif)?;

    let mut encoder: Encoder<&mut File> = Encoder::new(&mut file, config.width  as u16, config.height as u16, &[]).unwrap();
    
    for frame in &gif {
        let optimized_frame: Mat = optimize_frame(&frame.data, &config.width, &config.height)?;	
        
        let image_bytes: &[u8] = optimized_frame.data_bytes()?;

        let frame2: gif::Frame<'_> = gif::Frame::from_rgb_speed(config.width as u16, config.height as u16, &image_bytes, 10);

        encoder.write_frame(&frame2)?; 
    }

    Ok(gif)
}

fn create_file(gif: &Vec<Frame>) -> Result<File, std::io::Error>{
    let filename: String = format!("gifs/gif [from {} to {}].gif", gif.first().unwrap().count, gif.last().unwrap().count);

    let file: File = File::create(filename)?;

    Ok(file)
}

fn optimize_frame(frame: &Mat, width: &u32, height: &u32) -> Result<opencv::prelude::Mat, opencv::Error> {
    let mut best_color_frame: Mat = Mat::default();

    imgproc::cvt_color(frame, &mut best_color_frame, imgproc::COLOR_BGR2RGB, 0)?;

    let mut resized_frame: Mat = Mat::default();

    imgproc::resize(&best_color_frame, &mut resized_frame, core::Size::new(*width as i32, *height as i32), 0.0, 0.0, imgproc::INTER_LINEAR)?;

    Ok(resized_frame)
}