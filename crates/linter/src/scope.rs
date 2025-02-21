use mago_interner::StringIdentifier;
use mago_reflection::class_like::ClassLikeReflection;
use mago_reflection::function_like::FunctionLikeReflection;
use mago_span::Span;

use crate::context::LintContext;

/// Represents a scope related to class-like constructs.
/// This includes classes, interfaces, traits, enums, and anonymous classes.
/// The identifier or span stored in each variant is used to retrieve the corresponding reflection.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClassLikeScope {
    /// A regular class identified by its name.
    Class(StringIdentifier),
    /// An interface identified by its name.
    Interface(StringIdentifier),
    /// A trait identified by its name.
    Trait(StringIdentifier),
    /// An enum identified by its name.
    Enum(StringIdentifier),
    /// An anonymous class identified by its source code span.
    AnonymousClass(Span),
}

/// Represents a scope related to function-like constructs.
/// This includes functions, methods, closures, and arrow functions.
/// The identifier or span stored in each variant is used to retrieve the corresponding reflection.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FunctionLikeScope {
    /// A regular function identified by its name.
    Function(StringIdentifier),
    /// A method identified by its name.
    Method(StringIdentifier),
    /// A closure identified by its source code span.
    Closure(Span),
    /// An arrow function identified by its source code span.
    ArrowFunction(Span),
}

/// Represents an entry in the scope stack. It can be either class-like or function-like.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Scope {
    /// A class-like scope entry.
    ClassLike(ClassLikeScope),
    /// A function-like scope entry.
    FunctionLike(FunctionLikeScope),
}

/// A stack that tracks the scopes entered during linting.
///
/// The `ScopeStack` supports pushing and popping scope entries, and provides
/// methods to retrieve the reflection for the most recently entered scope of a
/// particular kind. For example, calling `get_class_reflection` returns the reflection
/// for the last entered class scope (if any), and similar for function-like scopes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScopeStack {
    stack: Vec<Scope>,
}

impl Default for ScopeStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeStack {
    /// Creates a new, empty `ScopeStack`.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes a new `Scope` onto the stack.
    ///
    /// # Parameters
    ///
    /// - `scope`: The scope to push onto the stack.
    pub fn push(&mut self, scope: Scope) {
        self.stack.push(scope);
    }

    /// Pops the most recently pushed `Scope` from the stack.
    ///
    /// Returns `Some(Scope)` if the stack was not empty, or `None` otherwise.
    pub fn pop(&mut self) -> Option<Scope> {
        self.stack.pop()
    }

