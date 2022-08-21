//use unroll::unroll_for_loops;

pub mod sponge {
    
const RC: [u64;24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808A,
    0x8000000080008000,
    0x000000000000808B,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008A,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000A,
    0x000000008000808B,
    0x800000000000008B,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800A,
    0x800000008000000A,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

const RIX: [[i32; 5] ; 5] = [
	[0,36,3,41,18],
	[1,44,10,45,2],
	[62,6,43,15,61],
	[28,55,25,21,56],
	[27,20,39,8,14],
];

fn rol8(x: u8, s: i32) -> u8 {
    let shft: i32 = s & 7;
    if shft > 0 {
        (x << shft) | (x >> (8-shft))  
    } else {
        x
    }
} 


pub struct Sponge  {
    a: [[u8;5];5]
}


impl Sponge {
    pub fn new() -> Sponge {
        Sponge { a: [[0,0,0,0,0,],[0,0,0,0,0,],[0,0,0,0,0,],[0,0,0,0,0,],[0,0,0,0,0,],] }
    }

    pub fn mix(&mut self, vals: &[u8]) {
        let bound = vals.len();
        debug_assert!(bound <= 25);
        let mut idx = 0;
        for y in 0..5 {
            for x in 0..5 {
                self.a[y][x] ^= vals[idx];
                idx += 1;
                if idx == bound {
                    return;
                }
            }
        }
    }

    pub fn extract(&self, output: &mut [u8]) {
        let entries = output.len();
        debug_assert!(entries <= 25);
        let mut idx = 0;
        for y in 0..5 {
            for x in 0..5 {
                output[idx] = self.a[y][x];
                idx += 1;
                if idx == entries {
                    return;
                }
            }
        }
    }

    #[unroll::unroll_for_loops]
    fn keccak_p(&mut self, rcv: u8){
        let mut c: [u8; 5] = [0; 5];
        let mut d: [u8; 5] = [0; 5];
        let mut b: [[u8; 5]; 5] = [[0; 5]; 5];

        for x in 0..5 {
            c[x] = self.a[0][x]^self.a[1][x]^self.a[2][x]^self.a[3][x]^self.a[4][x];
        }

        for x in 0..5 {
            d[x] = c[(x+4)%5] ^ rol8(c[(x+1)%5],1);
        }

        for y in 0..5 {
            for x in 0..5 {
                self.a[y][x] ^= d[x];
            }
        }

        for y in 0..5 {
            for x in 0..5 {
                b[y][(2*x+3*y)%5] = rol8(self.a[y][x], RIX[x][y]);
            }
        }

        for y in 0..5 {
            for x in 0..5 {
                self.a[y][x]=b[x][y] ^ ((!b[(x+1)%5][y]) & b[(x+2)%5][y]);

            }
        }
        self.a[0][0] ^= rcv;
    }
    
    pub fn keccak_f(&mut self) {
        for r in RC[0..18].iter() {
            self.keccak_p(*r as u8);
        }
    }

}





}
