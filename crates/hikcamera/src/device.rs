use std::fmt;
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};

use crate::{Camera, HikCamera, HikCameraError, Result, error::check, sys};

#[derive(Clone)]
pub struct Devices<'hik> {
    items: Vec<Device<'hik>>,
}

#[derive(Clone)]
pub struct Device<'hik> {
    _hik: &'hik HikCamera,
    raw: sys::MV_CC_DEVICE_INFO,
    info: DeviceInfo,
}

impl fmt::Debug for Devices<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Devices")
            .field("items", &self.items)
            .finish()
    }
}

impl fmt::Debug for Device<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Device")
            .field("info", &self.info)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub major_version: u16,
    pub minor_version: u16,

    pub transport: Transport,
    pub device_type: u32,

    pub model: Option<String>,
    pub serial: Option<String>,
    pub user_name: Option<String>,
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub family: Option<String>,

    pub device_id: Option<String>,
    pub interface_id: Option<String>,
    pub mac: Option<String>,

    pub ip: Option<String>,
    pub subnet: Option<String>,
    pub gateway: Option<String>,
    pub net_export: Option<String>,
    pub host_ip: Option<String>,
    pub multicast_ip: Option<String>,
    pub multicast_port: Option<u32>,

    pub ip_config: Option<IpConfig>,
    pub usb: Option<UsbInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    GigE,
    Usb,
    CameraLink,
    Ieee1394,
    VirtualGigE,
    VirtualUsb,
    GenTlGigE,
    GenTlCameraLink,
    GenTlCoaXPress,
    GenTlXoF,
    GenTlVirtual,
    Other(u32),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpConfig {
    pub option: u32,
    pub current: u32,
    pub gen_tl_type: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsbInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_number: u32,
    pub device_address: u32,
    pub usb_version: u32,
    pub control_in: u8,
    pub control_out: u8,
    pub stream: u8,
    pub event: u8,
}

impl<'hik> Devices<'hik> {
    pub(crate) fn list(hik: &'hik HikCamera) -> Result<Self> {
        let mut list = MaybeUninit::<sys::MV_CC_DEVICE_INFO_LIST>::zeroed();
        let types = sys::MV_GIGE_DEVICE | sys::MV_USB_DEVICE;

        check(unsafe { sys::MV_CC_EnumDevices(types, list.as_mut_ptr()) })?;

        Ok(Self::from_raw_list(hik, unsafe { list.assume_init() }))
    }

    fn from_raw_list(hik: &'hik HikCamera, list: sys::MV_CC_DEVICE_INFO_LIST) -> Self {
        let count = list.nDeviceNum.min(list.pDeviceInfo.len() as u32) as usize;
        let items = list
            .pDeviceInfo
            .iter()
            .take(count)
            .filter_map(|raw| {
                if raw.is_null() {
                    return None;
                }

                let raw = unsafe { **raw };
                Some(Device::from_raw(hik, raw))
            })
            .collect();

        Self { items }
    }

    // List access.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Device<'hik>> {
        self.items.iter()
    }

    pub fn as_slice(&self) -> &[Device<'hik>] {
        &self.items
    }

    pub fn get(&self, index: usize) -> Option<&Device<'hik>> {
        self.items.get(index)
    }

    pub fn default(mut self) -> Result<Device<'hik>> {
        if self.items.is_empty() {
            return Err(HikCameraError::NoDevice);
        }

        Ok(self.items.remove(0))
    }

    pub fn serial_number(self, value: &str) -> Result<Device<'hik>> {
        self.select_one(format!("serial_number={value}"), |device| {
            device.info.serial.as_deref() == Some(value)
        })
    }

    pub fn user_name(self, value: &str) -> Result<Device<'hik>> {
        self.select_one(format!("user_name={value}"), |device| {
            device.info.user_name.as_deref() == Some(value)
        })
    }

    pub fn ip(self, value: &str) -> Result<Device<'hik>> {
        self.select_one(format!("ip={value}"), |device| {
            device.info.ip.as_deref() == Some(value)
        })
    }

    pub fn mac(self, value: &str) -> Result<Device<'hik>> {
        self.select_one(format!("mac={value}"), |device| {
            device
                .info
                .mac
                .as_deref()
                .is_some_and(|mac| mac.eq_ignore_ascii_case(value))
        })
    }

    fn select_one(
        self,
        selector: String,
        mut matches: impl FnMut(&Device<'hik>) -> bool,
    ) -> Result<Device<'hik>> {
        let mut matched = self.items.into_iter().filter(|device| matches(device));
        let Some(device) = matched.next() else {
            return Err(HikCameraError::DeviceNotFound { selector });
        };

        let extra = matched.count();
        if extra > 0 {
            return Err(HikCameraError::MultipleDevices {
                selector,
                count: extra + 1,
            });
        }

        Ok(device)
    }
}

