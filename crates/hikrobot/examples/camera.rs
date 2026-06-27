use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};

use hikrobot::{Camera, HikRobot};

const DATA_DIR: &str = "crates/hikrobot/examples/datas";
const IMAGE_PATH: &str = "crates/hikrobot/examples/datas/image.bmp";
const VIDEO_PATH: &str = "crates/hikrobot/examples/datas/video.avi";

fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(DATA_DIR)?;

    let hik = HikRobot::new()?;
    let mut camera = camera(&hik)?;

    configure(&mut camera)?;

    let camera = take_image(camera)?;
    let camera = take_video(camera)?;

    camera.close()?;

    Ok(())
}

fn camera(hik: &HikRobot) -> hikrobot::Result<Camera<'_>> {
    let camera = hik.devices()?.default()?.open()?;

    println!("Device::open()");
    println!("  opened: true");

    Ok(camera)
}

fn configure(camera: &mut Camera<'_>) -> hikrobot::Result<()> {
    camera.set_exposure(8000.0)?;
    camera.set_gain(3.0)?;

    println!("Camera parameters");
    println!("  exposure: {:?}", camera.get_exposure()?.current);
    println!("  gain: {:?}", camera.get_gain()?.current);

    Ok(())
}

fn take_image(camera: Camera<'_>) -> hikrobot::Result<Camera<'_>> {
    let mut stream = camera.stream()?;
    let timeout = Duration::from_secs(1);

    let frame = stream.take_frame(timeout)?;
    let mut image = stream.save_image(Path::new(IMAGE_PATH))?;
    let image_path = image.path().to_owned();
    image.write_frame(&frame)?;
    image.finish()?;

    println!("Stream::take_frame()");
    println!("  image_path: {:?}", image_path);
    println!("  width: {:?}", frame.info.width);
    println!("  height: {:?}", frame.info.height);
    println!("  bytes: {:?}", frame.data.len());

    stream.stop()
}

fn take_video(camera: Camera<'_>) -> hikrobot::Result<Camera<'_>> {
    let mut stream = camera.stream()?;
    let timeout = Duration::from_secs(1);
    let mut video = stream.save_video(Path::new(VIDEO_PATH), 30.0)?;
    let started = Instant::now();

    while started.elapsed() < Duration::from_secs(10) {
        let frame = stream.take_frame(timeout)?;
        video.write_frame(&frame)?;
    }

    let video = video.finish()?;

    println!("Stream::save_video()");
    println!("  path: {:?}", video.path);
    println!("  frame_count: {:?}", video.frame_count);
    println!("  frame_rate: {:?}", video.frame_rate);
    println!("  width: {:?}", video.width);
    println!("  height: {:?}", video.height);
    println!("  pixel_type: {:?}", video.pixel_type);
    println!("  elapsed: {:?}", video.elapsed);

    stream.stop()
}
