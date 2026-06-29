use hikcamera::{Device, Devices, HikCamera};

fn main() -> hikcamera::Result<()> {
    let hik = HikCamera::new()?;

    let devices = devices(&hik)?;

    for (index, device) in devices.iter().enumerate() {
        info(index, device);
    }

    Ok(())
}

fn devices(hik: &HikCamera) -> hikcamera::Result<Devices<'_>> {
    let devices = hik.devices()?;

    println!("HikCamera::devices()");
    println!("  count: {}", devices.len());

    Ok(devices)
}

fn info(index: usize, device: &Device<'_>) {
    let info = device.info();

    println!("Device::info() #{index}");
    println!("  accessible: {:?}", device.is_accessible());
    println!("  major_version: {:?}", info.major_version);
    println!("  minor_version: {:?}", info.minor_version);
    println!("  transport: {:?}", info.transport);
    println!("  device_type: {:?}", info.device_type);
    println!("  model: {:?}", info.model);
    println!("  serial: {:?}", info.serial);
    println!("  user_name: {:?}", info.user_name);
    println!("  vendor: {:?}", info.vendor);
    println!("  version: {:?}", info.version);
    println!("  family: {:?}", info.family);
    println!("  device_id: {:?}", info.device_id);
    println!("  interface_id: {:?}", info.interface_id);
    println!("  mac: {:?}", info.mac);
    println!("  ip: {:?}", info.ip);
    println!("  subnet: {:?}", info.subnet);
    println!("  gateway: {:?}", info.gateway);
    println!("  net_export: {:?}", info.net_export);
    println!("  host_ip: {:?}", info.host_ip);
    println!("  multicast_ip: {:?}", info.multicast_ip);
    println!("  multicast_port: {:?}", info.multicast_port);
    println!("  ip_config: {:?}", info.ip_config);
    println!("  usb: {:?}", info.usb);
}
