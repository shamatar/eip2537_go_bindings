pub const G1ADD_OPERATION_RAW_VALUE: u8 = OperationType::G1ADD as u8;
pub const G1MUL_OPERATION_RAW_VALUE: u8 = OperationType::G1MUL as u8;
pub const G1MULTIEXP_OPERATION_RAW_VALUE: u8 = OperationType::G1MULTIEXP as u8;

pub const G2ADD_OPERATION_RAW_VALUE: u8 = OperationType::G2ADD as u8;
pub const G2MUL_OPERATION_RAW_VALUE: u8 = OperationType::G2MUL as u8;
pub const G2MULTIEXP_OPERATION_RAW_VALUE: u8 = OperationType::G2MULTIEXP as u8;

pub const PAIR_OPERATION_RAW_VALUE: u8 = OperationType::PAIR as u8;
pub const MAP_FP_TO_G1_OPERATION_RAW_VALUE: u8 = OperationType::MAPFPTOG1 as u8;
pub const MAP_FP2_TO_G2_OPERATION_RAW_VALUE: u8 = OperationType::MAPFP2TOG2 as u8;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperationType {
    G1ADD = 1,
    G1MUL = 2,
    G1MULTIEXP = 3,
    G2ADD = 4,
    G2MUL = 5,
    G2MULTIEXP = 6,
    PAIR = 7,
    MAPFPTOG1 = 8,
    MAPFP2TOG2 = 9,
}

impl OperationType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            G1ADD_OPERATION_RAW_VALUE => {
                Some(OperationType::G1ADD)
            },
            G1MUL_OPERATION_RAW_VALUE => {
                Some(OperationType::G1MUL)
            },
            G1MULTIEXP_OPERATION_RAW_VALUE => {
                Some(OperationType::G1MULTIEXP)
            },
            G2ADD_OPERATION_RAW_VALUE => {
                Some(OperationType::G2ADD)
            },
            G2MUL_OPERATION_RAW_VALUE => {
                Some(OperationType::G2MUL)
            },
            G2MULTIEXP_OPERATION_RAW_VALUE => {
                Some(OperationType::G2MULTIEXP)
            },
            PAIR_OPERATION_RAW_VALUE => {
                Some(OperationType::PAIR)
            },
            MAP_FP_TO_G1_OPERATION_RAW_VALUE => {
                Some(OperationType::MAPFPTOG1)
            },
            MAP_FP2_TO_G2_OPERATION_RAW_VALUE => {
                Some(OperationType::MAPFP2TOG2)
            },
            _ => {
                None
            }
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}