impl<'hik> IntoIterator for Devices<'hik> {
    type Item = Device<'hik>;
    type IntoIter = std::vec::IntoIter<Device<'hik>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'items, 'hik> IntoIterator for &'items Devices<'hik> {
    type Item = &'items Device<'hik>;
    type IntoIter = std::slice::Iter<'items, Device<'hik>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'hik> Device<'hik> {
    fn from_raw(hik: &'hik HikCamera, raw: sys::MV_CC_DEVICE_INFO) -> Self {
        Self {
            _hik: hik,
            info: DeviceInfo::from_raw(&raw),
            raw,
        }
    }

    pub fn info(&self) -> &DeviceInfo {
        &self.info
    }

    pub fn is_accessible(&self) -> bool {
        let mut raw = self.raw;
        unsafe { sys::MV_CC_IsDeviceAccessible(&mut raw, sys::MV_ACCESS_Exclusive) != 0 }
    }

    pub fn open(self) -> Result<Camera<'hik>> {
        let mut handle = ptr::null_mut();
        check(unsafe { sys::MV_CC_CreateHandle(&mut handle, self.raw()) })?;

        let Some(handle) = NonNull::new(handle.cast()) else {
            return Err(HikCameraError::NullHandle);
        };

        if let Err(error) =
            check(unsafe { sys::MV_CC_OpenDevice(handle.as_ptr(), sys::MV_ACCESS_Exclusive, 0) })
        {
            unsafe {
                sys::MV_CC_DestroyHandle(handle.as_ptr());
            }
            return Err(error);
        }

        Ok(Camera::from_handle(handle))
    }

    pub(crate) fn raw(&self) -> &sys::MV_CC_DEVICE_INFO {
        &self.raw
    }
}

