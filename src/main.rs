
use opencv::{videoio, prelude::*, core, imgproc};
use gif::Encoder;
use std::fs::File;

fn main() {
    let mut video: videoio::VideoCapture = videoio::VideoCapture::from_file("kon - 1.mkv", videoio::CAP_ANY).unwrap();

    if !video.is_opened().unwrap() {
        panic!("Unable to open the video!");
    }

    let mut current_frame: Mat = Mat::default();
    let mut last_frame: Mat = Mat::default();
    let mut gif: Vec<Mat> = Vec::new();

    let width: u16 = video.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap() as u16;
    let height: u16 = video.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap() as u16;

    while video.read(&mut current_frame).unwrap() {
        if last_frame.empty() {
            last_frame = current_frame.clone();
        }
    
        let difference: f64 = calculate_difference(&current_frame, &last_frame).unwrap();
        println!("Difference: {:.2}, gif: {}", difference, gif.len());

        if difference > 85.0 && gif.len() > 24 {
            println!("Saving gif..." );

            let filename: String = format!("gifs/gif{}.gif", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

            let mut image: File = File::create(filename).unwrap();

            let mut encoder: Encoder<&mut File> = Encoder::new(&mut image, width, height , &[]).unwrap();
            for image in &gif {
                let mut image_better = Mat::default();

                imgproc::cvt_color(image, &mut image_better, imgproc::COLOR_BGR2RGB, 0).unwrap();

                let bytes = image_better.data_bytes().unwrap();

                let frame = gif::Frame::from_rgb_speed(width, height, &bytes, 30);

                encoder.write_frame(&frame).unwrap();
            }

            gif.clear();
            gif.push(current_frame.clone());
            continue;            
        }
        
        gif.push(current_frame.clone());
        last_frame = current_frame.clone();
    }
}


fn calculate_difference(current_frame: &Mat, last_frame: &Mat) -> Result<f64, opencv::Error> {
    let mut current_frame_gray: Mat = current_frame.clone();
    let mut last_frame_gray: Mat = last_frame.clone();

    imgproc::cvt_color(&current_frame, &mut current_frame_gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    imgproc::cvt_color(&last_frame, &mut last_frame_gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    let mut diff: Mat = Mat::default();

    opencv::core::absdiff(&current_frame_gray, &last_frame_gray, &mut diff).unwrap();

    let mut diff_u8: Mat = Mat::default();
    diff.convert_to(&mut diff_u8, core::CV_8U, 1.0, 0.0).unwrap();

    let non_zero_count: f64 = core::count_non_zero(&diff_u8).unwrap() as f64;
    
    let percentage: f64 = (non_zero_count * 100.0) / (diff_u8.size().unwrap().area() as f64);
    
    Ok(percentage)
}