    /// Retrieves a reference to the most recently pushed class-like scope, regardless of its variant.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeScope)` if a class-like scope exists in the stack.
    /// - `None` if no class-like scope is found.
    pub fn get_class_like_scope(&self) -> Option<&ClassLikeScope> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(class_like) => Some(class_like),
            _ => None,
        })
    }

    /// Retrieves a reference to the most recently pushed function-like scope, regardless of its variant.
    ///
    /// # Returns
    ///
    /// - `Some(&FunctionLikeScope)` if a function-like scope exists in the stack.
    /// - `None` if no function-like scope is found.
    pub fn get_function_like_scope(&self) -> Option<&FunctionLikeScope> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::FunctionLike(function_like) => Some(function_like),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent class-like scope, regardless of its specific variant.
    ///
    /// This method uses the provided `context` to look up the corresponding reflection in the codebase.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeReflection)` if the reflection is found.
    /// - `None` if no matching reflection is found.
    pub fn get_class_like_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a ClassLikeReflection> {
        self.get_class_like_scope().and_then(|class_like| match class_like {
            ClassLikeScope::Class(name) => context.codebase.get_class(context.interner, name),
            ClassLikeScope::Interface(name) => context.codebase.get_interface(context.interner, name),
            ClassLikeScope::Trait(name) => context.codebase.get_trait(context.interner, name),
            ClassLikeScope::Enum(name) => context.codebase.get_enum(context.interner, name),
            ClassLikeScope::AnonymousClass(span) => context.codebase.get_anonymous_class(span),
        })
    }

    /// Retrieves the reflection for the most recent class scope.
    ///
    /// This method searches the scope stack (in reverse order) for the first entry that
    /// is a `Class` variant in a class-like scope, and returns its reflection using the
    /// given context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeReflection)` if a class reflection is found.
    /// - `None` if no class scope is present in the stack.
    pub fn get_class_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a ClassLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(ClassLikeScope::Class(name)) => context.codebase.get_class(context.interner, name),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent interface scope.
    ///
    /// This method searches the scope stack for the first occurrence of an `Interface` variant
    /// within a class-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeReflection)` if an interface reflection is found.
    /// - `None` if no interface scope is present.
    pub fn get_interface_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a ClassLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(ClassLikeScope::Interface(name)) => context.codebase.get_interface(context.interner, name),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent trait scope.
    ///
    /// This method searches the scope stack for the first occurrence of a `Trait` variant
    /// within a class-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeReflection)` if a trait reflection is found.
    /// - `None` if no trait scope is present.
    pub fn get_trait_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a ClassLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(ClassLikeScope::Trait(name)) => context.codebase.get_trait(context.interner, name),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent enum scope.
    ///
    /// This method searches the scope stack for the first occurrence of an `Enum` variant
    /// within a class-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeReflection)` if an enum reflection is found.
    /// - `None` if no enum scope is present.
    pub fn get_enum_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a ClassLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(ClassLikeScope::Enum(name)) => context.codebase.get_enum(context.interner, name),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent anonymous class scope.
    ///
    /// This method searches the scope stack for the first occurrence of an `AnonymousClass` variant
    /// within a class-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&ClassLikeReflection)` if an anonymous class reflection is found.
    /// - `None` if no anonymous class scope is present.
    pub fn get_anonymous_class_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a ClassLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(ClassLikeScope::AnonymousClass(span)) => context.codebase.get_anonymous_class(span),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent function-like scope,
    /// regardless of its specific variant (function, method, closure, or arrow function).
    ///
    /// This method uses the provided context to fetch the corresponding reflection from the codebase.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&FunctionLikeReflection)` if a function-like reflection is found.
    /// - `None` if no function-like scope is present.
    pub fn get_function_like_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a FunctionLikeReflection> {
        self.get_function_like_scope().and_then(|function_like| match function_like {
            FunctionLikeScope::Function(name) => context.codebase.get_function(context.interner, name),
            FunctionLikeScope::Closure(span) => context.codebase.get_closure(span),
            FunctionLikeScope::ArrowFunction(span) => context.codebase.get_arrow_function(span),
            FunctionLikeScope::Method(name) => {
                let class_like = self.get_class_like_reflection(context)?;
                context.codebase.get_method(context.interner, class_like, name)
            }
        })
    }

    /// Retrieves the reflection for the most recent function scope.
    ///
    /// This method searches the scope stack for the first occurrence of a `Function` variant
    /// within a function-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&FunctionLikeReflection)` if a function reflection is found.
    /// - `None` if no function scope is present.
    pub fn get_function_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a FunctionLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::FunctionLike(FunctionLikeScope::Function(name)) => {
                context.codebase.get_function(context.interner, name)
            }
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent closure scope.
    ///
    /// This method searches the scope stack for the first occurrence of a `Closure` variant
    /// within a function-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&FunctionLikeReflection)` if a closure reflection is found.
    /// - `None` if no closure scope is present.
    pub fn get_closure_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a FunctionLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::FunctionLike(FunctionLikeScope::Closure(span)) => context.codebase.get_closure(span),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent arrow function scope.
    ///
    /// This method searches the scope stack for the first occurrence of an `ArrowFunction` variant
    /// within a function-like scope and returns its reflection using the provided context.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&FunctionLikeReflection)` if an arrow function reflection is found.
    /// - `None` if no arrow function scope is present.
    pub fn get_arrow_function_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a FunctionLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::FunctionLike(FunctionLikeScope::ArrowFunction(span)) => context.codebase.get_arrow_function(span),
            _ => None,
        })
    }

    /// Retrieves the reflection for the most recent method scope.
    ///
    /// This method searches the scope stack for the first occurrence of a `Method` variant
    /// within a function-like scope. It then retrieves the reflection for the containing
    /// class-like scope and uses it to obtain the method reflection from the context's codebase.
    ///
    /// # Parameters
    ///
    /// - `context`: The linting context containing the codebase and interner.
    ///
    /// # Returns
    ///
    /// - `Some(&FunctionLikeReflection)` if a method reflection is found.
    /// - `None` if no method scope is present or if the containing class-like reflection is not available.
    pub fn get_method_reflection<'a>(&self, context: &'a LintContext) -> Option<&'a FunctionLikeReflection> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::FunctionLike(FunctionLikeScope::Method(name)) => {
                let class_like = self.get_class_like_reflection(context)?;
                context.codebase.get_method(context.interner, class_like, name)
            }
            _ => None,
        })
    }
}
