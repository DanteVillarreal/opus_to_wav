// use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_OPUS};
// use symphonia::core::formats::FormatOptions;
// use symphonia::core::io::MediaSourceStream;
// use symphonia::core::meta::MetadataOptions;
// use symphonia::core::probe::Hint;
// use symphonia::default::get_probe;
// use symphonia::core::codecs::CODEC_TYPE_PCM_S16LE;

// use symphonia::core::codecs::CODEC_TYPE_MP3;
// use std::fs::File;
// use std::io::BufReader;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Open the Opus file.
//     let file = File::open(r"D:\Downloads\aisolutions\test_audio\testing_1_2_3.opus")?;
//     let mss = MediaSourceStream::new(Box::new(file), Default::default());


//     // Create a hint to help the format registry guess what format reader is appropriate.
//     let mut hint = Hint::new();
//     hint.with_extension("opus");

//     // Use the default options for format and metadata readers.
//     let format_opts = FormatOptions::default();
//     let metadata_opts = MetadataOptions::default();

//     // Probe the media source.
//     let probed = get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

//     // Get the format reader.
//     let mut format = probed.format;

//     // Collect the track information before the loop.
//     let track = format.tracks().iter().find(|t| t.codec_params.codec == CODEC_TYPE_OPUS).expect("No Opus track found");
//     let track_id = track.id;

//     // Create a decoder for the track.
//     let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

//     // Open the output MP3 file.
//     let output_file = std::fs::File::create("testing_1_2_3.mp3")?;
//     let mut writer = std::io::BufWriter::new(output_file);

//     // Decode the packets and write to the MP3 file.
//     while let Ok(packet) = format.next_packet() {
//         if packet.track_id() == track_id {
//             let decoded = decoder.decode(&packet)?;
//             // Convert the decoded audio data to a SampleBuffer.
//             let mut sample_buffer = symphonia::core::audio::SampleBuffer::<i16>::new(decoded.capacity() as u64, *decoded.spec());
//             sample_buffer.copy_interleaved_ref(decoded);

//             // Convert i16 samples to u8.
//             let samples_u8: &[u8] = bytemuck::cast_slice(sample_buffer.samples());

//             // Write the decoded data to the MP3 file.
//             std::io::Write::write_all(&mut writer, samples_u8)?;
//         }
//     }

//     Ok(())
// }


// use std::process::Command; 
// use tokio;
// use tokio::process::Command as TokioCommand;

// #[tokio::main]
// async fn main() {
//     // Define the input and output file paths
//     let input = r"D:\Downloads\aisolutions\test_audio\testing_1_2_3.opus";
//     let output = r"D:\Downloads\aisolutions\test_audio\attempt0.mp3";

//     // Command to convert the audio file from .opus to .mp3 using ffmpeg
//     // -i specifies the input file
//     // The output file is defined directly
//     let status = TokioCommand::new("ffmpeg")
//         .arg("-i")
//         .arg(input)
//         .arg(output)
//         .status()
//         .await
//         .expect("failed to execute process");

//     if status.success() {
//         println!("Conversion successful!");
//     } else {
//         println!("Conversion failed!");
//     }
// }

// use opus::{Decoder, Channels};
// use std::fs::File;
// use std::io::{self, Read};

// fn main() -> io::Result<()> {
//     // Path to the Opus file
//     let path = "D:\\Downloads\\aisolutions\\test_audio\\testing_1_2_3.opus";
    
//     // Open the Opus file
//     let mut file = File::open(path)?;
    
//     // Read the file contents into a buffer
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;
    
//     // Create an Opus decoder
//     let mut decoder = Decoder::new(48000, Channels::Mono).unwrap();

//     // Get the actual sample rate the decoder was initialized with
//     let sample_rate = decoder.get_sample_rate().unwrap();
//     println!("Decoder initialized with sample rate: {} Hz", sample_rate);


//     // Calculate the maximum frame size for the given sample rate
//     let max_frame_size = (sample_rate as usize / 1000) * 60; // 60 ms frame size
    
//     // Create an output buffer with the maximum frame size
//     let mut output = vec![0; max_frame_size];

