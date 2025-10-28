pub type SymbolTable = Vec<Variable>;

pub struct Identifier {
    name: String,
    category: String,
    index: usize,
    usage: String,
}

#[derive(Debug)]
pub struct Variable {
    pub variable_name: String,
    pub variable_type: VariableType,
    pub variable_kind: VariableKind,
    pub variable_scope: VariableScope,
    pub variable_index: usize,
}

#[derive(Debug)]
pub enum VariableType {
    Integer,
    Boolean,
    Char,
    ClassName(String),
}

#[derive(Debug)]
pub enum VariableKind {
    Static,
    Field,
    Variable,
    Argument,
}

#[derive(Debug)]
pub enum VariableScope {
    ClassLevel,
    SubroutineLevel,
}
