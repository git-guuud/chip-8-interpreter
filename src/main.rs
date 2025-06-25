use pixels;
use std::fs;
// use std::thread;
// use std::sync::{Mutex, Arc};
use rand::Rng;
use winit::event_loop::{EventLoop};
use winit::window::WindowAttributes;
use winit::dpi::{PhysicalSize};
use winit::event::KeyEvent;
use winit::keyboard::{PhysicalKey, KeyCode};

struct State {
    memory: [u8; 4096],
    pc: u16,
    frame_buffer: [u8; 8 * 32],
    stack: Vec<u16>,
    registers: [u8; 16],
    I: u16,
    MOVE_VAL_8XY6E: bool,
    BXNN: bool,
    INCREMENT_I_ON_LOAD: bool,
    key: u16,
    key_pressed_this_frame: u16,
    key_released_this_frame: u16,
    delay_timer: u8,
    sound_timer: u8,
}


fn main() {

    let mut state = State {
        memory: [0u8; 4096],
        pc: 0x200,
        frame_buffer: [0u8; 8 * 32],
        stack: Vec::new(),
        registers: [0u8; 16],
        I: 0,
        MOVE_VAL_8XY6E: false,
        BXNN: false,
        INCREMENT_I_ON_LOAD: true,
        key: 0,
        key_pressed_this_frame: 0,
        key_released_this_frame: 0,
        delay_timer: 0,
        sound_timer: 0,
    };

    let font = [
        0xF0u8, 0x90, 0x90, 0x90, 0xF0, // 0
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

    // Load the font into memory
    for i in 0..80 {
        state.memory[i + 0x50] = font[i];
    }

    let code = fs::read("./Astro Dodge.ch8").expect("Failed to read file");
    for i in 0..code.len() {
        state.memory[i + 0x200] = code[i];
    }

    let event_loop = EventLoop::new().unwrap();
    let window_attributes = WindowAttributes::default()
    .with_title("CHIP-8 Emulator")
    .with_inner_size(PhysicalSize::new(64 * 20, 32 * 20))
    .with_resizable(false);
    let window = event_loop.create_window(window_attributes).unwrap();

    
    let mut display = pixels::PixelsBuilder::new(
        64,
        32,
        pixels::SurfaceTexture::new(1280, 640, &window),
    )
    .build()
    .unwrap();
    event_loop.run( |event, elwt| {
        elwt.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(16)));
        state.delay_timer = state.delay_timer.saturating_sub(1);
        state.sound_timer = state.sound_timer.saturating_sub(1);
        if state.sound_timer > 0 {
            // Play sound
        }

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        elwt.exit();
                        return;
                    }
                    winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                        let i = get_input(event);
                        state.key_pressed_this_frame = i.0;
                        state.key_released_this_frame = i.1;
                        state.key |= i.0;
                        state.key &= !i.1;
                    }
                    _ => {}
                }
            }
            // winit::event::Event::AboutToWait => {
            //     window.request_redraw();
            //     elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);
            // }
            // winit::event::Event::AboutToWait => {
            //     for _ in 0..20 {
            //         main_loop(&mut state, &window);
            //     }
            // }
            _ => {}
        }
        
        for _ in 0..10 {
            main_loop(&mut state, &window);
        }

        for i in 0..(64 * 32) {
            if state.frame_buffer[i / 8] & (1 << 7 - (i % 8)) == 0 {
                display.frame_mut()[i * 4 + 0] = 0x00;
                display.frame_mut()[i * 4 + 1] = 0x00;
                display.frame_mut()[i * 4 + 2] = 0x00;
                display.frame_mut()[i * 4 + 3] = 0xFF; 
            }
            else {
                display.frame_mut()[i * 4 + 0] = 0xFF; 
                display.frame_mut()[i * 4 + 1] = 0xFF;
                display.frame_mut()[i * 4 + 2] = 0xFF; 
                display.frame_mut()[i * 4 + 3] = 0xFF;
            }
        }
        
        display.render().unwrap();

    }).expect("Failed to run event loop");
}

