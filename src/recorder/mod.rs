use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SupportedStreamConfig};
use cpal::platform::{Stream, Device};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use hound::WavWriter;

#[derive(Parser, Debug)]
#[command(version, about = "CPAL record_wav example", long_about = None)]
struct Opt {
    /// The audio device to use
    #[arg(short, long, default_value_t = String::from("default"))]
    device: String,
}

pub struct Recorder {
    pub writer: Option<Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>>,
    pub stream: Option<Stream>,
    pub path: String,
    // pub path: String,
    pub config: SupportedStreamConfig,
    pub device: Device
}

impl<'a> Recorder {
    pub fn new() -> Result<Self, anyhow::Error> {
        let opt = Opt::parse();
    
        let host = cpal::default_host();
    
        // Set up the input device and stream with the default input config.
        let device = if opt.device == "default" {
            host.default_input_device()
        } else {
            host.input_devices()?
                .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
        }
        .expect("failed to find input device");
    
        println!("Input device: {}", device.name()?);
    
        let config = device
            .default_input_config()
            .expect("Failed to get default input config");
        println!("Default input config: {:?}", config);
    
        Ok(Self { writer: None, stream: None, path: super::get_file_path(), config, device })
    }


    pub fn begin_record(&mut self) -> Result<(), anyhow::Error> {
        let spec = wav_spec_from_config(&self.config);
        let writer = hound::WavWriter::create(self.path.as_str(), spec)?;
        let writer = Arc::new(Mutex::new(Some(writer)));
        
        self.writer = Some(writer);

        // Run the input stream on a separate thread.
        let writer_2 = self.writer.as_ref().unwrap().clone();
    
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };
    
        let config = self.config.clone();

        let stream = match self.config.sample_format() {
            cpal::SampleFormat::I8 => self.device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I32 => self.device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
                err_fn,
                None,
            )?,
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
                err_fn,
                None,
            )?,
            sample_format => {
                return Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                )))
            }
        };

        self.stream = Some(stream);

        self.stream.as_ref().unwrap().play()?;
        Ok(())
    }

    pub fn stop_record(&mut self) -> Result<(), anyhow::Error> {
        drop(&self.stream);
        self.writer.as_ref().unwrap().lock().unwrap().take().unwrap().finalize()?;
        println!("Recording {} complete!", self.path);
        Ok(())
    }
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
