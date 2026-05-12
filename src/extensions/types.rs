pub trait GetInner {
    type Inner;
}

impl<T> GetInner for Option<T> {
    type Inner = T;
}

pub type HandlerFuncInner = <crate::base::HandlerFunc as GetInner>::Inner;

pub type CondHandlerInner = <crate::base::CondHandler as GetInner>::Inner;

pub type NumMathFuncInner = <crate::base::NumMathFunc as GetInner>::Inner;
pub type StrMathFuncInner = <crate::base::StrMathFunc as GetInner>::Inner;