//     // Decode the Opus data
//     let mut offset = 0;
//     while offset < buffer.len() {
//         // Find the next packet boundary (this is a simplified example)
//         let packet_end = offset + 1275; // Adjust this based on your packet size
//         let packet = &buffer[offset..packet_end.min(buffer.len())];
        
//         match decoder.decode(packet, &mut output, false) {
//             Ok(decoded_samples) => {
//                 println!("Decoded {} samples", decoded_samples);
//                 offset += packet.len(); // Move to the next packet
//             }
//             Err(e) => {
//                 println!("Error decoding packet: {:?}", e);
//                 break;
//             }
//         }
//     }
    
//     // Process the decoded samples (e.g., save to a WAV file or play the audio)
//     //println!("Decoded {} samples", decoded_samples);
    
//     Ok(())
// }


// use opus::Decoder;
// use std::{fs::File, io::{BufReader, Cursor, Read}};
// use lewton::inside_ogg::OggStreamReader;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let file = BufReader::new(File::open("D:\\Downloads\\aisolutions\\test_audio\\testing_1_2_3.opus")?);
//     let mut ogg_reader = OggStreamReader::new(file)?;

//     // Assuming 48000 Hz sample rate, 2 channels (stereo)
//     let mut decoder = Decoder::new(48000, opus::Channels::Stereo)?;
//     let frame_size = 960;  // number of samples to decode per frame
//     let max_output_size = frame_size * 2 * 2;  // 2 channels, 2 bytes per sample
//     let mut output = vec![0; max_output_size];

//     while let Some(packet) = ogg_reader.read_dec_packet_itl()? {
//         let packet_bytes: Vec<u8> = packet.iter().flat_map(|&sample| sample.to_le_bytes()).collect();
//         let len = decoder.decode(&packet_bytes, &mut output[..], false)?;
//         // len is the number of samples decoded into the output buffer
    
//         println!("{:?}", &output[..len * 2 * 2]); // len samples per channel, 2 channels, 2 bytes per sample
//     }

//     Ok(())
// }



//----------------------WORKS BUT WRONG SAMPLE RATE------------------------//

        // use ogg_opus::decode;
        // use std::fs::File;
        // use std::io::{self};
        // use wav::{BitDepth, Header};

        // fn main() -> io::Result<()> {
        //     // Load your Ogg Opus file
        //     let mut f = File::open("D:\\Downloads\\aisolutions\\test_audio\\testing_1_2_3.opus")?;

        //     // Use ogg-opus to decode the file to raw audio samples
        //     let (raw_audio, header) = decode::<_, 48000>(&mut f).unwrap();

        //     // Convert raw audio samples to a writable format
        //     let raw_audio: Vec<i16> = raw_audio.to_vec();
        //     let mut samples: Vec<u8> = Vec::new();

        //     for sample in &raw_audio {
        //         samples.extend_from_slice(&sample.to_le_bytes());
        //     }

        //     // Create a WAV header
        //     let header = Header::new(1, 2, 48000, 16);
        //     let bit_depth = BitDepth::Sixteen(raw_audio);
            
        //     // Write the decoded audio samples to a WAV file
        //     let mut output_file = File::create("D:\\Downloads\\aisolutions\\test_audio\\attempt1.wav")?;
        //     wav::write(header, &bit_depth, &mut output_file).unwrap();

        //     println!("Decoded audio saved to output.wav");

        //     Ok(())
        // }


//
//
use ogg_opus::decode;
use std::fs::File;
use std::io::{self};
use hound;

fn main()  {
    // Load your Ogg Opus file
    let mut f = File::open("D:\\Downloads\\aisolutions\\test_audio\\testing_1_2_3.opus").expect("could not open file");

    // Use ogg-opus to decode the file to raw audio samples
    let (raw_audio, header) = decode::<_, 48000>(&mut f).unwrap();

    // Create a WAV writer using hound
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("D:\\Downloads\\aisolutions\\test_audio\\attempt1.wav", spec).expect("write could not work");

    // Write the decoded audio samples to WAV file
    for sample in raw_audio {
        writer.write_sample(sample).expect("write sample no worky");
    }

    writer.finalize().expect("could not finalize");  // Finalize the WAV file

    println!("Decoded audio saved to attempt1.wav");


}
