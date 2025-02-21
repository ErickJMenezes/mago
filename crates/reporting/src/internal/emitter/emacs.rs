use termcolor::WriteColor;

use mago_interner::ThreadedInterner;
use mago_source::HasSource;
use mago_source::SourceManager;

use crate::IssueCollection;
use crate::Level;
use crate::error::ReportingError;

pub fn emacs_format(
    writer: &mut dyn WriteColor,
    sources: &SourceManager,
    interner: &ThreadedInterner,
    issues: IssueCollection,
) -> Result<Option<Level>, ReportingError> {
    let highest_level = issues.get_highest_level();

    for issue in issues.iter() {
        let (file_path, line, column) = match issue.annotations.iter().find(|annotation| annotation.is_primary()) {
            Some(annotation) => {
                let source = sources.load(&annotation.span.source())?;

                let file_path = interner.lookup(&source.identifier.0).to_string();
                let line = source.line_number(annotation.span.start.offset) + 1;
                let column = source.column_number(annotation.span.start.offset) + 1;

                (file_path, line, column)
            }
            None => ("<unknown>".to_string(), 0, 0),
        };

        let severity = match issue.level {
            Level::Error => "error",
            Level::Warning | Level::Note | Level::Help => "warning",
        };

        let mut message = issue.message.clone();
        if let Some(link) = issue.link.as_deref() {
            message.push_str(" (see ");
            message.push_str(link);
            message.push(')');
        }

        let issue_type = issue.code.as_deref().unwrap_or("other");

        writeln!(writer, "{}:{}:{}:{} - {}: {}", file_path, line, column, severity, issue_type, message)?;
    }

    Ok(highest_level)
}
