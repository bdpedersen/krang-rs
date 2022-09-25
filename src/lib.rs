#![no_std]

extern crate core;
extern crate unroll; 

mod sponge;

use crate::sponge::sponge::Sponge;
use core::cmp::min;

pub struct Krang {
    sponge: Sponge
}

impl Krang {
    pub fn new() -> Krang {
        Krang { sponge: Sponge::new() }
    }

    pub fn seed(&mut self, val: &[u8]) {
        let mut idx = 0;
        while idx < val.len() {
            let stride = min(8,val.len()-idx);
            self.sponge.mix(&val[idx..idx+stride]);
            self.sponge.keccak_f();
            idx += stride;
        }
    }

    pub fn fetch(&mut self, val: &mut[u8]) {
        let mut idx = 0;
        while idx < val.len() {
            let stride = min(8,val.len()-idx);
            self.sponge.extract(&mut val[idx..idx+stride]);
            self.sponge.keccak_f();
            idx += stride;
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    fn compare_vectors(v1: &[u8], v2: &[u8]) -> bool {
        v1.iter().zip(v2.iter()).map(|a| a.0 == a.1).fold(true,|a,b| a & b)
    }

    #[test]
    fn test_random() {
        let output: [u8; 160] = [87,53,195,188,152,101,42,169,44,250,79,246,144,9,110,235,
        144,96,151,105,120,252,76,114,71,148,42,197,152,78,8,52,
        63,14,18,79,189,134,213,7,101,95,162,118,162,202,95,88,
        108,95,130,172,208,244,215,230,243,152,46,34,96,218,139,240,
        221,134,179,135,112,231,249,53,215,10,55,54,176,191,214,122,
        55,64,198,159,115,143,190,238,128,188,92,16,181,65,73,246,
        18,133,161,237,254,77,85,97,244,236,49,47,0,87,2,105,
        86,168,173,161,6,116,125,1,211,28,158,79,167,135,36,104,
        254,28,216,92,69,183,145,187,248,136,255,205,114,43,199,72,
        50,100,123,47,149,33,64,171,118,144,81,154,156,159,154,137,];

        let mut test: [u8; 160] = [0; 160];

        let mut rng = Krang::new();
        
        rng.seed("cafebabe".as_bytes());
        rng.fetch(&mut test);

        
        assert!(compare_vectors(&output, &test));
    }

}
