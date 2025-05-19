use brk_core::OutputType;

#[derive(Default, Clone)]
pub struct OutputsByType<T> {
    pub p2pk65: T,
    pub p2pk33: T,
    pub p2pkh: T,
    pub p2ms: T,
    pub p2sh: T,
    pub op_return: T,
    pub p2wpkh: T,
    pub p2wsh: T,
    pub p2tr: T,
    pub p2a: T,
    pub empty: T,
    pub unknown: T,
}

impl<T> OutputsByType<T> {
    pub fn get(&self, output_type: OutputType) -> &T {
        match output_type {
            OutputType::P2PK65 => &self.p2pk65,
            OutputType::P2PK33 => &self.p2pk33,
            OutputType::P2PKH => &self.p2pkh,
            OutputType::P2MS => &self.p2ms,
            OutputType::P2SH => &self.p2sh,
            OutputType::OpReturn => &self.op_return,
            OutputType::P2WPKH => &self.p2wpkh,
            OutputType::P2WSH => &self.p2wsh,
            OutputType::P2TR => &self.p2tr,
            OutputType::P2A => &self.p2a,
            OutputType::Empty => &self.empty,
            OutputType::Unknown => &self.unknown,
        }
    }

    pub fn get_mut(&mut self, output_type: OutputType) -> &mut T {
        match output_type {
            OutputType::P2PK65 => &mut self.p2pk65,
            OutputType::P2PK33 => &mut self.p2pk33,
            OutputType::P2PKH => &mut self.p2pkh,
            OutputType::P2MS => &mut self.p2ms,
            OutputType::P2SH => &mut self.p2sh,
            OutputType::OpReturn => &mut self.op_return,
            OutputType::P2WPKH => &mut self.p2wpkh,
            OutputType::P2WSH => &mut self.p2wsh,
            OutputType::P2TR => &mut self.p2tr,
            OutputType::P2A => &mut self.p2a,
            OutputType::Empty => &mut self.empty,
            OutputType::Unknown => &mut self.unknown,
        }
    }

    pub fn to_spendable_vec(&self) -> Vec<&T> {
        OutputType::as_vec()
            .into_iter()
            .map(|t| (self.get(t)))
            .collect::<Vec<_>>()
    }

    pub fn as_vec(&mut self) -> Vec<&T> {
        vec![
            &self.p2pk65,
            &self.p2pk33,
            &self.p2pkh,
            &self.p2ms,
            &self.p2sh,
            &self.op_return,
            &self.p2wpkh,
            &self.p2wsh,
            &self.p2tr,
            &self.p2a,
            &self.empty,
            &self.unknown,
        ]
    }

    pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
        vec![
            &mut self.p2pk65,
            &mut self.p2pk33,
            &mut self.p2pkh,
            &mut self.p2ms,
            &mut self.p2sh,
            &mut self.op_return,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.p2a,
            &mut self.empty,
            &mut self.unknown,
        ]
    }
}
