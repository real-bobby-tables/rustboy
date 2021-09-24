extern crate sdl2;
//mod gameboy;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[allow(dead_code)]
fn sdl_loop() -> () {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
    .window("Rustboy 64", 1280, 720)
    .resizable()
    .build()
    .unwrap();



    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {},
            }
        }

        //insert render code here
    }
}

#[allow(dead_code)]
fn read_rom(filename: &str) -> Option<Vec<u8>> {
    let mut rom = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    match rom.read_to_end(&mut buffer)//.expect("Failed to read rom file");
    {
        Ok(v) => {
            println!("Rom size was {0}", v);
            println!("Successfully read rom into memory!");
            Some(buffer)
        },
        Err(_e) => {
            println!("Failed to read rom into memory.");
            None
        }
    }
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Need a rom file to read!");
        return ()
    }

    let rom_name = &args[1];
    println!("Got filename {:?}", rom_name);
    let buff = read_rom(&rom_name);
}
