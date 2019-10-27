use crate::cli::History as HistoryFile;
use crate::commands::PerItemCommand;
use crate::errors::ShellError;
use crate::parser::registry::{self};
use crate::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct History;

impl PerItemCommand for History {
    fn name(&self) -> &str {
        "history"
    }

    fn signature(&self) -> registry::Signature {
        Signature::build("history")
    }

    fn usage(&self) -> &str {
        "Display command history."
    }

    fn run(
        &self,
        call_info: &CallInfo,
        _registry: &CommandRegistry,
        _raw_args: &RawCommandArgs,
        _input: Tagged<Value>,
    ) -> Result<OutputStream, ShellError> {
        let tag = call_info.name_tag.clone();

        let stream = async_stream! {
            let history_path = HistoryFile::path();
            let file = File::open(history_path);
            if let Ok(file) = file {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        yield ReturnSuccess::value(Value::string(line).tagged(tag.clone()));
                    }
                }
            } else {
                yield Err(ShellError::labeled_error("Could not open history", "history file could not be opened", tag.clone()));
            }
        };
        Ok(stream.to_output_stream())
    }
}
