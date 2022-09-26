pub struct Aoc2018_05 {
    word: String
}
        
impl Aoc2018_05 {
    pub fn new() -> Self {
        Self { word: String::new() }
    }

    fn get_hash(s: &str) -> String {
        format!("{:X}", md5::compute(s.as_bytes()))
    }
}
        
impl crate::Solution for Aoc2018_05 {
    fn name(&self) -> (usize, usize) {
        (2018, 05)
    }
        
    fn parse(&mut self) {
        self.word = aoc::read_to_chars("input/2018/05.txt").into_iter().collect::<String>();
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut i: usize = 0;
        let mut password: String = String::new();

        while password.len() != 8 {
            let hash: Vec<char> = Aoc2018_05::get_hash(&(self.word.to_owned() + &i.to_string())).chars().collect();
            if hash[0..5].iter().map(|v| match v {
                '0' => 0,
                _ => 1,
            }).sum::<i32>() == 0 {
                password.push(hash[5]);
            }

            i += 1;
        }
        crate::output(password) 
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut i: usize = 0;
        let mut password: Vec<char> = vec!['-'; 8];

        loop {
            let hash: Vec<char> = Aoc2018_05::get_hash(&(self.word.to_owned() + &i.to_string())).chars().collect();
            if hash[0..5].iter().map(|v| match v {
                '0' => 0,
                _ => 1,
            }).sum::<i32>() == 0 {
                let index: usize =(hash[5] as u8 - '0' as u8) as usize; 

                if index >= 8 {
                    i += 1;
                    continue;
                }

                if password[index] != '-' {
                    i += 1;
                    continue;
                } 

                password[index] = hash[6];
                
                if password.iter().map(|c| match c {
                    '-' => 1,
                    _ => 0,
                }).sum::<i32>() == 0 {
                    break;
                };
            }
           i += 1;
        }

        crate::output(password.into_iter().collect::<String>())
    }
}