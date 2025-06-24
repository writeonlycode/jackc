pub struct SymbolTable {
    variable_name: string,
    variable_type: VariableType,
    variable_kind: VariableKind,
    variable_index: usize
}

pub enum VariableType {
    Integer,
    Boolean,
    Char,
    ClassName
}

pub enum VariableKind {
    Static,
    Field
    Var
    Arg
}
