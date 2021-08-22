use rand::Rng;

mod constants;

pub struct Chip8 {
    mem: [u8; constants::MEMORY_SIZE],
    regs: [u8; constants::NUM_REGISTERS],
    pc: u16,
    idx: u16,
    stack: [u16; 16],
    sp: i8,
    pub sound: u8,
    delay: u8,
    keyboard: [bool; 16],
    pub screen: [bool; (constants::SCREEN_WIDTH * constants::SCREEN_HEIGHT) as usize],
    pub redraw: bool,
    pub status: CpuStatus,
}

pub enum CpuStatus {
    Running,
    AwaitingKeyPress,
}

impl Chip8 {
    pub fn new(rom_path: String) -> Self {
        let mut cpu = Self {
            mem: [0; constants::MEMORY_SIZE],
            regs: [0; constants::NUM_REGISTERS],
            pc: constants::PC_INITIAL,
            idx: 0,
            stack: [0; 16],
            sp: -1,
            sound: 0,
            delay: 0,
            keyboard: [false; 16],
            screen: [false; (constants::SCREEN_WIDTH * constants::SCREEN_HEIGHT) as usize],
            redraw: false,
            status: CpuStatus::Running,
        };
        for number in 0..80 {
            cpu.mem[number] = constants::FONT_DATA[number];
        }
        let rom = std::fs::read(rom_path).unwrap();
        for (pos, e) in rom.iter().enumerate() {
            cpu.mem[cpu.pc as usize + pos] = *e;
        }
        return cpu;
    }

    pub fn decrement_counters(&mut self) {
        if self.delay > 0 { self.delay = self.delay - 1 }
        if self.sound > 0 { self.sound = self.sound - 1 }
    }

