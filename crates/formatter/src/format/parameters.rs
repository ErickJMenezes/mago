use mago_ast::*;
use mago_span::*;

use crate::Formatter;
use crate::comment::CommentFlags;
use crate::document::*;
use crate::format::Format;

pub(super) fn should_hug_the_only_parameter<'a>(
    f: &mut Formatter<'a>,
    parameter_list: &'a FunctionLikeParameterList,
) -> bool {
    if parameter_list.parameters.len() != 1 {
        return false;
    }

    let Some(parameter) = parameter_list.parameters.first() else {
        return false;
    };

    // Avoid hugging the parameter if it has a comment anywhere around it
    if f.has_comment(parameter.span(), CommentFlags::all()) {
        return false;
    }

    // Don't hug the parameter if it has an attribute, or if it has a
    // property hook list.
    //
    // TODO: maybe hug the parameter if it has a single attribute and no hooks?
    if !parameter.attribute_lists.is_empty() || parameter.hooks.is_some() {
        return false;
    }

    if !parameter.modifiers.is_empty() && f.settings.break_promoted_properties_list {
        return false;
    }

    true
}

pub(super) fn print_function_like_parameters<'a>(
    f: &mut Formatter<'a>,
    parameter_list: &'a FunctionLikeParameterList,
) -> Document<'a> {
    let should_hug_the_parameters = should_hug_the_only_parameter(f, parameter_list);
    let should_break = !should_hug_the_parameters
        && f.settings.break_promoted_properties_list
        && parameter_list.parameters.iter().any(|p| p.is_promoted_property());

    let mut parts = vec![Document::String("(")];
    let mut printed = vec![];
    let len = parameter_list.parameters.len();
    for (i, parameter) in parameter_list.parameters.iter().enumerate() {
        printed.push(parameter.format(f));
        if i == len - 1 {
            break;
        }

        printed.push(Document::String(","));
        if should_hug_the_parameters {
            printed.push(Document::space());
        } else {
            printed.push(Document::Line(Line::default()));

            if f.is_next_line_empty(parameter.span()) {
                printed.push(Document::BreakParent);
                printed.push(Document::Line(Line::hardline()));
            }
        }
    }

    if should_hug_the_parameters {
        let mut contents = vec![Document::String("(")];
        contents.extend(printed);
        contents.push(Document::String(")"));

        return Document::Array(contents);
    }

    if !parameter_list.parameters.is_empty() {
        let mut contents = vec![Document::Line(Line::softline())];
        contents.extend(printed);
        parts.push(Document::Indent(contents));

        if f.settings.trailing_comma {
            parts.push(Document::IfBreak(IfBreak::then(Document::String(","))));
        }
    }

    if let Some(comments) =
        f.print_dangling_comments(parameter_list.left_parenthesis.join(parameter_list.right_parenthesis), true)
    {
        parts.push(comments);
    } else {
        parts.push(Document::Line(Line::softline()));
    }

    parts.push(Document::String(")"));

    if f.argument_state.expand_first_argument {
        Document::Array(parts)
    } else {
        Document::Group(Group::new(parts).with_break(should_break))
    }
}
