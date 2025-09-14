pub mod integer {
    pub mod constraint;
    pub mod m_integer;
}

pub mod string {
    pub mod m_string;
}

pub mod bool {
    pub mod m_bool;
}

pub use bool::m_bool::*;
pub use integer::constraint::*;
pub use integer::m_integer::*;
pub use string::m_string::*;

#[derive(Debug)]
pub enum MarbleType {
    MInteger,
    MString,
    Mbool,
}
