// use brk_core::OutputType;

#[derive(Default, Clone)]
pub struct OutputsByUnspendableType<T> {
    pub op_return: T,
    pub empty: T,
    // pub unknown: T,
}

impl<T> OutputsByUnspendableType<T> {
    // pub fn get(&self, output_type: OutputType) -> &T {
    //     match output_type {
    //         OutputType::OpReturn => &self.op_return,
    //         OutputType::Empty => &self.empty,
    //         OutputType::Unknown => &self.unknown,
    //         _ => unreachable!(),
    //     }
    // }

    // pub fn get_mut(&mut self, output_type: OutputType) -> &mut T {
    //     match output_type {
    //         OutputType::OpReturn => &mut self.op_return,
    //         OutputType::Empty => &mut self.empty,
    //         OutputType::Unknown => &mut self.unknown,
    //         _ => unreachable!(),
    //     }
    // }

    // pub fn to_unspendable_vec(&self) -> Vec<&T> {
    //     OutputType::as_vec()
    //         .into_iter()
    //         .map(|t| (self.get(t)))
    //         .collect::<Vec<_>>()
    // }

    pub fn as_vec(&self) -> [&T; 2] {
        [
            &self.op_return,
            &self.empty,
            // &self.unknown
        ]
    }

    // pub fn as_mut_vec(&mut self) -> [&mut T; 3] {
    //     [&mut self.op_return, &mut self.empty, &mut self.unknown]
    // }
}
