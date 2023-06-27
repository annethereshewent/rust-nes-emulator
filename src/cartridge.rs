const PRG_ROM_MULTIPLIER: usize = 16384;
const CHR_ROM_MULTIPLIER: usize = 8192;

const NES_ASCII: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

pub enum Mirroring {
  Horizontal,
  Vertical,
  FourScreen
}

pub struct Cartridge {
  pub prg_rom: Vec<u8>,
  pub chr_rom: Vec<u8>,
  pub mirroring: Mirroring
}

impl Cartridge {
  pub fn new(rom: Vec<u8>) -> Self {
    let prg_len: usize = rom[4] as usize * PRG_ROM_MULTIPLIER;
    let chr_len: usize = rom[5] as usize * CHR_ROM_MULTIPLIER;

    if rom[0..4] != NES_ASCII {
      panic!("file is not in ines format!");
    }

    let ines_ver = (rom[7] >> 2) & 0b11;
    if ines_ver != 0 {
        panic!("NES2.0 format is not supported");
    }

    let mapper_number = ((rom[7] >> 4) & 0b1111) << 4 | (rom[6] >> 4 & 0b1111);

    if mapper_number != 0 {
      panic!("mapper not supported");
    }
    let four_screen: bool = rom[6] & 0b1000 != 0;
    let vertical_mirroring = rom[6] & 0b1 != 0;

    let screen_mirroring = if four_screen {
      Mirroring::FourScreen
    } else if vertical_mirroring {
      Mirroring::Vertical
    } else {
      Mirroring::Horizontal
    };

    let skip_trainer = (rom[6] >> 3) & 0b1 == 1;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    let chr_rom_start = prg_rom_start + prg_len;

    Cartridge {
      prg_rom: rom[prg_rom_start .. (prg_rom_start + prg_len)].to_vec(),
      chr_rom: rom[chr_rom_start .. (chr_rom_start + chr_len)].to_vec(),
      mirroring: screen_mirroring
    }
  }
}