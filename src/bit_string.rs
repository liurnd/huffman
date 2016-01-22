#[derive(Clone)]
pub struct BitString{
    seq: Vec<u8>,
    bit_cnt: usize,
    next_mask: u8
}

impl BitString{
    pub fn new() -> Self{
        BitString{seq:Vec::<u8>::new(), bit_cnt:0, next_mask:1}
    }
    
    pub fn push_bit(&mut self, bit: bool){
        if (self.bit_cnt % 8) == 0{
            self.seq.push(0);
            self.next_mask = 1;
        }
        let last_idx = self.seq.len()-1;
        if bit {self.seq[last_idx] += self.next_mask;}
        
        self.next_mask <<= 1;
        self.bit_cnt += 1;
    }

    pub fn pop_bit(&mut self) -> Option<bool>{
        if self.bit_cnt == 0
        {
            return None;
        }
        
        let next_mask = 
            if self.bit_cnt% 8 == 0{
                self.seq.pop();
                0x80
            }
        else
        {
            self.next_mask >> 1
        };

        let last_idx = self.seq.len()-1;
        let res = (self.seq[last_idx] & next_mask)!=0;
        self.seq[last_idx] &= !next_mask;

        self.bit_cnt -= 1;
        if self.bit_cnt == 0 { self.seq.pop(); }
        self.next_mask = next_mask;
        Some(res)
    }

    pub fn len(&self) -> usize{
        self.bit_cnt
    }

    pub fn get_bit(&self, idx : usize) -> bool {
        let p_idx = idx / 8;
        let o_idx = idx % 8;
        let mask = 1 << o_idx;
        (self.seq[p_idx] & mask) != 0
    }
}

use std::ops::Add;
impl Add for BitString{
    type Output = BitString;
    fn add(self, rhs:BitString) -> Self{
        let mut res = self.clone();
        for i in 0..rhs.len(){
            res.push_bit(rhs.get_bit(i));
        }
        res
    }
}

use std::fmt;

impl fmt::Debug for BitString{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0 .. self.len(){
            try!(write!(f, "{}", if self.get_bit(i) {1} else {0}));
        }
        write!(f, ",{} Bit", self.len())
    }
}

#[cfg(test)]
mod test{
    use super::BitString;
    #[test]
    fn push_pop(){
        let ts = vec![true, true, true, false, false, true, false, false ,false, true, false, true, true];
        let mut s = BitString::new();
        for i in &ts{
            s.push_bit(*i);
        }

        for i in 0 .. ts.len(){
            let v = ts.len()-i-1;
            assert_eq!(s.pop_bit().unwrap(), ts[v]);
        }
    }

    
}
