
mod state;
pub mod circuit;
pub mod computer;
pub mod gate;
mod operations;
mod executable;
pub mod measure;

const N:usize = 17;
const POWER_OF_TWOS:[usize;N] = [1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536];

pub fn power_of_two(pow:u8) -> usize {
    let pow_usize = pow as usize;
    return if pow_usize < N {
        POWER_OF_TWOS[pow_usize]
    } else {
        (2 as usize).pow(pow as u32)
    }
}


pub trait QDimension {
    fn nb_qbits(&self) -> u8;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
