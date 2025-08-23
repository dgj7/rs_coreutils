
///
/// Things all commands do.
///
pub(crate) struct CommandDefinition {
    pub(crate) ind: String,
    pub(crate) desc: String,
}

///
/// Storage for a boolean input.
///
pub(crate) struct BooleanCommand {
    pub(crate) def: CommandDefinition,
    pub(crate) value: bool,
}

///
/// Storage for a string input.
///
pub(crate) struct StringCommand {
    pub(crate) def: CommandDefinition,
    pub(crate) value: Option<String>,
}

///
/// Storage for year input.
///
pub(crate) struct YearCommand {
    pub(crate) def: CommandDefinition,
    pub(crate) value: Option<u16>,
}

///
/// Storage for count input.
///
pub(crate) struct CountCommand {
    pub(crate) def: CommandDefinition,
    pub(crate) value: Option<usize>,
}


