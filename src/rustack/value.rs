use crate::rustack::vm::Vm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(i64),
    Op(String),
    Sym(String),
    Block(Vec<Value>),
    Native(NativeOp),
}

#[derive(Clone)]
pub struct NativeOp(pub fn(&mut Vm));

impl PartialEq for NativeOp {
    fn eq(&self, other: &NativeOp) -> bool {
        self.0 as *const fn() == other.0 as *const fn()
    }
}

impl Eq for NativeOp {}

impl std::fmt::Debug for NativeOp {
    fn fmt (
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "<nativeOp>")
    }
}

impl Value {
    pub fn as_num(&self) -> i64 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is not a number"),
        }
    }

    pub fn as_sym(&self) -> String {
        if let Self::Sym(sym) = self {
            sym.clone()
        } else {
            panic!("Value is not a symbol");
        }
    }

    pub fn to_block(self) -> Vec<Value> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Num(i) => i.to_string(),
            Self::Op(ref s) | Self::Sym(ref s) => s.clone(),
            Self::Block(_) => "<Block>".to_string(),
            Self::Native(_) => "<Native>".to_string(),
        }
    }
}
