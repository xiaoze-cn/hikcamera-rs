use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};

use hikcamera::{Camera, Device, HikCamera};

const DATA_DIR: &str = "crates/hikcamera/examples/datas";
const IMAGE_PATH: &str = "crates/hikcamera/examples/datas/image.bmp";
const VIDEO_PATH: &str = "crates/hikcamera/examples/datas/video.avi";
const FRAME_TIMEOUT: Duration = Duration::from_secs(1);
const VIDEO_DURATION: Duration = Duration::from_secs(2);

fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(DATA_DIR)?;

    let hik = HikCamera::new()?;
    let device = device(&hik)?;
    let mut camera = camera(device)?;

    configure(&mut camera)?;

    let camera = take_image(camera)?;
    let camera = take_video(camera)?;

    camera.close()?;

    Ok(())
}

fn device(hik: &HikCamera) -> hikcamera::Result<Device<'_>> {
    let devices = hik.devices()?;
    let count = devices.len();

    println!("HikCamera::devices()");
    println!("  count: {count}");

    let device = devices.default()?;

    println!("Devices::default()");
    println!("  selected: {:?}", device.info().serial);

    Ok(device)
}

fn camera(device: Device<'_>) -> hikcamera::Result<Camera<'_>> {
    let camera = device.open()?;

    println!("Device::open()");
    println!("  opened: true");

    Ok(camera)
}

fn configure(camera: &mut Camera<'_>) -> hikcamera::Result<()> {
    camera.set_exposure(8000.0)?;
    camera.set_gain(3.0)?;

    println!("Camera parameters");
    println!("  exposure: {:?}", camera.get_exposure()?.current);
    println!("  gain: {:?}", camera.get_gain()?.current);

    Ok(())
}

fn take_image(camera: Camera<'_>) -> hikcamera::Result<Camera<'_>> {
    let mut stream = camera.stream()?;

    let frame = stream.take_frame(FRAME_TIMEOUT)?;
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

fn take_video(camera: Camera<'_>) -> hikcamera::Result<Camera<'_>> {
    let mut stream = camera.stream()?;
    let mut video = stream.save_video(Path::new(VIDEO_PATH), 30.0)?;
    let started = Instant::now();

    while started.elapsed() < VIDEO_DURATION {
        let frame = stream.take_frame(FRAME_TIMEOUT)?;
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
