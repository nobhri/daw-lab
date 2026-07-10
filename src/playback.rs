use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample};
use std::error::Error;
use std::io;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

pub type PlaybackResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
pub struct AudioData {
    pub samples: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
}

pub fn read_wav<P: AsRef<Path>>(path: P) -> PlaybackResult<AudioData> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let samples = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().collect::<Result<Vec<_>, _>>()?,
        hound::SampleFormat::Int => {
            let scale = 2_f32.powi(i32::from(spec.bits_per_sample) - 1);
            reader
                .samples::<i32>()
                .map(|sample| sample.map(|sample| sample as f32 / scale))
                .collect::<Result<Vec<_>, _>>()?
        }
    };

    Ok(AudioData {
        samples,
        channels: spec.channels,
        sample_rate: spec.sample_rate,
    })
}

pub fn play_wav<P: AsRef<Path>>(path: P) -> PlaybackResult<()> {
    let audio = read_wav(path)?;
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no default audio output device"))?;

    let supported_config = device
        .supported_output_configs()?
        .find(|config| {
            config.min_sample_rate().0 <= audio.sample_rate
                && audio.sample_rate <= config.max_sample_rate().0
        })
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Unsupported,
                format!(
                    "the output device does not support {} Hz playback",
                    audio.sample_rate
                ),
            )
        })?
        .with_sample_rate(cpal::SampleRate(audio.sample_rate));

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();
    let (finished_tx, finished_rx) = mpsc::sync_channel(1);

    let stream = match sample_format {
        SampleFormat::I8 => build_stream::<i8>(&device, &config, audio, finished_tx)?,
        SampleFormat::I16 => build_stream::<i16>(&device, &config, audio, finished_tx)?,
        SampleFormat::I32 => build_stream::<i32>(&device, &config, audio, finished_tx)?,
        SampleFormat::I64 => build_stream::<i64>(&device, &config, audio, finished_tx)?,
        SampleFormat::U8 => build_stream::<u8>(&device, &config, audio, finished_tx)?,
        SampleFormat::U16 => build_stream::<u16>(&device, &config, audio, finished_tx)?,
        SampleFormat::U32 => build_stream::<u32>(&device, &config, audio, finished_tx)?,
        SampleFormat::U64 => build_stream::<u64>(&device, &config, audio, finished_tx)?,
        SampleFormat::F32 => build_stream::<f32>(&device, &config, audio, finished_tx)?,
        SampleFormat::F64 => build_stream::<f64>(&device, &config, audio, finished_tx)?,
        format => {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("unsupported output sample format: {format}"),
            )
            .into())
        }
    };

    stream.play()?;
    finished_rx.recv_timeout(Duration::from_secs(60))?;
    Ok(())
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    audio: AudioData,
    finished_tx: mpsc::SyncSender<()>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: SizedSample + FromSample<f32>,
{
    let output_channels = usize::from(config.channels);
    let input_channels = usize::from(audio.channels);
    let mut frame_index = 0;
    let mut finished = false;

    device.build_output_stream(
        config,
        move |output: &mut [T], _| {
            write_output_data(
                output,
                output_channels,
                &audio.samples,
                input_channels,
                &mut frame_index,
            );

            if !finished && frame_index * input_channels >= audio.samples.len() {
                finished = true;
                let _ = finished_tx.try_send(());
            }
        },
        |error| eprintln!("audio output stream error: {error}"),
        None,
    )
}

fn write_output_data<T>(
    output: &mut [T],
    output_channels: usize,
    input: &[f32],
    input_channels: usize,
    frame_index: &mut usize,
) where
    T: Sample + FromSample<f32>,
{
    for output_frame in output.chunks_mut(output_channels) {
        let input_start = *frame_index * input_channels;

        for (channel, output_sample) in output_frame.iter_mut().enumerate() {
            let sample = input
                .get(input_start + channel.min(input_channels - 1))
                .copied()
                .unwrap_or(0.0);
            *output_sample = T::from_sample(sample);
        }

        if input_start < input.len() {
            *frame_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mono_input_is_copied_to_each_output_channel() {
        let mut output = [0.0_f32; 4];
        let mut frame_index = 0;

        write_output_data(&mut output, 2, &[0.25, -0.5], 1, &mut frame_index);

        assert_eq!(output, [0.25, 0.25, -0.5, -0.5]);
        assert_eq!(frame_index, 2);
    }

    #[test]
    fn output_is_silent_after_input_ends() {
        let mut output = [1.0_f32; 4];
        let mut frame_index = 0;

        write_output_data(&mut output, 2, &[0.25], 1, &mut frame_index);

        assert_eq!(output, [0.25, 0.25, 0.0, 0.0]);
    }
}
