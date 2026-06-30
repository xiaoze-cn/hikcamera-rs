#ifndef HIKCAMERA_SYS_MV_CAMERA_H
#define HIKCAMERA_SYS_MV_CAMERA_H

// Defense in depth: `build.rs` already panics with a friendlier message when
// the target isn't Windows, but if anything ever bypasses the build script
// (manual `bindgen` invocation, future refactor), this `#error` makes the
// underlying reason visible at the C-preprocessor level instead of letting
// the include chain fail with cryptic "file not found" errors deep inside
// the Windows-only SDK headers.
#if !defined(_WIN32) && !defined(_WIN64)
#error "hikcamera-sys: the HikCamera MVS SDK is Windows-only and cannot be built here."
#endif

#include "MvCameraControl.h"

#endif
