#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
use winapi::{DWORD, HANDLE, INVALID_HANDLE_VALUE, wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT}, WCHAR, WORD};

#[cfg(target_os = "windows")]
static mut CONSOLE_HANDLE: Option<HANDLE> = None;

#[cfg(target_os = "windows")]
fn get_console_handle() -> HANDLE {
    unsafe {
        return if let Some(handle) = CONSOLE_HANDLE {
            handle
        } else {
            let handle: HANDLE = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);

            handle
        }
    }
}

#[cfg(target_os = "windows")]
fn get_buffer_info() -> CONSOLE_SCREEN_BUFFER_INFO {
    let handle: HANDLE = get_console_handle();

    if handle == INVALID_HANDLE_VALUE {
        panic!("NoConsole");
    }

    let mut buffer = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };

    unsafe {
        kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer);
    }

    buffer
}

#[cfg(target_os = "windows")]
fn set_cursor_position(x: i16, y: i16) {
    let handle = get_console_handle();

    if handle == INVALID_HANDLE_VALUE {
        panic!("NoConsole");
    }

    unsafe {
        kernel32::SetConsoleCursorPosition(handle, COORD { X: x, Y: y });
    }
}

#[cfg(target_os = "windows")]
pub fn clear() {
    let handle: HANDLE = get_console_handle();

    if handle == INVALID_HANDLE_VALUE {
        panic!("NoConsole");
    }

    let screen_buffer: CONSOLE_SCREEN_BUFFER_INFO = get_buffer_info();
    let console_size: DWORD = screen_buffer.dwSize.X as u32 * screen_buffer.dwSize.Y as u32;
    let coord_screen = COORD { X: 0, Y: 0 };

    let mut amount_chart_written: DWORD = 0;

    unsafe {
        kernel32::FillConsoleOutputCharacterW(
            handle,
            32 as WCHAR,
            console_size,
            coord_screen,
            &mut amount_chart_written,
        );
    }

    set_cursor_position(0, 0);
}

#[cfg(target_os = "linux")]
pub fn clear() {
    let output = Command::new("clear")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}