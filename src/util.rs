use core::fmt;

pub static PALINDROME_LENGTH: u32 = 5;
pub static GAP_LENGTH: u32 = 6;
pub static NUM_MISMATCH: u32 = 0;


#[derive(Debug)]
pub struct PalindromeData {
    start: u32,
    end: u32,
    length: u32,
    gap:u32,
    mismatches:u32,
    sequence: String,
}
impl PalindromeData {
    pub fn new(start:u32, end:u32, length:u32, gap:u32, mismatches:u32, sequence:String) -> Self {
        Self {
            start,
            end,
            length,
            gap,
            mismatches,
            sequence,
        }
    }
}
impl fmt::Display for PalindromeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: {}, End: {}, Length: {}, Gap Length: {}, Mismatches: {}, Sequence: {}", 
        self.start, self.end, self.length, self.gap, self.mismatches, self.sequence)
    }
}
pub fn seq_compliment(seq: &str) -> String {
    let mut output = String::new();
    for i in 0..seq.len() {
        output += get_complement(&seq[i..=i]);
    }
    return output;
}
pub fn get_complement(bp: &str) -> &str {
    let bpu = bp.to_uppercase();
    let letter = match &bpu[0..=0] {
        "A" => "T",
        "T" => "A",
        "C" => "G",
        "G" => "C",
        _ => panic!("Not a base pair"),
    };
    return letter;
}
pub fn is_complement(base1: &str, base2: &str) -> bool {
    return get_complement(base1) == base2;
}
