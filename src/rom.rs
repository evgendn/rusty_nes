use utils;

use std::io;
use std::convert::From;
use std::error;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const KB: u32 = 1024;

#[derive(Debug)]
pub enum ROMReadError {
    // IO error while reading the ROM image
    IoError(io::Error),
    // The ROM image has an invalid format
    FormatError,
    // The ROM format not supported
    NotSupported,
}

impl fmt::Display for ROMReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ROMReadError {
    fn description(&self) -> &str {
        match *self {
            ROMReadError::IoError(ref x) => x.description(),
            ROMReadError::FormatError => "Wrong ROM file format",
            ROMReadError::NotSupported => "The ROM format not supported"
        }
    }
}

impl From<io::Error> for ROMReadError {
    fn from(error: io::Error) -> ROMReadError {
        ROMReadError::IoError(error)
    }
}

// ROM Type need for define file format(iNES file)
enum ROMType {
    INES,
    UNIF,
    FDS,
}

pub enum MirroringType {
    Horizontal,
    Vertical,
}

// iNES header 
#[derive(Debug,Default)]
pub struct Header {
    // Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
    pub nes_constant: [u8; 4],
    
    // Size of PRG ROM in 16 KB units
    pub prg_rom_size: u8,
    
    // Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
    pub chr_rom_size: u8,
    
    // 76543210
    // ||||||||
    // |||||||+- Mirroring: 0: Horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
    // |||||||              1: vertical (Horizontal arrangement) (CIRAM A10 = PPU A10)
    // ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
    // |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
    // ||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
    // ++++----- Lower nybble of mapper number
    pub flags_6: u8,

    // 76543210
    // ||||||||
    // |||||||+- VS Unisystem
    // ||||||+-- PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
    // ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
    // ++++----- Upper nybble of mapper number
    pub flags_7: u8,

    // Size of PRG RAM in 8 KB units (Value 0 infers 8 KB for compatibility)
    pub prg_ram_size: u8,

    // 76543210
    // ||||||||
    // |||||||+- TV system (0: NTSC; 1: PAL)
    // +++++++-- Reserved, set to zero
    pub flags_9: u8,

    // 76543210
    //   ||  ||
    //   ||  ++- TV system (0: NTSC; 2: PAL; 1/3: dual compatible)
    //   |+----- PRG RAM ($6000-$7FFF) (0: present; 1: not present)
    //   +------ 0: Board has no bus conflicts; 1: Board has bus conflicts
    pub flags_10: u8,
}

impl Header {
    pub fn new(rom_header: &[u8]) -> Result<Header, ROMReadError> {
        let nes_const = &rom_header[0..4];
        match ROM::check_type(nes_const) {
            None => Err(ROMReadError::FormatError),
            Some(ROMType::UNIF) | 
            Some(ROMType::FDS) => Err(ROMReadError::NotSupported),            
            Some(ROMType::INES) => {
                let mut new_nes_const: [u8; 4] = [0; 4];
                new_nes_const.copy_from_slice(nes_const);

                let header = Header { 
                    nes_constant: new_nes_const,
                    prg_rom_size: rom_header[4],
                    chr_rom_size: rom_header[5],
                    flags_6: rom_header[6],
                    flags_7: rom_header[7],
                    prg_ram_size: rom_header[8],
                    flags_9: rom_header[9],
                    flags_10: rom_header[10],
                };
                Ok(header)
            },
        } 
    }
}

pub struct ROM {
    // 16 bytes
    pub header: Header,
    //pub trainer: [u8; 64],
    pub mirroring: MirroringType,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl ROM {
    pub fn load(path: &str) -> Result<ROM, ROMReadError> {
        let raw_data = match read_bin(Path::new(path)) {
            Ok(data) => data,
            Err(err) => panic!("Error: {}", err.description()),
        };
        let header = match Header::new(&raw_data[0..16]) {
            Ok(h) => h,
            Err(err) => panic!("Error: {}", err),
        };
        
        // TODO Trainer and other stuff

        let mirroring = match utils::get_bit(&header.flags_6, 0) {
            0u8 => MirroringType::Horizontal,
            1u8 => MirroringType::Vertical,
            _ => MirroringType::Vertical,
        }; 

        let data_start: usize = 16;

        let prg_rom_size_bytes = (header.prg_rom_size as u32) * 16 * KB;
        let chr_rom_size_bytes = (header.chr_rom_size as u32) * 8 * KB;
        let prg_ram_size_bytes = (header.prg_ram_size as u32) * 8 * KB;

        let prg_rom_end = data_start + prg_rom_size_bytes as usize;
        let chr_rom_end = prg_rom_end + chr_rom_size_bytes as usize;

        let rom = ROM {
            header: header,
            mirroring: mirroring,
            prg_rom: raw_data[data_start..prg_rom_end].to_vec(), 
            chr_rom: raw_data[prg_rom_end..chr_rom_end].to_vec(),
        };
        Ok(rom)
    }

    // Check ROM type for recognition file format
    fn check_type(nes_constant: &[u8]) -> Option<ROMType> {
        let ines_const = [0x4e, 0x45, 0x53, 0x1a];
        let fds_const = [0x46, 0x44, 0x53, 0x1a];
        let unif_const = [0x55, 0x4e, 0x49, 0x46];

        if nes_constant == fds_const {
            return Some(ROMType::FDS);
        } else if nes_constant == ines_const {
            return Some(ROMType::INES);
        } else if nes_constant == unif_const {
            return Some(ROMType::UNIF);
        }
        None
    }
}

impl fmt::Display for ROM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mirroring = match self.mirroring {
            MirroringType::Horizontal => "Horizontal",
            MirroringType::Vertical => "Vertical", 
        };
        let header = &self.header;
        write!(f, "Header: {:?}, Mirroring type: {}, \
               PRG ROM: {} KB, CHR ROM: {} KB",
               header.nes_constant, mirroring, 
               header.prg_rom_size, header.chr_rom_size)
    }
} 

pub fn read_bin<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ROMReadError> {
    let mut file = try!(File::open(path));
    let mut buffer: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut buffer));
    Ok(buffer)
}

