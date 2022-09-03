/*
 * Simple Seeded Xorshift128 implementation
 */

pub struct XorShift128
{
    state: [u64; 2],
}


impl XorShift128
{
    pub fn from_seed(a: u64, b: u64) -> XorShift128
    {
        XorShift128 { state: [a, b] } 
    }

    fn gen(&mut self)
    {
        let mut t = self.state[0];
        let s = self.state[1];

        self.state[0] = s;

        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);

        self.state[1] = t;
    }

    pub fn pick(&mut self) -> u128
    {
        self.gen();

        let mut ret: u128 = (self.state[0] as u128) << 64;
        ret += self.state[1] as u128;

        ret
    }
}


