use dasp_rs::{io::Decoder, pitch};

use crate::frontend::LEXEME_SPACE_FZ;

pub struct SrcCodeIter {
    source_code: Vec<u32>,
    current_fz_index: usize,
}

impl Iterator for SrcCodeIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // we start looking for the next one
            self.current_fz_index += 1;

            if let Some(current_fz) = self.source_code.get(self.current_fz_index) {
                if *current_fz == LEXEME_SPACE_FZ {
                    self.current_fz_index += 1;
                    continue;
                }
                return Some(*current_fz);
            }
            return None;
        }
    }
}

// get's the frecuency in the period of the frame_length place in the dsp_rs 'pyinBuilder'methods,
// then collapses without any 0.0 (which are values that we won't use) or above them.
//
// Besides that we collapse any similar same frecuency as neighboars cause it will be tak as 'noise'
// haha, noise music LMAO
pub fn parse_mp3_to_frecuency_array(src_path: &str) -> SrcCodeIter {
    let audio = Decoder::new(src_path).mono().load().unwrap();

    let raw_pitches = pitch::pyin(&audio.samples, 261., 522.)
        .sample_rate(audio.sample_rate)
        .frame_length(1024)
        .hop_length(254)
        .compute()
        .unwrap()
        .to_owned();

    let mut pitches_without_zeros = raw_pitches.iter().filter(|pitch| **pitch > 0.0).peekable();

    let mut actual_pitch = **pitches_without_zeros.peek().unwrap();

    let pitches_without_noise = pitches_without_zeros
        .filter(|pitch| {
            let checking = (actual_pitch - **pitch) != 0.0;

            actual_pitch = (**pitch).clone();

            checking
        })
        .collect::<Vec<&f32>>()
        .iter()
        .map(|pitch| (**pitch).ceil() as u32)
        .collect::<Vec<u32>>();

    SrcCodeIter {
        source_code: pitches_without_noise,
        current_fz_index: 0,
    }
}
