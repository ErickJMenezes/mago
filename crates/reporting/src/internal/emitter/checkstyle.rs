use std::collections::HashMap;
use termcolor::WriteColor;

use mago_interner::ThreadedInterner;
use mago_source::HasSource;
use mago_source::SourceManager;

use crate::IssueCollection;
use crate::Level;
use crate::error::ReportingError;
use crate::internal::emitter::utils::long_message;
use crate::internal::emitter::utils::xml_encode;

pub fn checkstyle_format(
    writer: &mut dyn WriteColor,
    sources: &SourceManager,
    interner: &ThreadedInterner,
    issues: IssueCollection,
) -> Result<Option<Level>, ReportingError> {
    let highest_level = issues.get_highest_level();

    // Group issues by file
    let mut issues_by_file: HashMap<String, Vec<String>> = HashMap::new();

    for issue in issues.iter() {
        let (filename, line, column) = match issue.annotations.iter().find(|annotation| annotation.is_primary()) {
            Some(annotation) => {
                let source = sources.load(&annotation.span.source())?;

                let filename = interner.lookup(&source.identifier.0).to_string();
                let line = source.line_number(annotation.span.start.offset) + 1;
                let column = source.column_number(annotation.span.start.offset) + 1;

                (filename, line, column)
            }
            None => ("<unknown>".to_string(), 0, 0),
        };

        let severity = match issue.level {
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Help | Level::Note => "info",
        };

        let message = xml_encode(long_message(issue));
        let error_tag = format!(
            "    <error line=\"{}\" column=\"{}\" severity=\"{}\" message=\"{}\" />",
            line, column, severity, message
        );

        issues_by_file.entry(filename).or_default().push(error_tag);
    }

    // Begin Checkstyle XML
    writeln!(writer, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(writer, "<checkstyle>")?;

    // Write grouped issues
    for (filename, errors) in issues_by_file {
        writeln!(writer, "  <file name=\"{}\">", xml_encode(&filename))?;
        for error in errors {
            writeln!(writer, "{}", error)?;
        }

        writeln!(writer, "  </file>")?;
    }

    // Close Checkstyle XML
    writeln!(writer, "</checkstyle>")?;

    Ok(highest_level)
}
