use crate::function::Function;
use core::fmt::Write;

static OLED_FONTS: [[u8; 16]; 95] = [
	[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],//  0
	
	[0x00,0x00,0x00,0xF8,0x00,0x00,0x00,0x00,
	0x00,0x00,0x00,0x33,0x30,0x00,0x00,0x00],//\! 1
	
	[0x00,0x10,0x0C,0x06,0x10,0x0C,0x06,0x00,
	0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],//" 2
	
	[0x40,0xC0,0x78,0x40,0xC0,0x78,0x40,0x00,
	0x04,0x3F,0x04,0x04,0x3F,0x04,0x04,0x00],//# 3
	
	[0x00,0x70,0x88,0xFC,0x08,0x30,0x00,0x00,
	0x00,0x18,0x20,0xFF,0x21,0x1E,0x00,0x00],//$ 4
	
	[0xF0,0x08,0xF0,0x00,0xE0,0x18,0x00,0x00,
	0x00,0x21,0x1C,0x03,0x1E,0x21,0x1E,0x00],//% 5
	
	[0x00,0xF0,0x08,0x88,0x70,0x00,0x00,0x00,
	0x1E,0x21,0x23,0x24,0x19,0x27,0x21,0x10],//& 6
	
	[0x10,0x16,0x0E,0x00,0x00,0x00,0x00,0x00,
	0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],//' 7
	
	[0x00,0x00,0x00,0xE0,0x18,0x04,0x02,0x00,
	0x00,0x00,0x00,0x07,0x18,0x20,0x40,0x00],//( 8
	
	[0x00,0x02,0x04,0x18,0xE0,0x00,0x00,0x00,
	0x00,0x40,0x20,0x18,0x07,0x00,0x00,0x00],//) 9
	
	[0x40,0x40,0x80,0xF0,0x80,0x40,0x40,0x00,
	0x02,0x02,0x01,0x0F,0x01,0x02,0x02,0x00],//* 10
	
	[0x00,0x00,0x00,0xF0,0x00,0x00,0x00,0x00,
	0x01,0x01,0x01,0x1F,0x01,0x01,0x01,0x00],//+ 11
	
	[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	0x80,0xB0,0x70,0x00,0x00,0x00,0x00,0x00],//, 12
	
	[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	0x00,0x01,0x01,0x01,0x01,0x01,0x01,0x01],//- 13
	
	[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	0x00,0x30,0x30,0x00,0x00,0x00,0x00,0x00],//. 14
	
	[0x00,0x00,0x00,0x00,0x80,0x60,0x18,0x04,
	0x00,0x60,0x18,0x06,0x01,0x00,0x00,0x00],//\/ 15
	
	[0x00,0xE0,0x10,0x08,0x08,0x10,0xE0,0x00,
	0x00,0x0F,0x10,0x20,0x20,0x10,0x0F,0x00],//0 16
	
	[0x00,0x10,0x10,0xF8,0x00,0x00,0x00,0x00,
	0x00,0x20,0x20,0x3F,0x20,0x20,0x00,0x00],//1 17
	
	[0x00,0x70,0x08,0x08,0x08,0x88,0x70,0x00,
	0x00,0x30,0x28,0x24,0x22,0x21,0x30,0x00],//2 18
	
	[0x00,0x30,0x08,0x88,0x88,0x48,0x30,0x00,
	0x00,0x18,0x20,0x20,0x20,0x11,0x0E,0x00],//3 19
	
	[0x00,0x00,0xC0,0x20,0x10,0xF8,0x00,0x00,
	0x00,0x07,0x04,0x24,0x24,0x3F,0x24,0x00],//4 20
	
	[0x00,0xF8,0x08,0x88,0x88,0x08,0x08,0x00,
	0x00,0x19,0x21,0x20,0x20,0x11,0x0E,0x00],//5 21
	
	[0x00,0xE0,0x10,0x88,0x88,0x18,0x00,0x00,
	0x00,0x0F,0x11,0x20,0x20,0x11,0x0E,0x00],//6 22
	
	[0x00,0x38,0x08,0x08,0xC8,0x38,0x08,0x00,
	0x00,0x00,0x00,0x3F,0x00,0x00,0x00,0x00],//7 23
	
	[0x00,0x70,0x88,0x08,0x08,0x88,0x70,0x00,
	0x00,0x1C,0x22,0x21,0x21,0x22,0x1C,0x00],//8 24
	
	[0x00,0xE0,0x10,0x08,0x08,0x10,0xE0,0x00,
	0x00,0x00,0x31,0x22,0x22,0x11,0x0F,0x00],//9 25
	
	[0x00,0x00,0x00,0xC0,0xC0,0x00,0x00,0x00,
	0x00,0x00,0x00,0x30,0x30,0x00,0x00,0x00],//: 26
	
	[0x00,0x00,0x00,0x80,0x00,0x00,0x00,0x00,
	0x00,0x00,0x80,0x60,0x00,0x00,0x00,0x00],//; 27
	
	[0x00,0x00,0x80,0x40,0x20,0x10,0x08,0x00,
	0x00,0x01,0x02,0x04,0x08,0x10,0x20,0x00],//< 28
	
	[0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x00,
	0x04,0x04,0x04,0x04,0x04,0x04,0x04,0x00],//= 29
	
	[0x00,0x08,0x10,0x20,0x40,0x80,0x00,0x00,
	0x00,0x20,0x10,0x08,0x04,0x02,0x01,0x00],//> 30
	
	[0x00,0x70,0x48,0x08,0x08,0x08,0xF0,0x00,
	0x00,0x00,0x00,0x30,0x36,0x01,0x00,0x00],//? 31
	
	[0xC0,0x30,0xC8,0x28,0xE8,0x10,0xE0,0x00,
	0x07,0x18,0x27,0x24,0x23,0x14,0x0B,0x00],//@ 32
	
	[0x00,0x00,0xC0,0x38,0xE0,0x00,0x00,0x00,
	0x20,0x3C,0x23,0x02,0x02,0x27,0x38,0x20],//A 33
	
	[0x08,0xF8,0x88,0x88,0x88,0x70,0x00,0x00,
	0x20,0x3F,0x20,0x20,0x20,0x11,0x0E,0x00],//B 34
	
	[0xC0,0x30,0x08,0x08,0x08,0x08,0x38,0x00,
	0x07,0x18,0x20,0x20,0x20,0x10,0x08,0x00],//C 35
	
	[0x08,0xF8,0x08,0x08,0x08,0x10,0xE0,0x00,
	0x20,0x3F,0x20,0x20,0x20,0x10,0x0F,0x00],//D 36
	
	[0x08,0xF8,0x88,0x88,0xE8,0x08,0x10,0x00,
	0x20,0x3F,0x20,0x20,0x23,0x20,0x18,0x00],//E 37
	
	[0x08,0xF8,0x88,0x88,0xE8,0x08,0x10,0x00,
	0x20,0x3F,0x20,0x00,0x03,0x00,0x00,0x00],//F 38
	
	[0xC0,0x30,0x08,0x08,0x08,0x38,0x00,0x00,
	0x07,0x18,0x20,0x20,0x22,0x1E,0x02,0x00],//G 39
	
	[0x08,0xF8,0x08,0x00,0x00,0x08,0xF8,0x08,
	0x20,0x3F,0x21,0x01,0x01,0x21,0x3F,0x20],//H 40
	
	[0x00,0x08,0x08,0xF8,0x08,0x08,0x00,0x00,
	0x00,0x20,0x20,0x3F,0x20,0x20,0x00,0x00],//I 41
	
	[0x00,0x00,0x08,0x08,0xF8,0x08,0x08,0x00,
	0xC0,0x80,0x80,0x80,0x7F,0x00,0x00,0x00],//J 42
	
	[0x08,0xF8,0x88,0xC0,0x28,0x18,0x08,0x00,
	0x20,0x3F,0x20,0x01,0x26,0x38,0x20,0x00],//K 43
	
	[0x08,0xF8,0x08,0x00,0x00,0x00,0x00,0x00,
	0x20,0x3F,0x20,0x20,0x20,0x20,0x30,0x00],//L 44
	
	[0x08,0xF8,0xF8,0x00,0xF8,0xF8,0x08,0x00,
	0x20,0x3F,0x00,0x3F,0x00,0x3F,0x20,0x00],//M 45
	
	[0x08,0xF8,0x30,0xC0,0x00,0x08,0xF8,0x08,
	0x20,0x3F,0x20,0x00,0x07,0x18,0x3F,0x00],//N 46
	
	[0xE0,0x10,0x08,0x08,0x08,0x10,0xE0,0x00,
	0x0F,0x10,0x20,0x20,0x20,0x10,0x0F,0x00],//O 47
	
	[0x08,0xF8,0x08,0x08,0x08,0x08,0xF0,0x00,
	0x20,0x3F,0x21,0x01,0x01,0x01,0x00,0x00],//P 48
	
	[0xE0,0x10,0x08,0x08,0x08,0x10,0xE0,0x00,
	0x0F,0x18,0x24,0x24,0x38,0x50,0x4F,0x00],//Q 49
	
	[0x08,0xF8,0x88,0x88,0x88,0x88,0x70,0x00,
	0x20,0x3F,0x20,0x00,0x03,0x0C,0x30,0x20],//R 50
	
	[0x00,0x70,0x88,0x08,0x08,0x08,0x38,0x00,
	0x00,0x38,0x20,0x21,0x21,0x22,0x1C,0x00],//S 51
	
	[0x18,0x08,0x08,0xF8,0x08,0x08,0x18,0x00,
	0x00,0x00,0x20,0x3F,0x20,0x00,0x00,0x00],//T 52
	
	[0x08,0xF8,0x08,0x00,0x00,0x08,0xF8,0x08,
	0x00,0x1F,0x20,0x20,0x20,0x20,0x1F,0x00],//U 53
	
	[0x08,0x78,0x88,0x00,0x00,0xC8,0x38,0x08,
	0x00,0x00,0x07,0x38,0x0E,0x01,0x00,0x00],//V 54
	
	[0xF8,0x08,0x00,0xF8,0x00,0x08,0xF8,0x00,
	0x03,0x3C,0x07,0x00,0x07,0x3C,0x03,0x00],//W 55
	
	[0x08,0x18,0x68,0x80,0x80,0x68,0x18,0x08,
	0x20,0x30,0x2C,0x03,0x03,0x2C,0x30,0x20],//X 56
	
	[0x08,0x38,0xC8,0x00,0xC8,0x38,0x08,0x00,
	0x00,0x00,0x20,0x3F,0x20,0x00,0x00,0x00],//Y 57
	
	[0x10,0x08,0x08,0x08,0xC8,0x38,0x08,0x00,
	0x20,0x38,0x26,0x21,0x20,0x20,0x18,0x00],//Z 58
	
	[0x00,0x00,0x00,0xFE,0x02,0x02,0x02,0x00,
	0x00,0x00,0x00,0x7F,0x40,0x40,0x40,0x00],//[ 59
	
	[0x00,0x0C,0x30,0xC0,0x00,0x00,0x00,0x00,
	0x00,0x00,0x00,0x01,0x06,0x38,0xC0,0x00],//\ 60
	
	[0x00,0x02,0x02,0x02,0xFE,0x00,0x00,0x00,
	0x00,0x40,0x40,0x40,0x7F,0x00,0x00,0x00],//] 61
	
	[0x00,0x00,0x04,0x02,0x02,0x02,0x04,0x00,
	0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],//^ 62
	
	[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80],//_ 63
	
	[0x00,0x02,0x02,0x04,0x00,0x00,0x00,0x00,
	0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],//` 64
	
	[0x00,0x00,0x80,0x80,0x80,0x80,0x00,0x00,
	0x00,0x19,0x24,0x22,0x22,0x22,0x3F,0x20],//a 65
	
	[0x08,0xF8,0x00,0x80,0x80,0x00,0x00,0x00,
	0x00,0x3F,0x11,0x20,0x20,0x11,0x0E,0x00],//b 66
	
	[0x00,0x00,0x00,0x80,0x80,0x80,0x00,0x00,
	0x00,0x0E,0x11,0x20,0x20,0x20,0x11,0x00],//c 67
	
	[0x00,0x00,0x00,0x80,0x80,0x88,0xF8,0x00,
	0x00,0x0E,0x11,0x20,0x20,0x10,0x3F,0x20],//d 68
	
	[0x00,0x00,0x80,0x80,0x80,0x80,0x00,0x00,
	0x00,0x1F,0x22,0x22,0x22,0x22,0x13,0x00],//e 69
	
	[0x00,0x80,0x80,0xF0,0x88,0x88,0x88,0x18,
	0x00,0x20,0x20,0x3F,0x20,0x20,0x00,0x00],//f 70
	
	[0x00,0x00,0x80,0x80,0x80,0x80,0x80,0x00,
	0x00,0x6B,0x94,0x94,0x94,0x93,0x60,0x00],//g 71
	
	[0x08,0xF8,0x00,0x80,0x80,0x80,0x00,0x00,
	0x20,0x3F,0x21,0x00,0x00,0x20,0x3F,0x20],//h 72
	
	[0x00,0x80,0x98,0x98,0x00,0x00,0x00,0x00,
	0x00,0x20,0x20,0x3F,0x20,0x20,0x00,0x00],//i 73
	
	[0x00,0x00,0x00,0x80,0x98,0x98,0x00,0x00,
	0x00,0xC0,0x80,0x80,0x80,0x7F,0x00,0x00],//j 74
	
	[0x08,0xF8,0x00,0x00,0x80,0x80,0x80,0x00,
	0x20,0x3F,0x24,0x02,0x2D,0x30,0x20,0x00],//k 75
	
	[0x00,0x08,0x08,0xF8,0x00,0x00,0x00,0x00,
	0x00,0x20,0x20,0x3F,0x20,0x20,0x00,0x00],//l 76
	
	[0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x00,
	0x20,0x3F,0x20,0x00,0x3F,0x20,0x00,0x3F],//m 77
	
	[0x80,0x80,0x00,0x80,0x80,0x80,0x00,0x00,
	0x20,0x3F,0x21,0x00,0x00,0x20,0x3F,0x20],//n 78
	
	[0x00,0x00,0x80,0x80,0x80,0x80,0x00,0x00,
	0x00,0x1F,0x20,0x20,0x20,0x20,0x1F,0x00],//o 79
	
	[0x80,0x80,0x00,0x80,0x80,0x00,0x00,0x00,
	0x80,0xFF,0xA1,0x20,0x20,0x11,0x0E,0x00],//p 80
	
	[0x00,0x00,0x00,0x80,0x80,0x80,0x80,0x00,
	0x00,0x0E,0x11,0x20,0x20,0xA0,0xFF,0x80],//q 81
	
	[0x80,0x80,0x80,0x00,0x80,0x80,0x80,0x00,
	0x20,0x20,0x3F,0x21,0x20,0x00,0x01,0x00],//r 82
	
	[0x00,0x00,0x80,0x80,0x80,0x80,0x80,0x00,
	0x00,0x33,0x24,0x24,0x24,0x24,0x19,0x00],//s 83
	
	[0x00,0x80,0x80,0xE0,0x80,0x80,0x00,0x00,
	0x00,0x00,0x00,0x1F,0x20,0x20,0x00,0x00],//t 84
	
	[0x80,0x80,0x00,0x00,0x00,0x80,0x80,0x00,
	0x00,0x1F,0x20,0x20,0x20,0x10,0x3F,0x20],//u 85
	
	[0x80,0x80,0x80,0x00,0x00,0x80,0x80,0x80,
	0x00,0x01,0x0E,0x30,0x08,0x06,0x01,0x00],//v 86
	
	[0x80,0x80,0x00,0x80,0x00,0x80,0x80,0x80,
	0x0F,0x30,0x0C,0x03,0x0C,0x30,0x0F,0x00],//w 87
	
	[0x00,0x80,0x80,0x00,0x80,0x80,0x80,0x00,
	0x00,0x20,0x31,0x2E,0x0E,0x31,0x20,0x00],//x 88
	
	[0x80,0x80,0x80,0x00,0x00,0x80,0x80,0x80,
	0x80,0x81,0x8E,0x70,0x18,0x06,0x01,0x00],//y 89
	
	[0x00,0x80,0x80,0x80,0x80,0x80,0x80,0x00,
	0x00,0x21,0x30,0x2C,0x22,0x21,0x30,0x00],//z 90
	
	[0x00,0x00,0x00,0x00,0x80,0x7C,0x02,0x02,
	0x00,0x00,0x00,0x00,0x00,0x3F,0x40,0x40],//[ 91
	
	[0x00,0x00,0x00,0x00,0xFF,0x00,0x00,0x00,
	0x00,0x00,0x00,0x00,0xFF,0x00,0x00,0x00],//| 92
	
	[0x00,0x02,0x02,0x7C,0x80,0x00,0x00,0x00,
	0x00,0x40,0x40,0x3F,0x00,0x00,0x00,0x00],//] 93
	
	[0x00,0x06,0x01,0x01,0x02,0x02,0x04,0x04,
	0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],//~ 94
];

