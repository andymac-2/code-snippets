struct Primes {
    index: u64,
    current: u64,
    stored: Vec<u64>
}

impl Primes {
    fn new () -> Primes {
        Primes {
            index: 0,
            current: 2,
            stored: vec![2],
        }
    }
    
    fn reset (&mut self) {
        self.index = 0;
    }
}

impl Iterator for Primes {
    type Item = u64;
    
    fn next (&mut self) -> Option<u64> {
        self.index += 1;
        
        if let prime = self.stored[self.index - 1] {
            return Some(prime);
        }
    
        let mut current = self.current;
        
        loop {
            current += 1;
            
            if self.stored.iter().all(|prime| current % prime != 0) {
                self.stored.push(current);
                self.current = current;
                break;
            }
        }
        
        Some(current)
    }
}