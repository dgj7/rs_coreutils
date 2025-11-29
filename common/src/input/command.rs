
///
/// Things all commands do.
///
pub struct CommandDefinition {
    pub ind: String,
    pub desc: String,
}

///
/// Storage for a boolean input.
///
pub struct BooleanCommand {
    pub def: CommandDefinition,
    pub value: bool,
}

///
/// Storage for a string input.
///
pub struct StringCommand {
    pub def: CommandDefinition,
    pub value: Option<String>,
}

///
/// Storage for year input.
///
pub struct YearCommand {
    pub def: CommandDefinition,
    pub value: Option<u16>,
}

///
/// Storage for count input.
///
pub struct CountCommand {
    pub def: CommandDefinition,
    pub value: Option<usize>,
}


