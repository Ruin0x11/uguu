#[macro_use]
extern crate clap;

extern crate bitreader;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use bitreader::BitReader;
use clap::{App, Arg};

pub enum UguuError {
    IoError(std::io::Error),
    BitError(bitreader::BitReaderError),
    UtfError(std::str::Utf8Error),
    MyError(String),
}

impl From<bitreader::BitReaderError> for UguuError {
    fn from(e: bitreader::BitReaderError) -> UguuError {
        UguuError::BitError(e)
    }
}

impl From<std::io::Error> for UguuError {
    fn from(e: std::io::Error) -> UguuError {
        UguuError::IoError(e)
    }
}

impl From<std::str::Utf8Error> for UguuError {
    fn from(e: std::str::Utf8Error) -> UguuError {
        UguuError::UtfError(e)
    }
}

impl std::fmt::Display for UguuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            UguuError::IoError(ref e) => write!(f, "{}", e),
            UguuError::BitError(ref e) => write!(f, "{}", e),
            UguuError::UtfError(ref e) => write!(f, "{}", e),
            UguuError::MyError(ref e) => write!(f, "{}", e),
        }
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(format!("uguu {}", crate_version!()))
        .arg(Arg::with_name("version").short("V").long("version").help(
            "Uguu?",
        ))
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Uguu~!"),
        )
        .arg(Arg::with_name("decode").short("d").long("decode").help(
            "Uguu...",
        ))
        .arg(Arg::with_name("input").value_name("INPUT").required(true))
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("Uguu!!!: {}", e);

        ::std::process::exit(1);
    }
}

fn run() -> Result<(), UguuError> {
    let matches = app().get_matches();
    if matches.is_present("version") {
        println!("uguu {}", crate_version!());
        return Ok(());
    }

    let input_name = matches.value_of("input").ok_or(UguuError::MyError(
        format!("No input."),
    ))?;

    let input_path = Path::new(input_name);

    let input = File::open(input_path)?;

    let output_name = match matches.value_of("output") {
        Some(o) => o.to_string(),
        None => {
            if uguuuu(input_path) {
                format!("{}", input_path.file_stem().unwrap().to_str().unwrap())
            } else {
                format!("{}.uguu", input_path.file_stem().unwrap().to_str().unwrap())
            }
        }
    };

    let output = File::create(output_name)?;

    if matches.is_present("decode") {
        uuugu(input, output)
    } else if matches.is_present("encode") {
        uguuu(input, output)
    } else if uguuuu(input_path) {
        uuugu(input, output)
    } else {
        uguuu(input, output)
    }
}

fn uguuu<R: Read, W: Write>(mut input: R, mut output: W) -> Result<(), UguuError> {
    let mut buffer = [0; 128];

    while let Ok(count) = input.read(&mut buffer) {
        if count == 0 {
            break;
        }
        let mut reader = BitReader::new(&buffer);
        for _ in 0..(count * 4) {
            let bits = reader.read_u8(2)?;
            let uguu = uguu(bits);
            let mut uguu_buf = [0; 3];
            uguu.encode_utf8(&mut uguu_buf);
            output.write(&uguu_buf)?;
        }
    }

    Ok(())
}

fn uguuuu(input: &Path) -> bool {
    input.extension().unwrap().to_str().unwrap() == "uguu"
}

fn uuugu<R: Read, W: Write>(mut input: R, mut output: W) -> Result<(), UguuError> {
    let mut buffer = [0; 96];
    let mut read = 0;

    while let Ok(count) = input.read(&mut buffer) {
        if count == 0 {
            break;
        }
        let s = if count == 96 {
            std::str::from_utf8(&buffer)?
        } else {
            let buf = &buffer[0..count];
            std::str::from_utf8(&buf)?
        };

        let mut byte: u32 = 0;
        for c in s.chars() {
            let uugu = uugu(c)?;
            byte |= (uugu as u32) << (2 * (3 - read));
            read += 1;

            if read == 4 {
                output.write(&[byte as u8])?;
                read = 0;
                byte = 0;
            }

        }

    }

    assert_eq!(read, 0);

    Ok(())
}

fn uguu(bits: u8) -> char {
    match bits {
        0b00 => 'う',
        0b01 => 'ぐ',
        0b10 => 'ぅ',
        0b11 => '〜',
        _ => unreachable!(),
    }
}

fn uugu(c: char) -> Result<u8, UguuError> {
    match c {
        'う' => Ok(0b00),
        'ぐ' => Ok(0b01),
        'ぅ' => Ok(0b10),
        '〜' => Ok(0b11),
        _ => Err(UguuError::MyError(format!("Invalid sequence"))),
    }
}
