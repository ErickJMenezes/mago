# Using the Formatter

The Mago formatter is designed to enforce consistent coding styles across your PHP project. This guide provides an overview of how to use the formatter effectively in your development workflow.

## Overview

The formatter standardizes your code's style according to rules defined in the `[formatter]` section of your `mago.toml` configuration file. It supports features like customizable indentation, line width, end-of-line characters, and more.

## Basic Usage

### Formatting Your Project

To format all PHP files in your project, run:

```bash
mago format
```

This command applies formatting rules to your codebase, ensuring consistency across all files.

### Previewing Changes

To preview changes without modifying files, use the `--dry-run` flag:

```bash
mago format --dry-run
```

This command shows the proposed changes without applying them, allowing you to review the modifications before committing them.

> Note: The `format` command will exit with a non-zero status if any changes are planned.

### Formatting Specific Files

To format specific files or directories, pass them as arguments to the `format` command:

```bash
mago format path/to/file.php path/to/directory
```

This command formats only the specified files and directories, leaving other files untouched.

### Formatting From Stdin

To format the source code from the stdin instead of the disk files, use the `--stdin` flag.

```bash
mago format --stdin < source_file.php
```

This command leaves the disk files untouched. It outputs the result to the stdout.


## Configuration

The formatter's behavior can be customized using the `mago.toml` configuration file.

Example configuration:

```toml
[format]
print_width = 80
tab_width = 2
function_brace_style = "same_line"
method_brace_style = "same_line"
classlike_brace_style = "same_line"
```

For more information on available options and their configurations, refer to the [Settings](/formatter/settings.md) guide.

---

By incorporating the formatter into your workflow, you can ensure a clean and consistent codebase that adheres to your project's style guidelines.
