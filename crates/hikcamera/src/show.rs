use std::time::Duration;

use crate::Stream;

pub const DEFAULT_TITLE: &str = "HikCamera Show";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

const DEFAULT_WIDTH: i32 = 1000;
const DEFAULT_HEIGHT: i32 = 750;

pub type ShowResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct ShowOptions {
    title: String,
    width: i32,
    height: i32,
    timeout: Duration,
}

#[derive(Debug)]
pub struct StreamShow<'hik> {
    stream: Stream<'hik>,
    options: ShowOptions,
}

pub trait ShowExt<'hik>: Sized {
    type Show;

    fn show(self) -> ShowResult<Self::Show>;
    fn show_with(self, options: ShowOptions) -> ShowResult<Self::Show>;
}

impl<'hik> ShowExt<'hik> for Stream<'hik> {
    type Show = StreamShow<'hik>;

    fn show(self) -> ShowResult<Self::Show> {
        self.show_with(ShowOptions::default())
    }

    fn show_with(self, options: ShowOptions) -> ShowResult<Self::Show> {
        Ok(StreamShow {
            stream: self,
            options,
        })
    }
}

impl<'hik> StreamShow<'hik> {
    pub fn run(self) -> ShowResult<Stream<'hik>> {
        run_show(self.stream, self.options)
    }
}

