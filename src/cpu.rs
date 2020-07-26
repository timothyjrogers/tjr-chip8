#[path = "screen.rs"]
mod screen;
extern crate sdl2;
use rand::Rng;

pub struct Cpu {
    mem: [u8; 4096],
    regs: [u8; 16],
    pc: u16,
    idx: u16,
    stack: [u16; 16],
    sp: i8,
    sound: u8,
    delay: u8,
    screen: screen::Screen,
    keypad: [bool; 16],
    wait_key: bool,
    key_reg: usize,
    beep: bool,
    running: bool,
}

impl Cpu {
    pub fn new(ctx: &sdl2::Sdl) -> Cpu {
        let screen = screen::Screen::new(ctx);
        let mut cpu = Cpu {
            mem: [0; 4096],
            regs: [0; 16],
            pc: 0x200,
            idx: 0,
            stack: [0; 16],
            sp: -1,
            sound: 0,
            delay: 0,
            screen: screen,
            keypad: [false; 16],
            wait_key: false,
            key_reg: 0,
            beep: false,
            running: true,
        };
        cpu.sound = 50;
        return cpu;
    }

    pub fn running(&self) -> bool {
        return self.running;
    }

    pub fn halt(&mut self) {
        self.running = false;
    }

    pub fn set_keypad(&mut self, num: usize, val: bool) {
        self.keypad[num] = val;
    }

    pub fn is_waiting(&self) -> bool {
        return self.wait_key;
    }

    pub fn set_waiting(&mut self, val: bool) {
        self.wait_key = val;
    }

    pub fn set_reg(&mut self, x: usize, val: u8) {
        self.regs[x] = val;
    }

    pub fn get_key_reg(&mut self) -> usize {
        return self.key_reg;
    }

    pub fn decrement_counters(&mut self) {
        if self.delay > 0 {
            self.delay = self.delay - 1;
        }
        if self.sound > 0 {
            self.sound = self.sound - 1;
        }
        if self.sound == 0 {
            self.beep = false;
        }
    }

    pub fn get_beep(&self) -> bool{
        return self.beep;
    }

    pub fn load_font(&mut self) {
        let font_data: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for number in 0..80 {
            self.mem[number] = font_data[number];
        }
    }

    pub fn load_rom(&mut self, fname: &str) {
        let rom = std::fs::read(fname).unwrap();
        let mem_start = 0x200;
        for (pos, e) in rom.iter().enumerate() {
            self.mem[mem_start + pos] = *e;
        }
    }

    pub fn tick(&mut self) {
        //fetch
        let instruction = Self::fetch(self);
        //decode
        let (category, x, y, n, nn, nnn) = Self::decode(self, instruction);
        //execute
        if category == 0x0 {
            Self::category_0(self, n);
        } else if category == 0x1 {
            Self::category_1(self, nnn);
        } else if category == 0x2 {
            Self::category_2(self, nnn);
        } else if category == 0x3 {
            Self::category_3(self, x, nn);
        } else if category == 0x4 {
            Self::category_4(self, x, nn);
        }  else if category == 0x5 {
            Self::category_5(self, x, y);
        } else if category == 0x6 {
            Self::category_6(self, x, nn);
        } else if category == 0x7 {
            Self::category_7(self, x, nn);
        } else if category == 0x8 {
            Self::category_8(self, x, y, n);
        } else if category == 0x9 {
            Self::category_9(self, x, y);
        } else if category == 0xA {
            Self::category_a(self, nnn);
        } else if category == 0xB {
            Self::category_b(self, nnn);
        } else if category == 0xC {
            Self::category_c(self, x, nn);
        } else if category == 0xD {
            Self::category_d(self, x, y, n);
        } else if category == 0xE {
            Self::category_e(self, x, n);
        } else if category == 0xF {
            Self::category_f(self, x, nn);
        }
    }

    fn fetch(&mut self) -> u16 {
        let mut opcode: u16 = 0;
        opcode = opcode + (self.mem[self.pc as usize]) as u16;
        opcode = opcode << 8;
        opcode = opcode + (self.mem[(self.pc+1) as usize]) as u16;
        self.pc = self.pc + 2;
        return opcode;
    }

    fn decode(&self, instruction: u16) -> (u8, u8, u8, u8, u8, u16) {
        let category: u8 = (instruction >> 12) as u8;
        let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
        let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
        let n: u8 = (instruction & 0x000F) as u8;
        let nn: u8 = (instruction & 0x00FF) as u8;
        let nnn: u16 = instruction & 0x0FFF;
        return (category, x, y, n, nn, nnn);
    }

    fn category_0(&mut self, n: u8) {
        if n == 0 {
            self.screen.clear_screen();
        } else if n == 0xE {
            self.pc = self.stack[self.sp as usize];
            self.sp = self.sp - 1;
        }
    }