fn main_loop(state: &mut State, window: &winit::window::Window) {
    // for i in 0..(8 * 32) {
    //     if frame_buffer[i] == 255 {
    //         frame_buffer[i] = 0;
    //     } else {
    //         frame_buffer[i] += 1;
    //     };
    // }
    let instruction: u16 = ((state.memory[state.pc as usize] as u16) << 8) | (state.memory[(state.pc + 1) as usize] as u16);
    state.pc += 2;
    // println!("Executing instruction: {:#04X}", instruction);
    // println!("PC: {:#04X}", state.pc);

    let opcode = ((instruction & 0xF000) >> 12) as u8; 
    let x = ((instruction & 0x0F00) >> 8) as u8;
    let y = ((instruction & 0x00F0) >> 4) as u8;
    let n = instruction & 0x000F;
    let nn = instruction & 0x00FF;
    let nnn = instruction & 0x0FFF;

    match instruction {
        _ if opcode == 0x1 => {
            state.pc = nnn;
        }
        _ if opcode == 0x2 => {
            state.stack.push(state.pc);
            state.pc = nnn;
        }
        _ if opcode == 0x3 => {
            if state.registers[x as usize] == nn as u8 {
                state.pc += 2;
            }
        }
        _ if opcode == 0x4 => {
            if state.registers[x as usize] != nn as u8 {
                state.pc += 2;
            }
        }
        _ if opcode == 0x5 && n == 0 => {
            if state.registers[x as usize] == state.registers[y as usize] {
                state.pc += 2;
            }
        }
        _ if opcode == 0x6 => {
            state.registers[x as usize] = nn as u8; 
        }
        _ if opcode == 0x7 => {
            state.registers[x as usize] = state.registers[x as usize].wrapping_add(nn as u8);
            // state.registers[x as usize] += nn as u8;
        }
        _ if opcode == 0x8 => {
            match n {
                0 => {state.registers[x as usize] = state.registers[y as usize]}
                1 => {state.registers[x as usize] |= state.registers[y as usize]},
                2 => {state.registers[x as usize] &= state.registers[y as usize]},
                3 => {state.registers[x as usize] ^= state.registers[y as usize]},
                4 => {
                    let (result, overflow) = state.registers[x as usize].overflowing_add(state.registers[y as usize]);
                    state.registers[x as usize] = result;
                    state.registers[0xF] = if overflow { 1 } else { 0 };
                }
                5 => {
                    if state.registers[x as usize] > state.registers[y as usize] {
                        state.registers[0xF] = 1;
                        state.registers[x as usize] -= state.registers[y as usize];
                    }
                    else {
                        state.registers[0xF] = 0;
                        state.registers[x as usize] = state.registers[x as usize].overflowing_sub(state.registers[y as usize]).0;
                    }
                }
                7 => {
                    if state.registers[x as usize] < state.registers[y as usize] {
                        state.registers[0xF] = 1;
                    }
                    else {
                        state.registers[0xF] = 0;
                    }
                    state.registers[x as usize] = state.registers[y as usize].overflowing_sub(state.registers[x as usize]).0;
                }
                6 => {
                    if state.MOVE_VAL_8XY6E {
                        state.registers[x as usize] = state.registers[y as usize];
                    }
                    let (result, overflow) = state.registers[x as usize].overflowing_shr(1);
                    state.registers[x as usize] = result;
                    state.registers[0xF] = if overflow {1} else {0};
                }
                0xE => {
                    if state.MOVE_VAL_8XY6E {
                        state.registers[x as usize] = state.registers[y as usize];
                    }
                    state.registers[0xF] = if state.registers[x as usize]&1 == 1 {1} else {0};
                    let result= state.registers[x as usize].overflowing_shl(1).0;
                    state.registers[x as usize] = result;
                }
                _ => {}
            }
        }
        _ if opcode == 0x9 && n == 0 => {
            if state.registers[x as usize] != state.registers[y as usize] {
                state.pc += 2;
            }
        }
        _ if opcode == 0xA => {
            state.I = nnn; 
        }
        _ if opcode == 0xD => {
            state.registers[0xF] = 0; 
            let x_pos = (state.registers[x as usize] % 64) as usize;
            let mut y_pos = (state.registers[y as usize] % 32) as usize;
            for i in 0..n {
                if y_pos == 32 {
                    break;
                }
                
                let row = state.memory[(state.I + i as u16) as usize];
                if state.frame_buffer[y_pos*8 + x_pos/8] & (row >> (x_pos % 8)) != 0 {
                    state.registers[0xF] = 1;
                }
                state.frame_buffer[y_pos*8 + x_pos/8] ^= row >> (x_pos % 8);
                if x_pos/8 == 7 { y_pos += 1;continue;}
                if state.frame_buffer[y_pos*8 + x_pos/8 + 1] & (row.overflowing_shl(8-(x_pos % 8) as u32).0) != 0 {
                    state.registers[0xF] = 1;
                }
                let (result, overflow) = row.overflowing_shl(8-(x_pos % 8) as u32);
                if !overflow {
                    state.frame_buffer[y_pos*8 + x_pos/8 + 1] ^= result;
                }

                y_pos += 1;
            }
            // window.request_redraw();
        }
        _ if opcode == 0xB => {
            if state.BXNN {
                state.pc = nnn + state.registers[x as usize] as u16;
            } else {
                state.pc = nnn + state.registers[0] as u16;
            }
        }
        _ if opcode == 0xC => {
            let r = rand::rng().random_range(0..255);
            state.registers[x as usize] = r&nn as u8;
        }
        _ if opcode == 0xE => {
            if nn == 0x9E {
                if state.key_pressed_this_frame & (1 << state.registers[x as usize]) != 0 {
                    state.pc += 2;
                }
            } else if nn == 0xA1 {
                if state.key_pressed_this_frame & (1 << state.registers[x as usize]) == 0 {
                    state.pc += 2;
                }
            }
        }

        0x00E0 => {
            for i in 0..(8 * 32) {
                state.frame_buffer[i] = 0;
            }
        }
        0x00EE => {
            state.pc = state.stack.pop().expect("No value in stack to pop.");
        }

        _ if opcode == 0xF => {
            match nn {
                0x07 => {
                    state.registers[x as usize] = state.delay_timer;
                }
                0x15 => {
                    state.delay_timer = state.registers[x as usize];
                }
                0x18 => {
                    state.sound_timer = state.registers[x as usize];
                }
                0x1E => {
                    state.I += state.registers[x as usize] as u16;
                    if state.I > 0xFFF {
                        state.registers[0xF] = 1; 
                    } else {
                        state.registers[0xF] = 0; 
                    }
                }
                0x0A => {
                    for i in 0..16{
                        if state.key_pressed_this_frame & (1 << i) != 0 {
                            state.registers[x as usize] = i as u8;
                            break;
                        }
                    }
                    if state.key_pressed_this_frame == 0 {state.pc -= 2;}
                }
                0x29 => {
                    state.I = (state.registers[x as usize] & 0x0F) as u16 * 5 + 0x50;
                }
                0x33 => {
                    let value = state.registers[x as usize];
                    state.memory[state.I as usize] = value / 100;
                    state.memory[(state.I + 1) as usize] = (value / 10) % 10;
                    state.memory[(state.I + 2) as usize] = value % 10;
                }
                0x55 => {
                    for i in 0..=x as usize {
                        state.memory[state.I as usize + i] = state.registers[i];
                    }
                    if state.INCREMENT_I_ON_LOAD {
                        state.I += x as u16 + 1;
                    }
                }
                0x65 => {
                    for i in 0..=x as usize {
                        state.registers[i] = state.memory[state.I as usize + i];
                    }
                    if state.INCREMENT_I_ON_LOAD {
                        state.I += x as u16 + 1;
                    }
                }
                _ => {}
            }
        }

        _ => {}
    }
    // let mut input: String = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn get_input(event: KeyEvent) -> (u16, u16){
    match event {
        KeyEvent { physical_key, state, .. } => {
            if let PhysicalKey::Code(key) = physical_key {
                let p = match key {
                    KeyCode::Digit1 => 1<<0x1,
                    KeyCode::Digit2 => 1<<0x2,
                    KeyCode::Digit3 => 1<<0x3,
                    KeyCode::Digit4 => 1<<0xC,
                    KeyCode::KeyQ => 1<<0x4,
                    KeyCode::KeyW => 1<<0x5,
                    KeyCode::KeyE => 1<<0x6,
                    KeyCode::KeyR => 1<<0xD,
                    KeyCode::KeyA => 1<<0x7,
                    KeyCode::KeyS => 1<<0x8,
                    KeyCode::KeyD => 1<<0x9,
                    KeyCode::KeyF => 1<<0xE,
                    KeyCode::KeyZ => 1<<0xA,
                    KeyCode::KeyX => 1<<0x0,
                    KeyCode::KeyC => 1<<0xB,
                    KeyCode::KeyV => 1<<0xF,
                    _ => 0
                };
                if state.is_pressed() {
                    return (p, 0);
                } else {
                    return (0, p);
                }
            }
            else {
                return (0, 0);
            }
        }
    }
}
