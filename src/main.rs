mod bluetooth;
mod arg_parser;
mod types;

use arg_parser::{parse_pixel_string, parse_color_string, parse_clock_mode_string, color_hue};
use clap::Parser;
use types::{IDMColor, IDMCommand, IDMPixel, IDMClockMode};
use std::{path::PathBuf, time::Duration};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    screen_on: bool,
    #[arg(long)]
    screen_off: bool,
    /// Pixel in format x,y,#ffffff
    #[arg(long, value_parser = parse_pixel_string)]
    set_pixel: Option<IDMPixel>,
    #[arg(long)]
    image_mode: Option<u8>,
    /// Path to png file
    #[arg(long)]
    upload_png: Option<PathBuf>,
    /// Path to gif file
    #[arg(long)]
    upload_gif: Option<PathBuf>,
    /// Color in hex format, e.g. #ffffff
    #[arg(long, value_parser = parse_color_string)]
    full_screen_color: Option<IDMColor>,
    /// Brightness in percent, e.g. 100
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100))]
    screen_brightness: Option<u8>,
    /// Countdown in seconds
    #[arg(long)]
    countdown_start: Option<u64>,
    #[arg(long)]
    countdown_cancel: bool,
    #[arg(long)]
    countdown_pause: bool,
    #[arg(long)]
    countdown_resume: bool,
    /// Continuously change color demo
    #[arg(long)]
    color_hue: bool,
    /// 0 = default, 1 = christmas, 2 = racing, 3 = inverted full screen, 4 = animated hourglass, 
    /// 5 = frame 1, 6 = frame 2, 7 = frame 3
    #[arg(long, value_parser = parse_clock_mode_string )]
    clock_mode: Option<IDMClockMode>
}

#[tokio::main]
async fn main() {
    let cli = Args::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("idotmatrix=info"))
        .init();

    let bluetooth = bluetooth::BluetoothWrapper::new().await;

    if cli.screen_on {
        bluetooth.send_command(&IDMCommand::ScreenOn).await;
    }

    if cli.screen_off {
        bluetooth.send_command(&IDMCommand::ScreenOff).await;
    }

    if let Some(pixel) = cli.set_pixel {
        bluetooth.send_command(&IDMCommand::SetPixel(pixel)).await;
    }

    if let Some(mode) = cli.image_mode {
        bluetooth.send_command(&IDMCommand::ImageMode(mode)).await;
    }

    if let Some(png) = cli.upload_png {
        let png_data = std::fs::read(png).unwrap();
        bluetooth
            .send_command(&IDMCommand::UploadPng(png_data))
            .await;
    }

    if let Some(gif) = cli.upload_gif {
        let gif_data = std::fs::read(gif).unwrap();
        bluetooth
            .send_command(&IDMCommand::UploadGif(gif_data))
            .await;
    }

    if let Some(color) = cli.full_screen_color {
        bluetooth
            .send_command(&IDMCommand::FullScreenColor(color))
            .await;
    }

    if let Some(brightness) = cli.screen_brightness {
        bluetooth
            .send_command(&IDMCommand::ScreenBrightness(brightness))
            .await;
    }

    if let Some(duration) = cli.countdown_start {
        bluetooth
            .send_command(&IDMCommand::CountdownStart(Duration::from_secs(duration)))
            .await;
    }

    if cli.countdown_cancel {
        bluetooth.send_command(&IDMCommand::CountdownCancel).await;
    }

    if cli.countdown_pause {
        bluetooth.send_command(&IDMCommand::CountdownPause).await;
    }

    if cli.countdown_resume {
        bluetooth.send_command(&IDMCommand::CountdownResume).await;
    }

    if cli.color_hue {
        color_hue(&bluetooth).await;
    }

    if let Some(clock_mode) = cli.clock_mode {
        bluetooth.send_command(&IDMCommand::ClockMode(clock_mode)).await;
    }
}
