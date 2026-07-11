use hikcamera::{HikCamera, ShowExt, ShowOptions};

fn main() -> hikcamera::ShowResult<()> {
    let hik = HikCamera::new()?;
    let mut camera = hik.devices()?.default()?.open()?;

    camera.set_exposure(20000.0)?;

    let stream = camera.stream()?;
    let stream = stream
        .show_with(ShowOptions::new().window_size(1000, 750))?
        .run()?;
    let camera = stream.stop()?;
    camera.close()?;

    Ok(())
}
