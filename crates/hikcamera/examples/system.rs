use hikcamera::{Devices, HikCamera};

fn main() -> hikcamera::Result<()> {
    let hik = hik_camera()?;

    version(&hik);
    devices(&hik)?;

    Ok(())
}

fn hik_camera() -> hikcamera::Result<HikCamera> {
    let hik = HikCamera::new()?;

    println!("HikCamera::new()");
    println!("  initialized: true");

    Ok(hik)
}

fn version(hik: &HikCamera) {
    let version = hik.version();

    println!("HikCamera::version()");
    println!("  major: {:?}", version.major);
    println!("  minor: {:?}", version.minor);
    println!("  patch: {:?}", version.patch);
    println!("  build: {:?}", version.build);
    println!("  raw: {:?}", version.raw);
}

fn devices(hik: &HikCamera) -> hikcamera::Result<Devices<'_>> {
    let devices = hik.devices()?;

    println!("HikCamera::devices()");
    println!("  count: {:?}", devices.len());

    Ok(devices)
}