pub trait SSD1306 {
    fn ssd1306_init(&mut self);
    fn ssd1306_clear(&mut self);
    fn _ssd1306_set_cursor(&mut self, y: u8, x: u8);
    fn _ssd1306_show_char(&mut self, line: u8, column: u8, ch: char);
    fn ssd1306_print(&mut self, row: u8, column: u8, string: &str);
}

impl SSD1306 for Function {
    fn ssd1306_init(&mut self) {
        let mut i2c = self.i2c.borrow_mut();
        i2c.write(0x3c, &[0x00, 0xAE]).unwrap();
        i2c.write(0x3c, &[0x00, 0x00]).unwrap();
        i2c.write(0x3c, &[0x00, 0x10]).unwrap();
        i2c.write(0x3c, &[0x00, 0x40]).unwrap();
        i2c.write(0x3c, &[0x00, 0xB0]).unwrap();
        i2c.write(0x3c, &[0x00, 0x81]).unwrap();
        i2c.write(0x3c, &[0x00, 0xFF]).unwrap();
        i2c.write(0x3c, &[0x00, 0xA1]).unwrap();
        i2c.write(0x3c, &[0x00, 0xA6]).unwrap();
        i2c.write(0x3c, &[0x00, 0xA8]).unwrap();
        i2c.write(0x3c, &[0x00, 0x3F]).unwrap();
        i2c.write(0x3c, &[0x00, 0xC8]).unwrap();
        i2c.write(0x3c, &[0x00, 0xD3]).unwrap();
        i2c.write(0x3c, &[0x00, 0x00]).unwrap();
        i2c.write(0x3c, &[0x00, 0xD5]).unwrap();
        i2c.write(0x3c, &[0x00, 0x80]).unwrap();
        i2c.write(0x3c, &[0x00, 0xD9]).unwrap();
        i2c.write(0x3c, &[0x00, 0xF1]).unwrap();
        i2c.write(0x3c, &[0x00, 0xDA]).unwrap();
        i2c.write(0x3c, &[0x00, 0x12]).unwrap();
        i2c.write(0x3c, &[0x00, 0xDB]).unwrap();
        i2c.write(0x3c, &[0x00, 0x30]).unwrap();
        i2c.write(0x3c, &[0x00, 0x8D]).unwrap();
        i2c.write(0x3c, &[0x00, 0x14]).unwrap();
        i2c.write(0x3c, &[0x00, 0xAF]).unwrap();
        i2c.write(0x3c, &[0x00, 0x2e]).unwrap();
        writeln!(self.usart1, "{}", "SSD1306 Initialization Done").unwrap();
    }

