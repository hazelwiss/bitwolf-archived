#[cfg(test)]
mod test {
    use crate::{
        instrutions::{
            decode::{ALUArg, RPTblEntry, RSTVec, ALU, CC, DEC, E8, INC, LD, ROT},
            Unprefixed,
        },
        registers::{R16, R8},
    };

    impl PartialEq for Unprefixed {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::RST(l0), Self::RST(r0)) => l0 == r0,
                (Self::PUSH(l0), Self::PUSH(r0)) => l0 == r0,
                (Self::POP(l0), Self::POP(r0)) => l0 == r0,
                (Self::CALLCC(l0), Self::CALLCC(r0)) => l0 == r0,
                (Self::JPCC(l0), Self::JPCC(r0)) => l0 == r0,
                (Self::ADDHL(l0), Self::ADDHL(r0)) => l0 == r0,
                (Self::RETCC(l0), Self::RETCC(r0)) => l0 == r0,
                (Self::JRCC(l0), Self::JRCC(r0)) => l0 == r0,
                (Self::LD(l0), Self::LD(r0)) => l0 == r0,
                (Self::INC(l0), Self::INC(r0)) => l0 == r0,
                (Self::DEC(l0), Self::DEC(r0)) => l0 == r0,
                (Self::ALU(l0), Self::ALU(r0)) => l0 == r0,
                (Self::ROT(l0), Self::ROT(r0)) => l0 == r0,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }

    impl PartialEq for RSTVec {
        fn eq(&self, other: &Self) -> bool {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }

    impl PartialEq for R16 {
        fn eq(&self, other: &Self) -> bool {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }

    impl PartialEq for CC {
        fn eq(&self, other: &Self) -> bool {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }

    impl PartialEq for RPTblEntry {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::R16(l0), Self::R16(r0)) => l0 == r0,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }

    impl PartialEq for ALUArg {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::E8(l0), Self::E8(r0)) => l0 == r0,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }

    impl PartialEq for INC {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::E8(l0), Self::E8(r0)) => l0 == r0,
                (Self::RPTblRet(l0), Self::RPTblRet(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl PartialEq for DEC {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::E8(l0), Self::E8(r0)) => l0 == r0,
                (Self::RPTblRet(l0), Self::RPTblRet(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl PartialEq for ALU {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::ADD(l0), Self::ADD(r0)) => l0 == r0,
                (Self::ADC(l0), Self::ADC(r0)) => l0 == r0,
                (Self::SUB(l0), Self::SUB(r0)) => l0 == r0,
                (Self::SBC(l0), Self::SBC(r0)) => l0 == r0,
                (Self::AND(l0), Self::AND(r0)) => l0 == r0,
                (Self::XOR(l0), Self::XOR(r0)) => l0 == r0,
                (Self::OR(l0), Self::OR(r0)) => l0 == r0,
                (Self::CP(l0), Self::CP(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl PartialEq for ROT {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::RLC(l0), Self::RLC(r0)) => l0 == r0,
                (Self::RRC(l0), Self::RRC(r0)) => l0 == r0,
                (Self::RL(l0), Self::RL(r0)) => l0 == r0,
                (Self::RR(l0), Self::RR(r0)) => l0 == r0,
                (Self::SLA(l0), Self::SLA(r0)) => l0 == r0,
                (Self::SRA(l0), Self::SRA(r0)) => l0 == r0,
                (Self::SWAP(l0), Self::SWAP(r0)) => l0 == r0,
                (Self::SRL(l0), Self::SRL(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl PartialEq for LD {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::E8_R8(l0, l1), Self::E8_R8(r0, r1)) => l0 == r0 && l1 == r1,
                (Self::E8_N(l0), Self::E8_N(r0)) => l0 == r0,
                (Self::R16_NN(l0), Self::R16_NN(r0)) => l0 == r0,
                (Self::PR16_R8(l0, l1), Self::PR16_R8(r0, r1)) => l0 == r0 && l1 == r1,
                (Self::R8_PR16(l0, l1), Self::R8_PR16(r0, r1)) => l0 == r0 && l1 == r1,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }

    impl PartialEq for E8 {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::R8(l0), Self::R8(r0)) => l0 == r0,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }

    impl PartialEq for R8 {
        fn eq(&self, other: &Self) -> bool {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }

    #[test]
    fn test_decoding() {
        for i in 0..=255 as u8 {
            let unprefixed = Unprefixed::from_byte(i);
            println!("{i:08b} ({i:02X}) -> {unprefixed:?}");
            //let cmp = match i {
            //    0 => ,
            //    1 => ,
            //    2 => ,
            //    3
            //};
            //if unprefixed != cmp {
            //    panic!("invalid opcode at {i:02X} -> {unprefixed:?}. Expected: {cmp:?}")
            //}
        }
        panic!("YEAH!")
    }
}
