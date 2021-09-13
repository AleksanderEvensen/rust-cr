use winapi::um::{
    processenv::GetStdHandle,
    wincon::{
        SetConsoleTitleW,
        WriteConsoleOutputW,
        SetConsoleWindowInfo,
        SetConsoleScreenBufferSize,
        SetConsoleActiveScreenBuffer,
    },
    wincontypes::{
        CHAR_INFO, CHAR_INFO_Char,
        COORD,
        SMALL_RECT,
    },
    winnt::{
        SHORT,
        HANDLE,
    },

    handleapi::INVALID_HANDLE_VALUE,
    winbase::{ STD_OUTPUT_HANDLE },
};


use winapi::ctypes::c_int;
use widestring::U16CString;

use std::convert::TryInto;
use std::mem::zeroed;

trait Empty {
    fn empty() -> Self;
}

trait FromChar {
    fn from_char(chr: char) -> Self;
}

impl Empty for CHAR_INFO {
    fn empty() -> CHAR_INFO {
        CHAR_INFO {
            Char: CHAR_INFO_Char::empty(),
            Attributes: 0,
        }
    }
}

impl Empty for CHAR_INFO_Char {
    fn empty() -> CHAR_INFO_Char {
        unsafe { zeroed::<CHAR_INFO_Char>() }
    }
}

impl FromChar for CHAR_INFO_Char {
    fn from_char(chr: char) -> CHAR_INFO_Char {
        let chr: SHORT = chr as SHORT;

        let mut inf_char = CHAR_INFO_Char::empty();
        unsafe {
            *inf_char.UnicodeChar_mut() = chr.try_into().unwrap();   
        };

        inf_char
    }
}

pub struct ConsoleRenderer {
    handle: HANDLE,
    text_buffer: Vec<CHAR_INFO>,
    screen_size: COORD,

    window_rect: SMALL_RECT,
}


impl ConsoleRenderer {
    pub fn new(title: &str, width:i16, height:i16) -> ConsoleRenderer {
        let console_output_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

        if console_output_handle == INVALID_HANDLE_VALUE {
            panic!("Console Handle was invalid");
        }

        let window_rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: width-1,
            Bottom: height-1,
        };


        unsafe {
            SetConsoleWindowInfo(console_output_handle, 1, &window_rect);
            SetConsoleScreenBufferSize(console_output_handle, COORD { X: width, Y: height });
            SetConsoleActiveScreenBuffer(console_output_handle);
            SetConsoleTitleW(U16CString::from_str(title).unwrap().as_ptr());
        }



        ConsoleRenderer {
            handle: console_output_handle,
            text_buffer:vec!(CHAR_INFO::empty(); (width*height).try_into().unwrap()),
            screen_size: COORD { X: width, Y: height },

            window_rect: window_rect,
        }
    }

    pub fn set_title(&self, title: &str) {
        let title = U16CString::from_str(title).expect("Failed to convert &str to U16CString");
        unsafe { SetConsoleTitleW(title.as_ptr()) };
    }

    pub fn draw(&mut self, x: i16, y: i16, text: char, color: u16) {
        if x >= 0 && x <= self.screen_size.X && y >= 0 && y <= self.screen_size.Y {
            let mut char_info = CHAR_INFO {
                Char: CHAR_INFO_Char::from_char(text),
                Attributes: color,
            };
            self.text_buffer[(y*self.screen_size.X + x) as usize] = char_info;
        }
    }

    pub fn fill(&mut self, x: i16, y: i16, width: i16, height: i16, text: char, color: u16) {
        if width <= 0 && height <= 0 {return};
        
        let end = COORD { X: (x+width), Y: (y+height) };
        if x >= 0 && end.X <= self.screen_size.X && y >= 0 && end.Y <= self.screen_size.Y {
            
            for i in 0..height {
                for j in 0..width {
                    self.draw(x+j,y+i,text,color);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        &self.text_buffer.iter_mut().for_each(|x| *x = CHAR_INFO::empty());
    }

    pub fn blit(&self) {
        let mut rect = self.window_rect;
        let rect_ptr = &mut rect;
        unsafe { WriteConsoleOutputW(self.handle, self.text_buffer.as_ptr(), self.screen_size, COORD { X: 0, Y: 0 }, rect_ptr) };
    }

    pub fn is_key_down(key:c_int) -> bool {
        unsafe { winapi::um::winuser::GetAsyncKeyState(key) != 0 }
    }
}
