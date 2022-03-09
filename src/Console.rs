use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HANDLE, INVALID_HANDLE_VALUE},
        System::Console::*,
        UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY},
    },
};

type Number = i16;

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

        if console_out_handle == INVALID_HANDLE_VALUE {
            panic!("Failed to get console handle. Handle was invalid");
        }

        ConsoleRenderer {
            handle: console_out_handle,
            screen_size: COORD { X: 0, Y: 0 },
            text_buffer: vec![CHAR_INFO::empty(); 0],
            window_rect: SMALL_RECT {
                Left: 0,
                Top: 0,
                Right: 0,
                Bottom: 0,
            },

            font: CONSOLE_FONT_INFOEX::empty(),
        }
    }

    pub fn construct(&mut self, width: Number, height: Number, title: &str) {
        self.set_window_size(width, height, true);
        self.set_window_title(title);
    }
}

// Winapi functions with rust translation
impl ConsoleRenderer {
    pub fn set_window_size(&mut self, width: Number, height: Number, absolute: bool) {
        self.window_rect.Bottom = height - 1;

        self.window_rect.Bottom = height - 1;
        self.window_rect.Right = width - 1;
        self.text_buffer = vec![CHAR_INFO::empty(); (width * height) as usize];
        self.screen_size.X = width as i16;
        self.screen_size.Y = height as i16;

        unsafe {
            SetConsoleScreenBufferSize(self.handle, self.screen_size);
            SetConsoleActiveScreenBuffer(self.handle);
            SetConsoleWindowInfo(self.handle, absolute, &self.window_rect);
        };
    }

    pub fn set_window_title(&self, title: &str) {
        unsafe {
            SetConsoleTitleW(PCWSTR::from_str(title));
        };
        // unsafe { SetConsoleTitleW(U16CString::from_str(title).unwrap().as_ptr()) };
    }

    pub fn set_font_size(&mut self, font_weight: u32, font_size: (i16, i16)) {
        self.font.FontWeight = font_weight;
        self.font.dwFontSize = COORD {
            X: font_size.0,
            Y: font_size.1,
        };
        self.font.cbSize = std::mem::size_of::<CONSOLE_FONT_INFOEX>() as u32;
        unsafe {
            SetCurrentConsoleFontEx(self.handle, false, &mut self.font);
        }
    }
}

#[allow(dead_code)]
impl ConsoleRenderer {
    pub fn draw(&mut self, x: Number, y: Number, text: char, color: u16) {
        if x >= 0
            && x <= self.screen_size.X as Number - 1
            && y >= 0
            && y <= self.screen_size.Y as Number - 1
        {
            let mut test = CHAR_INFO_0::default();

            test.UnicodeChar = text as u16;

            self.text_buffer[(y * self.screen_size.X as Number + x) as usize] = CHAR_INFO {
                Char: test,
                Attributes: color,
            };
        }
    }

    pub fn draw_string(&mut self, x: Number, y: Number, text: &str, color: u16) {
        for (i, chr) in text.chars().enumerate() {
            self.draw(x + i as Number, y, chr, color);
        }
    }

    pub fn fill(
        &mut self,
        x: Number,
        y: Number,
        width: Number,
        height: Number,
        text: char,
        color: u16,
    ) {
        if width <= 0 && height <= 0 {
            return;
        };
        for i in 0..height {
            for j in 0..width {
                self.draw(x + j, y + i, text, color);
            }
        }
    }