impl ShowOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn window_size(mut self, width: i32, height: i32) -> Self {
        self.width = width.max(1);
        self.height = height.max(1);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl Default for ShowOptions {
    fn default() -> Self {
        Self {
            title: DEFAULT_TITLE.to_owned(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

#[cfg(windows)]
fn run_show(stream: Stream<'_>, options: ShowOptions) -> ShowResult<Stream<'_>> {
    windows::run_show(stream, options)
}

#[cfg(not(windows))]
fn run_show(stream: Stream<'_>, _options: ShowOptions) -> ShowResult<Stream<'_>> {
    Err("hikcamera show is only implemented on Windows".into())
}

#[cfg(windows)]
mod windows {
    use std::ptr;

    use crate::{Frame, HikCameraError, Status, Stream, sys};
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow,
        DispatchMessageW, IDC_ARROW, IsWindow, LoadCursorW, MSG, PM_REMOVE, PeekMessageW,
        RegisterClassW, TranslateMessage, WM_DESTROY, WM_KEYDOWN, WM_QUIT, WNDCLASSW,
        WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    };

    use super::{ShowOptions, ShowResult};

    const RENDER_MODE_GDI: u32 = 0;
    const RENDER_MODE_D3D: u32 = 1;
    const PIXEL_BGR8: u32 = sys::MvGvspPixelType_PixelType_Gvsp_BGR8_Packed as u32;

    pub(super) fn run_show(mut stream: Stream<'_>, options: ShowOptions) -> ShowResult<Stream<'_>> {
        let window = Window::new(&options.title, options.width, options.height)?;
        let mut choice: Option<DisplayChoice> = Option::None;

        while window.is_open() {
            let frame = stream.take_frame(options.timeout)?;

            let selected = match choice {
                Option::Some(choice) => choice,
                Option::None => {
                    let selected = select_path(&stream, window.hwnd, &frame)?;
                    choice = Option::Some(selected);
                    selected
                }
            };

            let converted;
            let display_frame = match selected.source {
                DisplaySource::Raw => &frame,
                DisplaySource::Bgr8 => {
                    converted = stream.convert_frame(&frame, PIXEL_BGR8)?;
                    &converted
                }
            };

            show_frame(&stream, window.hwnd, display_frame, selected.render_mode)?;

            window.pump_messages();
        }

        Ok(stream)
    }

    fn select_path(stream: &Stream<'_>, hwnd: HWND, frame: &Frame) -> ShowResult<DisplayChoice> {
        let candidates = [
            DisplayChoice {
                source: DisplaySource::Raw,
                render_mode: RENDER_MODE_D3D,
            },
            DisplayChoice {
                source: DisplaySource::Raw,
                render_mode: RENDER_MODE_GDI,
            },
            DisplayChoice {
                source: DisplaySource::Bgr8,
                render_mode: RENDER_MODE_D3D,
            },
            DisplayChoice {
                source: DisplaySource::Bgr8,
                render_mode: RENDER_MODE_GDI,
            },
        ];

        for candidate in candidates {
            let converted;
            let display_frame = match candidate.source {
                DisplaySource::Raw => frame,
                DisplaySource::Bgr8 => {
                    converted = stream.convert_frame(frame, PIXEL_BGR8)?;
                    &converted
                }
            };

            let result = show_frame(stream, hwnd, display_frame, candidate.render_mode);
            match result {
                Ok(()) => return Ok(candidate),
                Err(error) => println!("display path failed: {}: {error}", candidate.label()),
            }
        }

        Err("no SDK display path worked".into())
    }

    fn show_frame(
        stream: &Stream<'_>,
        hwnd: HWND,
        frame: &Frame,
        render_mode: u32,
    ) -> std::result::Result<(), HikCameraError> {
        let mut display_info = sys::MV_DISPLAY_FRAME_INFO_EX {
            nWidth: frame.info.width,
            nHeight: frame.info.height,
            enPixelType: frame.info.pixel_type as sys::MvGvspPixelType,
            pImageBuf: frame.data.as_ptr() as *mut _,
            nImageBufLen: frame.info.frame_len.try_into().map_err(|_| {
                HikCameraError::ValueOutOfRange {
                    field: "frame length",
                }
            })?,
            enRenderMode: render_mode,
            nRes: [0; 3],
        };

        let status = unsafe {
            sys::MV_CC_DisplayOneFrameEx(stream.raw_handle(), hwnd.cast(), &mut display_info)
        };
        if status == sys::MV_OK {
            Ok(())
        } else {
            Err(HikCameraError::Sdk {
                status: Status(status),
            })
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct DisplayChoice {
        source: DisplaySource,
        render_mode: u32,
    }

    impl DisplayChoice {
        fn label(self) -> &'static str {
            match (self.source, self.render_mode) {
                (DisplaySource::Raw, RENDER_MODE_D3D) => "raw + D3D",
                (DisplaySource::Raw, RENDER_MODE_GDI) => "raw + GDI",
                (DisplaySource::Bgr8, RENDER_MODE_D3D) => "BGR8 + D3D",
                (DisplaySource::Bgr8, RENDER_MODE_GDI) => "BGR8 + GDI",
                _ => "unknown",
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum DisplaySource {
        Raw,
        Bgr8,
    }

    struct Window {
        hwnd: HWND,
    }

    impl Window {
        fn new(title: &str, width: i32, height: i32) -> ShowResult<Self> {
            let class_name = wide("HikCameraStudioShowWindow");
            let title = wide(title);
            let instance = unsafe { GetModuleHandleW(ptr::null()) };

            let window_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: ptr::null_mut(),
                hCursor: unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };

            unsafe {
                RegisterClassW(&window_class);
            }

            let hwnd = unsafe {
                CreateWindowExW(
                    0,
                    class_name.as_ptr(),
                    title.as_ptr(),
                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    width,
                    height,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    instance,
                    ptr::null_mut(),
                )
            };
            if hwnd.is_null() {
                return Err("failed to create Win32 window".into());
            }

            Ok(Self { hwnd })
        }

        fn is_open(&self) -> bool {
            unsafe { IsWindow(self.hwnd) != 0 }
        }

        fn pump_messages(&self) {
            let mut message = MSG {
                hwnd: ptr::null_mut(),
                message: 0,
                wParam: 0,
                lParam: 0,
                time: 0,
                pt: POINT { x: 0, y: 0 },
            };

            while unsafe { PeekMessageW(&mut message, ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
                if message.message == WM_QUIT {
                    break;
                }

                unsafe {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        }
    }

    impl Drop for Window {
        fn drop(&mut self) {
            if !self.hwnd.is_null() && unsafe { IsWindow(self.hwnd) != 0 } {
                unsafe {
                    DestroyWindow(self.hwnd);
                }
            }
        }
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            value if value == WM_KEYDOWN && wparam == VK_ESCAPE as WPARAM => {
                unsafe {
                    DestroyWindow(hwnd);
                }
                0
            }
            value if value == WM_DESTROY => 0,
            _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
        }
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }
}
