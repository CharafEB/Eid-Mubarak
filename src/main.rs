use anyhow::{anyhow, Result};
use ffmpeg_next as ffmpeg;
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{Context as Scaler, flag::Flags};
use ffmpeg_next::util::format::Pixel;
use std::path::Path;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{SetForegroundColor, Color, ResetColor, Print};


use std::io::stdout;
use std::time::{Duration , Instant};
use std::thread;

fn process_video_frames(video_path: &str) -> Result<()> {
    ffmpeg::init().map_err(|e| anyhow!("Failed to initialize FFmpeg: {}", e))?;

    let mut input = ffmpeg::format::input(&Path::new(video_path))
        .map_err(|e| anyhow!("Failed to open input file: {}", e))?;

    let input_stream = input
        .streams()
        .best(Type::Video)
        .ok_or(anyhow!("No video stream found"))?;

    let stream_index = input_stream.index();

    let context_decoder =
        ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let out_width = 50;
    let out_height = 20;

    let mut scaler = Scaler::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        out_width,
        out_height,
        Flags::BILINEAR,
    )?;

    let frame_rate = input_stream.rate(); 
    let fps = frame_rate.0 as f64 / frame_rate.1 as f64;
    let frame_duration = Duration::from_secs_f64(1.0 / fps);

    let mut frame = ffmpeg::frame::Video::empty();
    let mut rgb_frame = ffmpeg::frame::Video::empty();

    for (stream, packet) in input.packets() {
        if stream.index() == stream_index {
            decoder.send_packet(&packet)?;

            while decoder.receive_frame(&mut frame).is_ok() {
                let start = Instant::now();

                scaler.run(&frame, &mut rgb_frame)?;

                let pixel = rgb_frame.data(0);
                let stride = rgb_frame.stride(0) as usize;
                let width = rgb_frame.width() as usize;
                let height = rgb_frame.height() as usize;

                print!("\x1b[2J\x1b[H");

                for h in 0..height {
                    execute!(stdout(), MoveTo(0, h as u16))?;
                    for w in 0..width {
                        let offset = h * stride + w * 3;
                        let r = pixel[offset] as f32;
                        let g = pixel[offset + 1] as f32;
                        let b = pixel[offset + 2] as f32;

                        let brightness = 0.299 * r + 0.587 * g + 0.114 * b;

                        static ASCII_SCALE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

                        let index = (brightness / 255.0 * (ASCII_SCALE.len() - 1) as f32) as usize;
                        let ch = ASCII_SCALE.chars().nth(index).unwrap_or(' ');

                        print!("\x1b[38;2;{};{};{}m{}", r as u8, g as u8, b as u8, ch);
                    }
                    execute!(stdout(), MoveTo(60, h as u16))?;
                    execute!(stdout(), MoveTo(60, h as u16))?;
                                        
                        if h == 5 {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
                                Print(r" _____ _     _  ___  ___      _                     _    "),
                                ResetColor
                            )?;
                        } else if h == 6 {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
                                Print(r"|  ___(_)   | | |  \/  |     | |                   | |   "),
                                ResetColor
                            )?;
                        } else if h == 7 {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
                                Print(r"| |__  _  __| | | .  . |_   _| |__   __ _ _ __ __ _| | __"),
                                ResetColor
                            )?;
                        } else if h == 8 {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
                                Print(r"|  __|| |/ _` | | |\/| | | | | '_ \ / _` | '__/ _` | |/ /"),
                                ResetColor
                            )?;
                        } else if h == 9 {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
                                Print(r"| |___| | (_| | | |  | | |_| | |_) | (_| | | | (_| |   < "),
                                ResetColor
                            )?;
                        } else if h == 10 {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
                                Print(r"\____/|_|\__,_| \_|  |_/\__,_|_.__/ \__,_|_|  \__,_|_|\_\"),
                                ResetColor
                            )?;
                        } else if h == 13 {
                            execute!(stdout(), SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }), Print("✨ عيدكم مبارك ✨"), ResetColor)?;
                        } else if h == 14 {
                            execute!(stdout(), SetForegroundColor(Color::White), Print("كل عام وأنتم بخير وصحة وسعادة وعافية"), ResetColor)?;
                        } else if h == 15 {
                            execute!(stdout(), SetForegroundColor(Color::Rgb { r: 200, g: 162, b: 200 }), Print("🌙 عيد أضحى مبارك 🌙"), ResetColor)?;
                        } else if h == 16 {
                            execute!(stdout(), SetForegroundColor(Color::White), Print("تقبل الله منا ومنكم صالح الأعمال"), ResetColor)?;
                        }
                    println!();
                }

                 let elapsed = start.elapsed();
                    if elapsed < frame_duration {
                    thread::sleep(frame_duration - elapsed);
                }
            }
        }
    }

    print!("\x1b[0m");
    Ok(())
}

fn main() {
    if let Err(e) = process_video_frames("/assets/zino_nhar_lyome.mp4") {
        println!("Error: {}", e);
    }
}   


