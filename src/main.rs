use std::path::PathBuf;

pub enum Action {
    Add { task: String },
    Done { position: usize },
    List,
}

pub structCommandLineArgs {
    pub action: Action,
    pub journal_file: Option<PathBuf>,
}