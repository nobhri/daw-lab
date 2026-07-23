use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        None | Some("generate") => {
            let path = args.next().unwrap_or_else(|| "output/sine_440.wav".into());
            daw_lab::write_default_sine_wav(path)?;
        }
        Some("play") => {
            let path = args.next().unwrap_or_else(|| "output/sine_440.wav".into());
            daw_lab::playback::play_wav(path)?;
        }
        Some("click") => {
            let path = args.next().unwrap_or_else(|| "output/click_120.wav".into());
            daw_lab::write_default_click_wav(path)?;
        }
        Some(command) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown command '{command}'; use 'generate', 'click', or 'play'"),
            )
            .into());
        }
    }

    Ok(())
}