    fn category_1(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn category_2(&mut self, nnn: u16) {
        self.sp = self.sp + 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
    }

    fn category_3(&mut self, x: u8, nn: u8) {
        let xval: u8 = self.regs[x as usize];
        if xval == nn {
            self.pc = self.pc + 2;
        }
    }

    fn category_4(&mut self, x: u8, nn: u8) {
        let xval: u8 = self.regs[x as usize];
        if xval != nn {
            self.pc = self.pc + 2;
        }
    }

    fn category_5(&mut self, x: u8, y: u8) {
        let xval: u8 = self.regs[x as usize];
        let yval: u8 = self.regs[y as usize];
        if xval == yval {
            self.pc = self.pc + 2;
        }
    }

    fn category_6(&mut self, x: u8, nn: u8) {
        self.regs[x as usize] = nn;
    }

    fn category_7(&mut self, x: u8, nn: u8) {
        let addn: (u8, bool) = self.regs[x as usize].overflowing_add(nn);
        self.regs[x as usize] = addn.0;
    }

    fn category_8(&mut self, x: u8, y: u8, n: u8) {
        if n == 0 {
            self.regs[x as usize] = self.regs[y as usize];
        } else if n == 1 {
            self.regs[x as usize] = self.regs[x as usize] | self.regs[y as usize];
        } else if n == 2 {
            self.regs[x as usize] = self.regs[x as usize] & self.regs[y as usize];
        } else if n == 3 {
            self.regs[x as usize] = self.regs[x as usize] ^ self.regs[y as usize];
        } else if n == 4 {
            let addn: (u8, bool) = self.regs[x as usize].overflowing_add(self.regs[y as usize]);
            self.regs[x as usize] = addn.0;
            if addn.1 {
                self.regs[0xF] = 1;
            } else {
                self.regs[0xF] = 0;
            }
        } else if n == 5 {
            let xval = self.regs[x as usize];
            let yval = self.regs[y as usize];
            if xval > yval {
                self.regs[0xF] = 1;
                self.regs[x as usize] = xval -  yval;
            } else {
                self.regs[0xF] = 0;
                self.regs[x as usize] = xval.wrapping_sub(yval);
            }
        } else if n == 6 {
            self.regs[0xF] = (self.regs[x as usize] & 0x1) as u8;
            self.regs[x as usize] = self.regs[x as usize] >> 1;
        } else if n == 7 {
            let xval = self.regs[x as usize];
            let yval = self.regs[y as usize];
            if yval > xval {
                self.regs[0xF] = 1;
                self.regs[x as usize] = yval - xval;
            } else {
                self.regs[0xF] = 0;
                self.regs[x as usize] = yval.wrapping_sub(xval);
            }
        } else if n == 0xE {
            self.regs[0xF] = ((self.regs[x as usize] & 0xA0) >> 7) as u8;
            self.regs[x as usize] = self.regs[x as usize] << 1;
        }
    }

    fn category_9(&mut self, x: u8, y: u8) {
        let xval: u8 = self.regs[x as usize];
        let yval: u8 = self.regs[y as usize];
        if xval != yval {
            self.pc = self.pc + 2;
        }
    }

    fn category_a(&mut self, nnn: u16) {
        self.idx = nnn;
    }

    fn category_b(&mut self, nnn: u16) {
        self.pc = (self.regs[0] as u16) + nnn;
    }

    fn category_c(&mut self, x: u8, nn: u8) {
        let rnum: u8 = rand::thread_rng().gen_range(0, 256) as u8;
        self.regs[x as usize] = rnum & nn;
    }

    fn category_d(&mut self, x: u8, y: u8, n: u8) {
        let xcoord: usize = (self.regs[x as usize] as usize) % (screen::SCREEN_WIDTH_BASE);
        let ycoord: usize = (self.regs[y as usize] as usize) % (screen::SCREEN_HEIGHT_BASE);
        self.regs[0xF] = 0;
        for number in 0..(n as usize) {
            if ycoord + number >= screen::SCREEN_HEIGHT_BASE {
                continue;
            }
            let line = self.mem[(self.idx as usize) + number];
            for x in 0..8 {
                let pix_idx = xcoord + x + ((ycoord + number)* screen::SCREEN_WIDTH_BASE);
                if (xcoord + x) >= screen::SCREEN_WIDTH_BASE {
                    continue;
                }
                let sprite_pixel = ((line as usize) >> (7 - x)) & 0x1;
                let cur_pixel = self.screen.get_pixel(pix_idx);
                let new_pixel = cur_pixel ^ (sprite_pixel != 0);
                if cur_pixel == true && new_pixel == false {
                    self.regs[0xF] = 1;
                }
                self.screen.set_pixel(pix_idx, new_pixel);
            }
        }
        self.screen.draw();
    }

    fn category_e(&mut self, x: u8, n: u8) {
        let xval = self.regs[x as usize];
        if n == 0xE {
            if self.keypad[xval as usize] {
                self.pc = self.pc + 2;
            }
        } else if n == 1 {
            if !self.keypad[xval as usize] {
                self.pc = self.pc + 2;
            }
        }
    }

    fn category_f(&mut self, x: u8, nn: u8) {
        if nn == 0x07 {
            self.regs[x as usize] = self.delay;
        } else if nn == 0x15 {
            self.delay = self.regs[x as usize];
        } else if nn == 0x18 {
            self.sound = self.regs[x as usize];
            if self.sound > 0 {
                self.beep = true;
            }
        } else if nn == 0x1E {
            let addn: (u16, bool) = self.idx.overflowing_add(self.regs[x as usize] as u16);
            self.idx = addn.0;
        } else if nn == 0x0A {
            self.wait_key = true;
            self.key_reg = x as usize;
        } else if nn == 0x29 {
            self.idx = (self.regs[x as usize] as u16) * 5;
        } else if nn == 0x33 {
            let xval = self.regs[x as usize];
            let ones = xval % 10;
            let tens = (xval / 10) % 10;
            let hundreds = (xval / 100) % 10;
            self.mem[self.idx as usize] = hundreds;
            self.mem[(self.idx + 1) as usize] = tens;
            self.mem[(self.idx + 2) as usize] = ones;
        } else if nn == 0x55 {
            for number in 0..(x + 1) as usize {
                self.mem[(self.idx as usize) + number] = self.regs[number];
            }
        } else if nn == 0x65 {
            for number in 0..(x + 1) as usize {
                self.regs[number] = self.mem[(self.idx as usize) + number];
            }
        }
    }

}