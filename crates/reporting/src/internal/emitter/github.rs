use termcolor::WriteColor;

use mago_interner::ThreadedInterner;
use mago_source::HasSource;
use mago_source::SourceManager;

use crate::IssueCollection;
use crate::Level;
use crate::error::ReportingError;
use crate::internal::emitter::utils::long_message;

pub fn github_format(
    writer: &mut dyn WriteColor,
    sources: &SourceManager,
    interner: &ThreadedInterner,
    issues: IssueCollection,
) -> Result<Option<Level>, ReportingError> {
    let highest_level = issues.get_highest_level();

    for issue in issues.iter() {
        let level = match &issue.level {
            Level::Note => "notice",
            Level::Help => "notice",
            Level::Warning => "warning",
            Level::Error => "error",
        };

        let properties = match issue.annotations.iter().find(|annotation| annotation.is_primary()) {
            Some(annotation) => {
                let source = sources.load(&annotation.span.source())?;
                let name = interner.lookup(&source.identifier.0);
                let start_line = source.line_number(annotation.span.start.offset) + 1;
                let end_line = source.line_number(annotation.span.end.offset) + 1;

                if let Some(code) = issue.code.as_ref() {
                    format!("file={name},line={start_line},endLine={end_line},title={code}")
                } else {
                    format!("file={name},line={start_line},endLine={end_line}")
                }
            }
            None => {
                if let Some(code) = issue.code.as_ref() {
                    format!("title={code}")
                } else {
                    String::new()
                }
            }
        };

        // we must use `%0A` instead of `\n`.
        //
        // see: https://github.com/actions/toolkit/issues/193
        let message = long_message(issue).replace("\n", "%0A");

        writeln!(writer, "::{} {}::{}", level, properties, message)?;
    }

    Ok(highest_level)
}
