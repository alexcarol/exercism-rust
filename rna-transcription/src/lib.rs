pub struct DeoxyribonucleicAcid {
    s: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { s: s.to_string() }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, bool> {
        let res = (&self.s)
            .chars()
            .filter_map(|c| match c {
                'G' => Some('C'),
                'C' => Some('G'),
                'T' => Some('A'),
                'A' => Some('U'),
                _ => None,
            })
            .collect::<String>();

        if res.len() != self.s.len() {
            return Result::Err(false);
        }

        Result::Ok(RibonucleicAcid::new(res.as_ref()))
    }
}

#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    s: String,
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> RibonucleicAcid {
        RibonucleicAcid { s: s.to_string() }
    }
}
