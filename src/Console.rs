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

use widestring::U16CString;

use std::{convert::TryInto};
use std::mem::zeroed;

type Number = i32;

pub struct ConsoleRenderer {
    handle: HANDLE,
    text_buffer: Vec<CHAR_INFO>,

    pub screen_size: COORD,
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

    pub fn construct(&mut self, width: Number, height:Number, title: &str) {
        self.set_window_size(width, height, true);
        self.set_window_title(title);
    }

}


// Winapi functions with rust translation
impl ConsoleRenderer {
    pub fn set_window_size(&mut self, width: Number, height: Number, absolute:bool) {
        self.window_rect.Bottom = (height-1) as SHORT;
        self.window_rect.Right = (width-1) as SHORT;
        self.text_buffer = vec![CHAR_INFO::empty(); (width * height) as usize];
        self.screen_size.X = width as i16;
        self.screen_size.Y = height as i16;
        unsafe {  
            SetConsoleScreenBufferSize(self.handle, self.screen_size);
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

#[allow(dead_code)]
impl ConsoleRenderer {
    pub fn draw(&mut self, x: Number, y: Number, text: char, color: u16) {
        if x >= 0 && x <= self.screen_size.X as Number - 1 && y >= 0 && y <= self.screen_size.Y as Number-1 {
            self.text_buffer[(y*self.screen_size.X as Number + x) as usize] = CHAR_INFO {
                Char: CHAR_INFO_Char::from_char(text),
                Attributes: color,
            };
        }
    }

    pub fn draw_string(&mut self, x: Number, y: Number, text: &str, color: u16) {
        for (i, chr) in text.chars().enumerate() {
            self.draw(x + i as Number, y, chr, color);
        }
    }

    pub fn fill(&mut self, x: Number, y: Number, width: Number, height: Number, text: char, color: u16) {
        if width <= 0 && height <= 0 { return };
        for i in 0..height {
            for j in 0..width {
                self.draw(x+j,y+i,text,color);
            }
        }
    }

    // Bresenham's line algorithm
    // Source: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw_line(&mut self, point_a: (Number,Number), point_b: (Number,Number), text: char, color: u16) {
        let dx =  (point_b.0-point_a.0).abs();
        let sx = if point_a.0<point_b.0 { 1 } else { -1 };
        let dy = -(point_b.1-point_a.1).abs();
        let sy = if point_a.1<point_b.1 { 1 } else { -1 };
        let mut err = dx+dy;

        let mut x = point_a.0;
        let mut y = point_a.1;

        loop {
            self.draw(x, y, text, color);
            if x == point_b.0 && y == point_b.1 { break };
            let e2 = 2*err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    // Polygon rendering in a nutshell
    // Decided not to do filling of polygons. Cuz shit too fucking hard
    pub fn draw_polygon(&mut self, points: Vec<(Number,Number)>, text:char, color:u16) {
        if points.len() == 0 { return };
        if points.len() == 1 { return self.draw(points[0].0, points[0].1, text, color) };

        for i in 0..points.len() {
            let point = points[i];
            if i == points.len()-1 {
                self.draw_line(point, points[0], text, color);
                continue;
            }
            self.draw_line(point, points[i+1], text, color);
        }

    }


    // Mid-Point Circle Drawing Algorithm
    // Source: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    pub fn draw_circle(&mut self, x: Number, y: Number, r: Number, text: char, color: u16, fill:bool){

        let mut _x = r;
        let mut _y = 0;
        if r > 0 {
            if fill {
                self.draw_line((-_y+x, _x+y), (x,y-r), text, color);
            } else {
                self.draw(_x+x, -_y+y, text, color);
                self.draw(-_y+x, _x+y, text, color);
                self.draw(x-r, y, text, color);
                self.draw(x,y-r, text, color);
            }
        }
        let mut P = 1 - r;   
        while _x > _y {
            _y += 1;
            
            if P <= 0 {
                P = P + 2*_y + 1;
            } else {
                _x -= 1;
                P = P + 2*_y - 2*_x + 1;
            }
            
            if _x < _y {
                break;
            }

            
            if fill {
                self.draw_line((_x + x,_y + y), (_x + x,-_y + y), text, color);
                self.draw_line((-_x + x,_y + y), (-_x + x,-_y + y), text, color);
            } else {
                self.draw(_x + x,_y + y, text, color);
                self.draw(-_x + x,_y + y, text, color);
                self.draw(_x + x,-_y + y, text, color);
                self.draw(-_x + x,-_y + y, text, color);
            }
            
            if _x != _y {
                if fill {
                    self.draw_line((_y + x, _x + y), (_y + x, -_x + y), text, color);
                    self.draw_line((-_y + x, _x + y), (-_y + x, -_x + y), text, color);
                } else {
                    self.draw(_y + x, _x + y, text, color);
                    self.draw(-_y + x, _x + y, text, color);
                    self.draw(_y + x, -_x + y, text, color);
                    self.draw(-_y + x, -_x + y, text, color);
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

    pub fn is_key_down(key:i32) -> bool {
        unsafe { winapi::um::winuser::GetAsyncKeyState(key) != 0 }
    }
}


// Impl's that is used for initializing winapi types
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

pub struct Keys { }
#[allow(dead_code)]
impl Keys {
    pub const VK_ABNT_C1:i32 = 0xC1; //	Abnt C1
    pub const VK_ABNT_C2:i32 = 0xC2; //	Abnt C2
    pub const VK_ADD:i32 = 0x6B; //	Numpad +
    pub const VK_ATTN:i32 = 0xF6; //	Attn
    pub const VK_BACK:i32 = 0x08; //	Backspace
    pub const VK_CANCEL:i32 = 0x03; //	Break
    pub const VK_CLEAR:i32 = 0x0C; //	Clear
    pub const VK_CRSEL:i32 = 0xF7; //	Cr Sel
    pub const VK_DECIMAL:i32 = 0x6E; //	Numpad .
    pub const VK_DIVIDE:i32 = 0x6F; //	Numpad /
    pub const VK_EREOF:i32 = 0xF9; //	Er Eof
    pub const VK_ESCAPE:i32 = 0x1B; //	Esc
    pub const VK_EXECUTE:i32 = 0x2B; //	Execute
    pub const VK_EXSEL:i32 = 0xF8; //	Ex Sel
    pub const VK_ICO_CLEAR:i32 = 0xE6; //	IcoClr
    pub const VK_ICO_HELP:i32 = 0xE3; //	IcoHlp
    pub const VK_0:i32 = 0x30; // ('0')	0
    pub const VK_1:i32 = 0x31; // ('1')	1
    pub const VK_2:i32 = 0x32; // ('2')	2
    pub const VK_3:i32 = 0x33; // ('3')	3
    pub const VK_4:i32 = 0x34; // ('4')	4
    pub const VK_5:i32 = 0x35; // ('5')	5
    pub const VK_6:i32 = 0x36; // ('6')	6
    pub const VK_7:i32 = 0x37; // ('7')	7
    pub const VK_8:i32 = 0x38; // ('8')	8
    pub const VK_9:i32 = 0x39; // ('9')	9
    pub const VK_A:i32 = 0x41; // ('A')	A
    pub const VK_B:i32 = 0x42; // ('B')	B
    pub const VK_C:i32 = 0x43; // ('C')	C
    pub const VK_D:i32 = 0x44; // ('D')	D
    pub const VK_E:i32 = 0x45; // ('E')	E
    pub const VK_F:i32 = 0x46; // ('F')	F
    pub const VK_G:i32 = 0x47; // ('G')	G
    pub const VK_H:i32 = 0x48; // ('H')	H
    pub const VK_I:i32 = 0x49; // ('I')	I
    pub const VK_J:i32 = 0x4A; // ('J')	J
    pub const VK_K:i32 = 0x4B; // ('K')	K
    pub const VK_L:i32 = 0x4C; // ('L')	L
    pub const VK_M:i32 = 0x4D; // ('M')	M
    pub const VK_N:i32 = 0x4E; // ('N')	N
    pub const VK_O:i32 = 0x4F; // ('O')	O
    pub const VK_P:i32 = 0x50; // ('P')	P
    pub const VK_Q:i32 = 0x51; // ('Q')	Q
    pub const VK_R:i32 = 0x52; // ('R')	R
    pub const VK_S:i32 = 0x53; // ('S')	S
    pub const VK_T:i32 = 0x54; // ('T')	T
    pub const VK_U:i32 = 0x55; // ('U')	U
    pub const VK_V:i32 = 0x56; // ('V')	V
    pub const VK_W:i32 = 0x57; // ('W')	W
    pub const VK_X:i32 = 0x58; // ('X')	X
    pub const VK_Y:i32 = 0x59; // ('Y')	Y
    pub const VK_Z:i32 = 0x5A; // ('Z')	Z
    pub const VK_MULTIPLY:i32 = 0x6A; //	Numpad *
    pub const VK_NONAME:i32 = 0xFC; //	NoName
    pub const VK_NUMPAD0:i32 = 0x60; //	Numpad 0
    pub const VK_NUMPAD1:i32 = 0x61; //	Numpad 1
    pub const VK_NUMPAD2:i32 = 0x62; //	Numpad 2
    pub const VK_NUMPAD3:i32 = 0x63; //	Numpad 3
    pub const VK_NUMPAD4:i32 = 0x64; //	Numpad 4
    pub const VK_NUMPAD5:i32 = 0x65; //	Numpad 5
    pub const VK_NUMPAD6:i32 = 0x66; //	Numpad 6
    pub const VK_NUMPAD7:i32 = 0x67; //	Numpad 7
    pub const VK_NUMPAD8:i32 = 0x68; //	Numpad 8
    pub const VK_NUMPAD9:i32 = 0x69; //	Numpad 9
    pub const VK_OEM_1:i32 = 0xBA; //	OEM_1 (: ;)
    pub const VK_OEM_102:i32 = 0xE2; //	OEM_102 (> <)
    pub const VK_OEM_2:i32 = 0xBF; //	OEM_2 (? /)
    pub const VK_OEM_3:i32 = 0xC0; //	OEM_3 (~ `)
    pub const VK_OEM_4:i32 = 0xDB; //	OEM_4 ({ [)
    pub const VK_OEM_5:i32 = 0xDC; //	OEM_5 (| \)
    pub const VK_OEM_6:i32 = 0xDD; //	OEM_6 (} ])
    pub const VK_OEM_7:i32 = 0xDE; //	OEM_7 (" ')
    pub const VK_OEM_8:i32 = 0xDF; //	OEM_8 (§ !)
    pub const VK_OEM_ATTN:i32 = 0xF0; //	Oem Attn
    pub const VK_OEM_AUTO:i32 = 0xF3; //	Auto
    pub const VK_OEM_AX:i32 = 0xE1; //	Ax
    pub const VK_OEM_BACKTAB:i32 = 0xF5; //	Back Tab
    pub const VK_OEM_CLEAR:i32 = 0xFE; //	OemClr
    pub const VK_OEM_COMMA:i32 = 0xBC; //	OEM_COMMA (< ,)
    pub const VK_OEM_COPY:i32 = 0xF2; //	Copy
    pub const VK_OEM_CUSEL:i32 = 0xEF; //	Cu Sel
    pub const VK_OEM_ENLW:i32 = 0xF4; //	Enlw
    pub const VK_OEM_FINISH:i32 = 0xF1; //	Finish
    pub const VK_OEM_FJ_LOYA:i32 = 0x95; //	Loya
    pub const VK_OEM_FJ_MASSHOU:i32 = 0x93; //	Mashu
    pub const VK_OEM_FJ_ROYA:i32 = 0x96; //	Roya
    pub const VK_OEM_FJ_TOUROKU:i32 = 0x94; //	Touroku
    pub const VK_OEM_JUMP:i32 = 0xEA; //	Jump
    pub const VK_OEM_MINUS:i32 = 0xBD; //	OEM_MINUS (_ -)
    pub const VK_OEM_PA1:i32 = 0xEB; //	OemPa1
    pub const VK_OEM_PA2:i32 = 0xEC; //	OemPa2
    pub const VK_OEM_PA3:i32 = 0xED; //	OemPa3
    pub const VK_OEM_PERIOD:i32 = 0xBE; //	OEM_PERIOD (> .)
    pub const VK_OEM_PLUS:i32 = 0xBB; //	OEM_PLUS (+ =)
    pub const VK_OEM_RESET:i32 = 0xE9; //	Reset
    pub const VK_OEM_WSCTRL:i32 = 0xEE; //	WsCtrl
    pub const VK_PA1:i32 = 0xFD; //	Pa1
    pub const VK_PACKET:i32 = 0xE7; //	Packet
    pub const VK_PLAY:i32 = 0xFA; //	Play
    pub const VK_PROCESSKEY:i32 = 0xE5; //	Process
    pub const VK_RETURN:i32 = 0x0D; //	Enter
    pub const VK_SELECT:i32 = 0x29; //	Select
    pub const VK_SEPARATOR:i32 = 0x6C; //	Separator
    pub const VK_SPACE:i32 = 0x20; //	Space
    pub const VK_SUBTRACT:i32 = 0x6D; //	Num -
    pub const VK_TAB:i32 = 0x09; //	Tab
    pub const VK_ZOOM:i32 = 0xFB; //	Zoom
    pub const VK_ACCEPT:i32 = 0x1E; //	Accept
    pub const VK_APPS:i32 = 0x5D; //	Context Menu
    pub const VK_BROWSER_BACK:i32 = 0xA6; //	Browser Back
    pub const VK_BROWSER_FAVORITES:i32 = 0xAB; //	Browser Favorites
    pub const VK_BROWSER_FORWARD:i32 = 0xA7; //	Browser Forward
    pub const VK_BROWSER_HOME:i32 = 0xAC; //	Browser Home
    pub const VK_BROWSER_REFRESH:i32 = 0xA8; //	Browser Refresh
    pub const VK_BROWSER_SEARCH:i32 = 0xAA; //	Browser Search
    pub const VK_BROWSER_STOP:i32 = 0xA9; //	Browser Stop
    pub const VK_CAPITAL:i32 = 0x14; //	Caps Lock
    pub const VK_CONVERT:i32 = 0x1C; //	Convert
    pub const VK_DELETE:i32 = 0x2E; //	Delete
    pub const VK_DOWN:i32 = 0x28; //	Arrow Down
    pub const VK_END:i32 = 0x23; //	End
    pub const VK_F1:i32 = 0x70; //	F1
    pub const VK_F10:i32 = 0x79; //	F10
    pub const VK_F11:i32 = 0x7A; //	F11
    pub const VK_F12:i32 = 0x7B; //	F12
    pub const VK_F13:i32 = 0x7C; //	F13
    pub const VK_F14:i32 = 0x7D; //	F14
    pub const VK_F15:i32 = 0x7E; //	F15
    pub const VK_F16:i32 = 0x7F; //	F16
    pub const VK_F17:i32 = 0x80; //	F17
    pub const VK_F18:i32 = 0x81; //	F18
    pub const VK_F19:i32 = 0x82; //	F19
    pub const VK_F2:i32 = 0x71; //	F2
    pub const VK_F20:i32 = 0x83; //	F20
    pub const VK_F21:i32 = 0x84; //	F21
    pub const VK_F22:i32 = 0x85; //	F22
    pub const VK_F23:i32 = 0x86; //	F23
    pub const VK_F24:i32 = 0x87; //	F24
    pub const VK_F3:i32 = 0x72; //	F3
    pub const VK_F4:i32 = 0x73; //	F4
    pub const VK_F5:i32 = 0x74; //	F5
    pub const VK_F6:i32 = 0x75; //	F6
    pub const VK_F7:i32 = 0x76; //	F7
    pub const VK_F8:i32 = 0x77; //	F8
    pub const VK_F9:i32 = 0x78; //	F9
    pub const VK_FINAL:i32 = 0x18; //	Final
    pub const VK_HELP:i32 = 0x2F; //	Help
    pub const VK_HOME:i32 = 0x24; //	Home
    pub const VK_ICO_00:i32 = 0xE4; //	Ico00 *
    pub const VK_INSERT:i32 = 0x2D; //	Insert
    pub const VK_JUNJA:i32 = 0x17; //	Junja
    pub const VK_KANA:i32 = 0x15; //	Kana
    pub const VK_KANJI:i32 = 0x19; //	Kanji
    pub const VK_LAUNCH_APP1:i32 = 0xB6; //	App1
    pub const VK_LAUNCH_APP2:i32 = 0xB7; //	App2
    pub const VK_LAUNCH_MAIL:i32 = 0xB4; //	Mail
    pub const VK_LAUNCH_MEDIA_SELECT:i32 = 0xB5; //	Media
    pub const VK_LBUTTON:i32 = 0x01; //	Left Button **
    pub const VK_LCONTROL:i32 = 0xA2; //	Left Ctrl
    pub const VK_LEFT:i32 = 0x25; //	Arrow Left
    pub const VK_LMENU:i32 = 0xA4; //	Left Alt
    pub const VK_LSHIFT:i32 = 0xA0; //	Left Shift
    pub const VK_LWIN:i32 = 0x5B; //	Left Win
    pub const VK_MBUTTON:i32 = 0x04; //	Middle Button **
    pub const VK_MEDIA_NEXT_TRACK:i32 = 0xB0; //	Next Track
    pub const VK_MEDIA_PLAY_PAUSE:i32 = 0xB3; //	Play / Pause
    pub const VK_MEDIA_PREV_TRACK:i32 = 0xB1; //	Previous Track
    pub const VK_MEDIA_STOP:i32 = 0xB2; //	Stop
    pub const VK_MODECHANGE:i32 = 0x1F; //	Mode Change
    pub const VK_NEXT:i32 = 0x22; //	Page Down
    pub const VK_NONCONVERT:i32 = 0x1D; //	Non Convert
    pub const VK_NUMLOCK:i32 = 0x90; //	Num Lock
    pub const VK_OEM_FJ_JISHO:i32 = 0x92; //	Jisho
    pub const VK_PAUSE:i32 = 0x13; //	Pause
    pub const VK_PRINT:i32 = 0x2A; //	Print
    pub const VK_PRIOR:i32 = 0x21; //	Page Up
    pub const VK_RBUTTON:i32 = 0x02; //	Right Button **
    pub const VK_RCONTROL:i32 = 0xA3; //	Right Ctrl
    pub const VK_RIGHT:i32 = 0x27; //	Arrow Right
    pub const VK_RMENU:i32 = 0xA5; //	Right Alt
    pub const VK_RSHIFT:i32 = 0xA1; //	Right Shift
    pub const VK_RWIN:i32 = 0x5C; //	Right Win
    pub const VK_SCROLL:i32 = 0x91; //	Scrol Lock
    pub const VK_SLEEP:i32 = 0x5F; //	Sleep
    pub const VK_SNAPSHOT:i32 = 0x2C; //	Print Screen
    pub const VK_UP:i32 = 0x26; //	Arrow Up
    pub const VK_VOLUME_DOWN:i32 = 0xAE; //	Volume Down
    pub const VK_VOLUME_MUTE:i32 = 0xAD; //	Volume Mute
    pub const VK_VOLUME_UP:i32 = 0xAF; //	Volume Up
    pub const VK_XBUTTON1:i32 = 0x05; //	X Button 1 **
    pub const VK_XBUTTON2:i32 = 0x06; //	X Button 2 **
}


pub struct Color;
#[allow(dead_code)]
impl Color {
    pub const FG_BLACK:u16	     = 0x0000;
    pub const FG_DARKBLUE:u16    = 0x0001;	
    pub const FG_DARKGREEN:u16   = 0x0002;
    pub const FG_DARKCYAN:u16    = 0x0003;
    pub const FG_DARKRED:u16     = 0x0004;
    pub const FG_DARKMAGENTA:u16 = 0x0005;
    pub const FG_DARKYELLOW:u16  = 0x0006;
    pub const FG_GREY:u16        = 0x0007;
    pub const FG_DARKGREY:u16    = 0x0008;
    pub const FG_BLUE:u16        = 0x0009;
    pub const FG_GREEN:u16		 = 0x000A;
    pub const FG_CYAN:u16        = 0x000B;
    pub const FG_RED:u16         = 0x000C;
    pub const FG_MAGENTA:u16     = 0x000D;
    pub const FG_YELLOW:u16      = 0x000E;
    pub const FG_WHITE:u16		 = 0x000F;
    pub const BG_BLACK:u16	     = 0x0000;
    pub const BG_DARKBLUE:u16	 = 0x0010;
    pub const BG_DARKGREEN:u16	 = 0x0020;
    pub const BG_DARKCYAN:u16	 = 0x0030;
    pub const BG_DARKRED:u16     = 0x0040;
    pub const BG_DARKMAGENTA:u16 = 0x0050;
    pub const BG_DARKYELLO:u16   = 0x0060;
    pub const BG_GREY:u16        = 0x0070;
    pub const BG_DARKGREY:u16	 = 0x0080;
    pub const BG_BLUE:u16        = 0x0090;
    pub const BG_GREEN:u16		 = 0x00A0;
    pub const BG_CYAN:u16        = 0x00B0;
    pub const BG_RED:u16         = 0x00C0;
    pub const BG_MAGENTA:u16     = 0x00D0;
    pub const BG_YELLOW:u16      = 0x00E0;
    pub const BG_WHITE:u16		 = 0x00F0;
}