    // Bresenham's line algorithm
    // Source: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw_line(
        &mut self,
        point_a: (Number, Number),
        point_b: (Number, Number),
        text: char,
        color: u16,
    ) {
        let dx = (point_b.0 - point_a.0).abs();
        let sx = if point_a.0 < point_b.0 { 1 } else { -1 };
        let dy = -(point_b.1 - point_a.1).abs();
        let sy = if point_a.1 < point_b.1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = point_a.0;
        let mut y = point_a.1;

        loop {
            self.draw(x, y, text, color);
            if x == point_b.0 && y == point_b.1 {
                break;
            };
            let e2 = 2 * err;
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
    pub fn draw_polygon(&mut self, points: Vec<(Number, Number)>, text: char, color: u16) {
        if points.len() == 0 {
            return;
        };
        if points.len() == 1 {
            return self.draw(points[0].0, points[0].1, text, color);
        };

        for i in 0..points.len() {
            let point = points[i];
            if i == points.len() - 1 {
                self.draw_line(point, points[0], text, color);
                continue;
            }
            self.draw_line(point, points[i + 1], text, color);
        }
    }

    // Mid-Point Circle Drawing Algorithm
    // Source: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    pub fn draw_circle(
        &mut self,
        x: Number,
        y: Number,
        r: Number,
        text: char,
        color: u16,
        fill: bool,
    ) {
        let mut _x = r;
        let mut _y = 0;
        if r > 0 {
            if fill {
                self.draw_line((-_y + x, _x + y), (x, y - r), text, color);
            } else {
                self.draw(_x + x, -_y + y, text, color);
                self.draw(-_y + x, _x + y, text, color);
                self.draw(x - r, y, text, color);
                self.draw(x, y - r, text, color);
            }
        }
        let mut P = 1 - r;
        while _x > _y {
            _y += 1;

            if P <= 0 {
                P = P + 2 * _y + 1;
            } else {
                _x -= 1;
                P = P + 2 * _y - 2 * _x + 1;
            }

            if _x < _y {
                break;
            }

            if fill {
                self.draw_line((_x + x, _y + y), (_x + x, -_y + y), text, color);
                self.draw_line((-_x + x, _y + y), (-_x + x, -_y + y), text, color);
            } else {
                self.draw(_x + x, _y + y, text, color);
                self.draw(-_x + x, _y + y, text, color);
                self.draw(_x + x, -_y + y, text, color);
                self.draw(-_x + x, -_y + y, text, color);
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
        self.text_buffer
            .iter_mut()
            .for_each(|x| *x = CHAR_INFO::empty());
    }

    pub fn blit(&self) {
        let mut rect = self.window_rect;
        let rect_ptr = &mut rect;
        unsafe {
            WriteConsoleOutputW(
                self.handle,
                self.text_buffer.as_ptr(),
                self.screen_size,
                COORD { X: 0, Y: 0 },
                rect_ptr,
            )
        };
    }

    pub fn is_key_down(key: VIRTUAL_KEY) -> bool {
        unsafe { GetAsyncKeyState(key.0 as i32) != 0 }
    }
}

// Impl's that is used for initializing winapi types
trait Empty {
    fn empty() -> Self;
}

trait FromChar {
    fn from_char(chr: char) -> Self;
}

trait FromString {
    fn from_str(s: &str) -> Self;
    fn from_string(s: String) -> Self;
}

impl Empty for COORD {
    fn empty() -> COORD {
        COORD { X: 0, Y: 0 }
    }
}

impl Empty for CHAR_INFO {
    fn empty() -> CHAR_INFO {
        CHAR_INFO {
            Char: CHAR_INFO_0::from_char(' '),
            Attributes: 0,
        }
    }
}

impl FromChar for CHAR_INFO_0 {
    fn from_char(chr: char) -> CHAR_INFO_0 {
        let mut chr_info = CHAR_INFO_0::default();
        chr_info.UnicodeChar = chr as u16;
        return chr_info;
    }
}

impl FromString for PCWSTR {
    fn from_str(s: &str) -> Self {
        let mut result = PCWSTR::default();
        result.0 = s.encode_utf16().collect::<Vec<u16>>().as_ptr();
        return result;
    }

    fn from_string(s: String) -> Self {
        let mut result = PCWSTR::default();
        result.0 = s.encode_utf16().collect::<Vec<u16>>().as_ptr();
        return result;
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
            FaceName: [0; 32],
        }
    }
}

pub struct Color;
#[allow(dead_code)]
impl Color {
    pub const FG_BLACK: u16 = 0x0000;
    pub const FG_DARKBLUE: u16 = 0x0001;
    pub const FG_DARKGREEN: u16 = 0x0002;
    pub const FG_DARKCYAN: u16 = 0x0003;
    pub const FG_DARKRED: u16 = 0x0004;
    pub const FG_DARKMAGENTA: u16 = 0x0005;
    pub const FG_DARKYELLOW: u16 = 0x0006;
    pub const FG_GREY: u16 = 0x0007;
    pub const FG_DARKGREY: u16 = 0x0008;
    pub const FG_BLUE: u16 = 0x0009;
    pub const FG_GREEN: u16 = 0x000A;
    pub const FG_CYAN: u16 = 0x000B;
    pub const FG_RED: u16 = 0x000C;
    pub const FG_MAGENTA: u16 = 0x000D;
    pub const FG_YELLOW: u16 = 0x000E;
    pub const FG_WHITE: u16 = 0x000F;
    pub const BG_BLACK: u16 = 0x0000;
    pub const BG_DARKBLUE: u16 = 0x0010;
    pub const BG_DARKGREEN: u16 = 0x0020;
    pub const BG_DARKCYAN: u16 = 0x0030;
    pub const BG_DARKRED: u16 = 0x0040;
    pub const BG_DARKMAGENTA: u16 = 0x0050;
    pub const BG_DARKYELLO: u16 = 0x0060;
    pub const BG_GREY: u16 = 0x0070;
    pub const BG_DARKGREY: u16 = 0x0080;
    pub const BG_BLUE: u16 = 0x0090;
    pub const BG_GREEN: u16 = 0x00A0;
    pub const BG_CYAN: u16 = 0x00B0;
    pub const BG_RED: u16 = 0x00C0;
    pub const BG_MAGENTA: u16 = 0x00D0;
    pub const BG_YELLOW: u16 = 0x00E0;
    pub const BG_WHITE: u16 = 0x00F0;
}