    fn ssd1306_clear(&mut self) {
        let mut i2c = self.i2c.borrow_mut();
        for i in 0u8..8 {
            i2c.write(0x3c, &[0x00, 0xB0+i]).unwrap();
            i2c.write(0x3c, &[0x00, 0x00]).unwrap();         
            i2c.write(0x3c, &[0x00, 0x10]).unwrap();
            for _j in 0u8..128 {
                i2c.write(0x3c, &[0x40, 0]).unwrap();
            }
        }
    }

    fn _ssd1306_set_cursor(&mut self,y: u8, x: u8) {
        let mut i2c = self.i2c.borrow_mut();
		let row = x + 2;
        i2c.write(0x3c, &[0x00, 0xb0 | y]).unwrap();
        i2c.write(0x3c, &[0x00, 0x10 | ((row & 0xf0) >> 4)]).unwrap();
        i2c.write(0x3c, &[0x00, 0x00 | (row & 0x0f)]).unwrap();
    }

    fn _ssd1306_show_char(&mut self, row: u8, column: u8, ch: char) {
        {
            self._ssd1306_set_cursor((row - 1) * 2, (column - 1) * 8);
        }
        {
            let mut i2c = self.i2c.borrow_mut();
            for i in 0..8 {
                i2c.write(0x3c, &[0x40, OLED_FONTS[((ch as u32) - ((' ') as u32)) as usize][i]]).unwrap();
            }
        }
        {
            self._ssd1306_set_cursor((row - 1) * 2 + 1, (column - 1) * 8);
        }
        {
            let mut i2c = self.i2c.borrow_mut();
            for i in 0..8 {
                i2c.write(0x3c, &[0x40, OLED_FONTS[((ch as u32) - ((' ') as u32)) as usize][i + 8]]).unwrap();
            }
        }
    }

    fn ssd1306_print(&mut self, row: u8, column: u8, string: &str) {
        let mut counter = 0;
        for x in string.chars() {
            self._ssd1306_show_char(row, column + counter, x);
            counter += 1;
        }
    }

}