use winapi::{
    shared::minwindef::{BOOL},
    um::{
        handleapi::INVALID_HANDLE_VALUE,
        processenv::GetStdHandle,
        winbase::{ STD_OUTPUT_HANDLE },
        wincon::{
            CONSOLE_FONT_INFOEX,
            SetConsoleActiveScreenBuffer,
            SetConsoleScreenBufferSize,
            SetConsoleTitleW,
            SetConsoleWindowInfo,
            SetCurrentConsoleFontEx,
            WriteConsoleOutputW,
        },
        wincontypes::{
            CHAR_INFO, CHAR_INFO_Char,
            COORD,
            SMALL_RECT,
        },
        winnt::{
            SHORT,
            HANDLE,
            WCHAR,
        }
    }
};



use winapi::ctypes::c_int;
use widestring::U16CString;

use std::{convert::TryInto};
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


impl Empty for COORD {
    fn empty() -> COORD {
        COORD {
            X:0,
            Y:0,
        }
    }
}

impl Empty for CONSOLE_FONT_INFOEX {
    fn empty() -> CONSOLE_FONT_INFOEX {
        CONSOLE_FONT_INFOEX {
            cbSize: 0,
            nFont: 0,
            dwFontSize: COORD::empty(),
            FontFamily: 0,
            FontWeight: 0,
            FaceName: [0 as WCHAR; 32],
        }
    }
    
}


pub struct ConsoleRenderer {
    handle: HANDLE,
    text_buffer: Vec<CHAR_INFO>,

    screen_size: COORD,
    window_rect: SMALL_RECT,

    font: CONSOLE_FONT_INFOEX,
}


// Basic initialization and setup functions
impl ConsoleRenderer {
    pub fn new() -> ConsoleRenderer {
        let console_out_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        if console_out_handle == INVALID_HANDLE_VALUE { panic!("Console Handle was invalid") };
        ConsoleRenderer {
            handle: console_out_handle,
            screen_size: COORD { X:0, Y:0 },
            text_buffer: vec![CHAR_INFO::empty(); 0],
            window_rect:SMALL_RECT { Left:0, Top:0, Right:0, Bottom:0 },

            font: CONSOLE_FONT_INFOEX::empty(),
        }
    }

    pub fn construct(&mut self, width: u16, height:u16, title: &str) {
        self.set_window_size(width, height, true);
        self.set_window_title(title);
    }

}


// Winapi functions with rust translation
impl ConsoleRenderer {
    pub fn set_window_size(&mut self, width: u16, height: u16, absolute:bool) {
        self.window_rect.Bottom = (height-1) as SHORT;
        self.window_rect.Right = (width-1) as SHORT;
        self.text_buffer = vec![CHAR_INFO::empty(); (width * height) as usize];
        self.screen_size.X = width as i16;
        self.screen_size.Y = height as i16;
        unsafe {  
            SetConsoleScreenBufferSize(self.handle, COORD { X: width as i16, Y: height as i16 });
            SetConsoleActiveScreenBuffer(self.handle);
            SetConsoleWindowInfo(self.handle, absolute as BOOL, &self.window_rect);
        };
    }

    pub fn set_window_title(&self, title: &str) {
        unsafe { SetConsoleTitleW(U16CString::from_str(title).unwrap().as_ptr()) };
    }

    pub fn set_font_size(&mut self, font_weight: u32, font_size: (i16,i16)) {
        self.font.FontWeight = font_weight;
        self.font.dwFontSize = COORD { X: font_size.0, Y: font_size.1 };
        self.font.cbSize = std::mem::size_of::<CONSOLE_FONT_INFOEX>() as u32;
        unsafe { SetCurrentConsoleFontEx(self.handle, 0, &mut self.font); }
    }  
}


impl ConsoleRenderer {
    pub fn draw(&mut self, x: i16, y: i16, text: char, color: u16) {
        if x >= 0 && x <= self.screen_size.X-1 && y >= 0 && y <= self.screen_size.Y-1 {
            self.text_buffer[(y*self.screen_size.X + x) as usize] = CHAR_INFO {
                Char: CHAR_INFO_Char::from_char(text),
                Attributes: color,
            };
        }
    }

    pub fn draw_string(&mut self, x: i16, y: i16, text: &str, color: u16) {
        for (i, chr) in text.chars().enumerate() {
            self.draw(x + i as i16, y, chr, color);
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
        self.text_buffer.iter_mut().for_each(|x| *x = CHAR_INFO::empty());
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
