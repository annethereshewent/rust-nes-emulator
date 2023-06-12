const PRG_ROM_MULTIPLIER: usize = 16384;
const CHR_ROM_MULTIPLIER: usize = 8192;

const NES_ASCII: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

pub struct Cartridge {
  pub prg_rom: Vec<u8>,
  pub chr_rom: Vec<u8>
}

impl Cartridge {
  pub fn new(rom: Vec<u8>) -> Self {
    let prg_len: usize = rom[4] as usize * PRG_ROM_MULTIPLIER;
    let chr_len: usize = rom[5] as usize * CHR_ROM_MULTIPLIER;

    if rom[0..4] != NES_ASCII {
      panic!("file is not in ines format!");
    }

    let skip_trainer = (rom[6] >> 3) & 0b1 == 1;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    let chr_rom_start = prg_rom_start + prg_len;

    Cartridge {
      prg_rom: rom[prg_rom_start .. (prg_rom_start + prg_len)].to_vec(),
      chr_rom: rom[chr_rom_start .. (chr_rom_start + chr_len)].to_vec()
    }
  }
}