    pub fn tick(&mut self, kb_state: [bool; 16]) {
        self.keyboard = kb_state;
        //fetch
        let instruction = Self::fetch(self);
        //decode
        let (category, x, y, n, nn, nnn) = Self::decode(self, instruction);
        //execute
        match category {
            0x0 => Self::category_0(self, n),
            0x1 => Self::category_1(self, nnn),
            0x2 => Self::category_2(self, nnn),
            0x3 => Self::category_3(self, x, nn),
            0x4 => Self::category_4(self, x, nn),
            0x5 => Self::category_5(self, x, y),
            0x6 => Self::category_6(self, x, nn),
            0x7 => Self::category_7(self, x, nn),
            0x8 => Self::category_8(self, x, y, n),
            0x9 => Self::category_9(self, x, y),
            0xA => Self::category_a(self, nnn),
            0xB => Self::category_b(self, nnn),
            0xC => Self::category_c(self, x, nn),
            0xD => Self::category_d(self, x, y, n),
            0xE => Self::category_e(self, x, n),
            0xF => Self::category_f(self, x, nn),
            _ => {
                panic!("Unknown opcode, panicking.");
            }
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
        match n {
            0x0 => self.screen = [false; (constants::SCREEN_WIDTH * constants::SCREEN_HEIGHT) as usize],
            0xE => {
                self.pc = self.stack[self.sp as usize];
                self.sp = self.sp - 1;
            },
            _ => panic!("Unsupported opcode.")
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
        match n {
            0x0 => self.regs[x as usize] = self.regs[y as usize],
            0x1 => self.regs[x as usize] = self.regs[x as usize] | self.regs[y as usize],
            0x2 => self.regs[x as usize] = self.regs[x as usize] & self.regs[y as usize],
            0x3 => self.regs[x as usize] = self.regs[x as usize] ^ self.regs[y as usize],
            0x4 => {
                let addn: (u8, bool) = self.regs[x as usize].overflowing_add(self.regs[y as usize]);
                self.regs[x as usize] = addn.0;
                if addn.1 {
                    self.regs[0xF] = 1;
                } else {
                    self.regs[0xF] = 0;
                }
            },
            0x5 => {
                let xval = self.regs[x as usize];
                let yval = self.regs[y as usize];
                if xval > yval {
                    self.regs[0xF] = 1;
                    self.regs[x as usize] = xval - yval;
                } else {
                    self.regs[0xF] = 0;
                    self.regs[x as usize] = xval.wrapping_sub(yval);
                }
            },
            0x6 => {
                self.regs[0xF] = (self.regs[x as usize] & 0x1) as u8;
                self.regs[x as usize] = self.regs[x as usize] >> 1;
            },
            0x7 => {
                let xval = self.regs[x as usize];
                let yval = self.regs[y as usize];
                if yval > xval {
                    self.regs[0xF] = 1;
                    self.regs[x as usize] = yval - xval;
                } else {
                    self.regs[0xF] = 0;
                    self.regs[x as usize] = yval.wrapping_sub(xval);
                }
            },
            0xE => {
                self.regs[0xF] = ((self.regs[x as usize] & 0xA0) >> 7) as u8;
                self.regs[x as usize] = self.regs[x as usize] << 1;
            },
            _ => panic!("Unsupported op code")
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
        let xcoord: usize = (self.regs[x as usize] as usize) % (constants::SCREEN_WIDTH as usize);
        let ycoord: usize = (self.regs[y as usize] as usize) % (constants::SCREEN_HEIGHT as usize);
        self.regs[0xF] = 0;
        for number in 0..(n as usize) {
            if ycoord + number >= constants::SCREEN_HEIGHT as usize {
                continue;
            }
            let line = self.mem[(self.idx as usize) + number];
            for x in 0..8 {
                let pix_idx = xcoord + x + ((ycoord + number)* constants::SCREEN_WIDTH as usize);
                if (xcoord + x) >= constants::SCREEN_WIDTH as usize {
                    continue;
                }
                let sprite_pixel = ((line as usize) >> (7 - x)) & 0x1;
                let cur_pixel = self.screen[pix_idx];
                let new_pixel = cur_pixel ^ (sprite_pixel != 0);
                if cur_pixel == true && new_pixel == false {
                    self.regs[0xF] = 1;
                }
                self.screen[pix_idx] = new_pixel;
            }
        }
        self.redraw = true;
    }

    fn category_e(&mut self, x: u8, n: u8) {
        let xval = self.regs[x as usize];
        match n {
            0x1 => {
                if !self.keyboard[xval as usize] {
                    self.pc = self.pc + 2;
                }
            },
            0xE => {
                if self.keyboard[xval as usize] {
                    self.pc = self.pc + 2;
                }
            },
            _ => panic!("Unsupported opcode")
        }
    }

    fn category_f(&mut self, x: u8, nn: u8) {
        match nn {
            0x07 => self.regs[x as usize] = self.delay,
            0x15 => self.delay = self.regs[x as usize],
            0x18 => {
                self.sound = self.regs[x as usize];
            },
            0x1E => {
                let addn: (u16, bool) = self.idx.overflowing_add(self.regs[x as usize] as u16);
                self.idx = addn.0;
            },
            0x0A => {
                self.status = CpuStatus::AwaitingKeyPress;
            },
            0x29 => self.idx = (self.regs[x as usize] as u16) * 5,
            0x33 => {
                let xval = self.regs[x as usize];
                let ones = xval % 10;
                let tens = (xval / 10) % 10;
                let hundreds = (xval / 100) % 10;
                self.mem[self.idx as usize] = hundreds;
                self.mem[(self.idx + 1) as usize] = tens;
                self.mem[(self.idx + 2) as usize] = ones;
            },
            0x55 => {
                for number in 0..(x + 1) as usize {
                    self.mem[(self.idx as usize) + number] = self.regs[number];
                }
            },
            0x65 => {
                for number in 0..(x + 1) as usize {
                    self.regs[number] = self.mem[(self.idx as usize) + number];
                }
            }
            _ => panic!("Unsupported opcode")
        }
    }
}