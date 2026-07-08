use hikcamera::HikCamera;

fn main() -> hikcamera::Result<()> {
    let hik = hikcamera()?;

    version(&hik);

    Ok(())
}

fn hikcamera() -> hikcamera::Result<HikCamera> {
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

    assert_ne!(version.raw, 0);
}
