#[derive(Debug, Clone, PartialEq)]
pub struct Mask {
    pub flags: Vec<bool>,
}

impl Mask {
    pub fn new(len: usize) -> Self {
        let mut flags: Vec<bool> = vec![];
        for _i in 0..len {
            flags.push(false);
        }
        Self {
            flags
        }
    }

    pub fn union(masks: &Vec<Mask>) -> Option<Mask> {
        if masks.len() > 0 && masks[0].flags.len() > 0 {
            let mut result = Mask::new(masks[0].flags.len());
            for m in masks {
                for (index, _flag) in m.flags.iter().enumerate() {
                    result.flags[index] |= m.flags[index];
                }
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn append(&self, mask: &Mask) -> Option<Mask> {
        // without checking source data: self.flags и mask.flags не должны содержать разрывов в последовательностях true элементов
        let mut flags = self.flags.clone();
        let mut appended = false;
        for (index, _flag) in self.flags.iter().enumerate() {
            if index > 0 && flags[index-1] && !flags[index] {
                flags[index] |= mask.flags[index];
                appended = true;
            }
        }
        if appended {
            Some(Mask { flags })
        } else {
            None
        }
    }

    pub fn split(&self) -> Vec<Mask> {
        let mut result = vec![];

        let mut j = 0;
        for (i, f) in self.flags.iter().enumerate() {
            if (i == 0 && *f) || (i > 0 && *f && !self.flags[i-1]) {
                // new
                if result.len() > 0 {
                    j += 1;
                }
                result.push(Mask::new(self.flags.len()));
                result[j].flags[i] = true;
            } else if (i > 0) && self.flags[i-1] {
                // append
                result[j].flags[i] = true;
            }
        }

        result
    }
}