impl DeviceInfo {
    fn from_raw(raw: &sys::MV_CC_DEVICE_INFO) -> Self {
        let transport = Transport::from_raw(raw.nTLayerType);

        match transport {
            Transport::GigE | Transport::VirtualGigE | Transport::GenTlGigE => {
                let info = unsafe { raw.SpecialInfo.stGigEInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: text(&info.chUserDefinedName),
                    vendor: text(&info.chManufacturerName),
                    version: text(&info.chDeviceVersion),
                    family: None,
                    device_id: None,
                    interface_id: None,
                    mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                    ip: Some(format_ip(info.nCurrentIp)),
                    subnet: Some(format_ip(info.nCurrentSubNetMask)),
                    gateway: Some(format_ip(info.nDefultGateWay)),
                    net_export: Some(format_ip(info.nNetExport)),
                    host_ip: Some(format_ip(info.nHostIP)),
                    multicast_ip: Some(format_ip(info.nMulticastIP)),
                    multicast_port: Some(info.nMulticastPort),
                    ip_config: Some(IpConfig {
                        option: info.nIpCfgOption,
                        current: info.nIpCfgCurrent,
                        gen_tl_type: info.nGenTLType,
                    }),
                    usb: None,
                }
            }
            Transport::Usb | Transport::VirtualUsb => {
                let info = unsafe { raw.SpecialInfo.stUsb3VInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: text(&info.chUserDefinedName),
                    vendor: text(&info.chManufacturerName).or_else(|| text(&info.chVendorName)),
                    version: text(&info.chDeviceVersion),
                    family: text(&info.chFamilyName),
                    device_id: text(&info.chDeviceGUID),
                    interface_id: None,
                    mac: None,
                    ip: None,
                    subnet: None,
                    gateway: None,
                    net_export: None,
                    host_ip: None,
                    multicast_ip: None,
                    multicast_port: None,
                    ip_config: None,
                    usb: Some(UsbInfo {
                        vendor_id: info.idVendor,
                        product_id: info.idProduct,
                        device_number: info.nDeviceNumber,
                        device_address: info.nDeviceAddress,
                        usb_version: info.nbcdUSB,
                        control_in: info.CrtlInEndPoint,
                        control_out: info.CrtlOutEndPoint,
                        stream: info.StreamEndPoint,
                        event: info.EventEndPoint,
                    }),
                }
            }
            Transport::CameraLink => {
                let info = unsafe { raw.SpecialInfo.stCamLInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: None,
                    vendor: text(&info.chManufacturerName),
                    version: text(&info.chDeviceVersion),
                    family: text(&info.chFamilyName),
                    device_id: text(&info.chPortID),
                    interface_id: text(&info.chPortID),
                    mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                    ip: None,
                    subnet: None,
                    gateway: None,
                    net_export: None,
                    host_ip: None,
                    multicast_ip: None,
                    multicast_port: None,
                    ip_config: None,
                    usb: None,
                }
            }
            Transport::GenTlCameraLink => {
                let info = unsafe { raw.SpecialInfo.stCMLInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: text(&info.chUserDefinedName),
                    vendor: text(&info.chVendorName),
                    version: text(&info.chDeviceVersion),
                    family: None,
                    device_id: text(&info.chDeviceID),
                    interface_id: text(&info.chInterfaceID),
                    mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                    ip: None,
                    subnet: None,
                    gateway: None,
                    net_export: None,
                    host_ip: None,
                    multicast_ip: None,
                    multicast_port: None,
                    ip_config: None,
                    usb: None,
                }
            }
            Transport::GenTlCoaXPress => {
                let info = unsafe { raw.SpecialInfo.stCXPInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: text(&info.chUserDefinedName),
                    vendor: text(&info.chVendorName),
                    version: text(&info.chDeviceVersion),
                    family: None,
                    device_id: text(&info.chDeviceID),
                    interface_id: text(&info.chInterfaceID),
                    mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                    ip: None,
                    subnet: None,
                    gateway: None,
                    net_export: None,
                    host_ip: None,
                    multicast_ip: None,
                    multicast_port: None,
                    ip_config: None,
                    usb: None,
                }
            }
            Transport::GenTlXoF => {
                let info = unsafe { raw.SpecialInfo.stXoFInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: text(&info.chUserDefinedName),
                    vendor: text(&info.chVendorName),
                    version: text(&info.chDeviceVersion),
                    family: None,
                    device_id: text(&info.chDeviceID),
                    interface_id: text(&info.chInterfaceID),
                    mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                    ip: None,
                    subnet: None,
                    gateway: None,
                    net_export: None,
                    host_ip: None,
                    multicast_ip: None,
                    multicast_port: None,
                    ip_config: None,
                    usb: None,
                }
            }
            Transport::GenTlVirtual => {
                let info = unsafe { raw.SpecialInfo.stVirInfo };
                Self {
                    major_version: raw.nMajorVer,
                    minor_version: raw.nMinorVer,
                    transport,
                    device_type: raw.nDevTypeInfo,
                    model: text(&info.chModelName),
                    serial: text(&info.chSerialNumber),
                    user_name: text(&info.chUserDefinedName),
                    vendor: text(&info.chVendorName),
                    version: text(&info.chDeviceVersion),
                    family: text(&info.chTLType),
                    device_id: text(&info.chDeviceID),
                    interface_id: text(&info.chInterfaceID),
                    mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                    ip: None,
                    subnet: None,
                    gateway: None,
                    net_export: None,
                    host_ip: None,
                    multicast_ip: None,
                    multicast_port: None,
                    ip_config: None,
                    usb: None,
                }
            }
            _ => Self {
                major_version: raw.nMajorVer,
                minor_version: raw.nMinorVer,
                transport,
                device_type: raw.nDevTypeInfo,
                model: None,
                serial: None,
                user_name: None,
                vendor: None,
                version: None,
                family: None,
                device_id: None,
                interface_id: None,
                mac: Some(format_mac(raw.nMacAddrHigh, raw.nMacAddrLow)),
                ip: None,
                subnet: None,
                gateway: None,
                net_export: None,
                host_ip: None,
                multicast_ip: None,
                multicast_port: None,
                ip_config: None,
                usb: None,
            },
        }
    }
}

impl Transport {
    fn from_raw(value: u32) -> Self {
        match value {
            sys::MV_UNKNOW_DEVICE => Self::Unknown,
            sys::MV_GIGE_DEVICE => Self::GigE,
            sys::MV_1394_DEVICE => Self::Ieee1394,
            sys::MV_USB_DEVICE => Self::Usb,
            sys::MV_CAMERALINK_DEVICE => Self::CameraLink,
            sys::MV_VIR_GIGE_DEVICE => Self::VirtualGigE,
            sys::MV_VIR_USB_DEVICE => Self::VirtualUsb,
            sys::MV_GENTL_GIGE_DEVICE => Self::GenTlGigE,
            sys::MV_GENTL_CAMERALINK_DEVICE => Self::GenTlCameraLink,
            sys::MV_GENTL_CXP_DEVICE => Self::GenTlCoaXPress,
            sys::MV_GENTL_XOF_DEVICE => Self::GenTlXoF,
            sys::MV_GENTL_VIR_DEVICE => Self::GenTlVirtual,
            other => Self::Other(other),
        }
    }
}
fn text<const N: usize>(bytes: &[u8; N]) -> Option<String> {
    let bytes = bytes.as_slice();
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    let value = String::from_utf8_lossy(&bytes[0..end]).trim().to_owned();

    if value.is_empty() { None } else { Some(value) }
}

fn format_ip(value: u32) -> String {
    value.to_be_bytes().map(|part| part.to_string()).join(".")
}

fn format_mac(high: u32, low: u32) -> String {
    let value = ((high as u64) << 32) | low as u64;
    let bytes = value.to_be_bytes();
    bytes[2..]
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(":")
}
