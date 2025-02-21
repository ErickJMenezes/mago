use ordered_float::OrderedFloat;
use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;
use mago_span::Span;
use mago_trinary::Trinary;

use crate::function_like::FunctionLikeReflection;
use crate::identifier::ClassLikeName;

/// Represents a template type parameter with a name and a set of constraints.
/// For example, in `T extends Foo`, `T` is the template parameter with `Foo` as its constraint.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Template {
    /// The name of the template parameter.
    name: StringIdentifier,

    /// A list of type constraints that the template parameter must satisfy.
    constraints: Vec<TypeKind>,
}

/// Represents scalar types, including specialized scalar types with additional properties.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ScalarTypeKind {
    /// The `bool` type.
    Bool,

    /// The `int` type.
    /// The `Option` types represent inclusive minimum and maximum bounds.
    Integer { min: Option<isize>, max: Option<isize> },

    /// The `float` type.
    Float,

    /// The `string` type.
    String,

    /// An integer mask, representing a union of integers formed by bitwise OR of the given values.
    /// For example, `int-mask<1, 2, 4>` includes all combinations of these bits set.
    IntegerMask(Vec<isize>),

    /// An integer mask of constants from a class.
    /// For example, `int-mask-of<Class, CONST_PREFIX_*>` represents a mask using constants from `Class` with a given prefix.
    IntegerMaskOf(StringIdentifier, StringIdentifier),

    /// A class string type, optionally specifying a class.
    /// For example, `class-string` or `class-string<Foo>`, representing the name of a class as a string.
    ClassString(Option<StringIdentifier>),

    /// A trait string type, representing the name of a trait as a string.
    TraitString,

    /// An enum string type, representing the name of an enum as a string.
    EnumString,

    /// A callable string type, representing a string that refers to a callable function or method.
    CallableString,

    /// A numeric string type, representing a string that contains a numeric value.
    NumericString,

    /// A literal string type, representing strings known at compile time.
    LiteralString,

    /// A literal integer type, representing integers known at compile time.
    LiteralInt,

    /// A non-empty string type.
    NonEmptyString,

    /// The `array-key` type, representing values that can be used as array keys (`int` or `string`).
    ArrayKey,

    /// The `numeric` type, representing either `int` or `float`.
    Numeric,

    /// The `scalar` type, representing `bool`, `int`, `float`, or `string`.
    Scalar,
}

/// Represents a property in an object type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ObjectProperty {
    /// The name of the property.
    pub name: StringIdentifier,

    /// The type of the property.
    pub kind: TypeKind,

    /// Indicates whether the property is optional.
    pub optional: bool,
}

/// Represents object types, including specific instances and generic types.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ObjectTypeKind {
    /// Represents any object (`object` type), without specifying any properties or class.
    AnyObject,

    /// A typed object with specified properties.
    /// For example, `object{ foo: string, bar: int }` defines an object with properties `foo` and `bar`.
    TypedObject {
        /// The properties of the object.
        properties: Vec<ObjectProperty>,
    },

    /// A named object with generic type parameters.
    /// For example, `Foo<T, U>` represents an instance of class `Foo` with type parameters `T` and `U`.
    NamedObject {
        /// The name of the class.
        name: StringIdentifier,

        /// The type parameters of the object class.
        type_parameters: Vec<TypeKind>,
    },

    /// An instance of an anonymous class.
    AnonymousObject {
        /// The span of the anonymous class definition in the source code.
        span: Span,
    },

    /// An enum case, representing a specific case of an enum.
    EnumCase {
        /// The name of the enum.
        enum_name: StringIdentifier,

        /// The case of the enum.
        case_name: StringIdentifier,
    },

    /// A generator type with specified key, value, send, and return types.
    /// For example, `Generator<T, U, V, W>`.
    Generator { key: Box<TypeKind>, value: Box<TypeKind>, send: Box<TypeKind>, r#return: Box<TypeKind> },

    /// The `static` type, representing the class of the called context.
    Static {
        /// The scope of the `static` type.
        scope: StringIdentifier,
    },

    /// The `parent` type, representing the parent class in the class hierarchy.
    Parent {
        /// The scope of the `parent` type.
        scope: StringIdentifier,
    },

    /// The `self` type, representing the current class.
    Self_ {
        /// The scope of the `self` type.
        scope: StringIdentifier,
    },
}

/// Represents a key in an array shape property.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ArrayShapePropertyKey {
    String(StringIdentifier),
    Integer(isize),
}

/// Represents a property in an array shape type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ArrayShapeProperty {
    /// The key of the property.
    pub key: Option<ArrayShapePropertyKey>,

    /// The type of the property.
    pub kind: TypeKind,

    /// Indicates whether the property is optional.
    pub optional: bool,
}

/// Represents an array shape type, which is an array with specified keys and types.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ArrayShape {
    /// The properties (key-value pairs) of the array shape.
    pub properties: Vec<ArrayShapeProperty>,

    /// Additional properties specified by key and value types.
    /// For example, `...array<array-key, mixed>` allows additional entries beyond the specified properties.
    pub additional_properties: Option<(
        Box<TypeKind>, // Key type
        Box<TypeKind>, // Value type
    )>,
}

/// Represents array types, including specialized arrays like lists and shapes.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ArrayTypeKind {
    /// An array with specified key and value types.
    /// For example, `array<string, int>` represents an array with `string` keys and `int` values.
    Array {
        /// Indicates whether the array is non-empty.
        non_empty: bool,

        /// The type of the array keys.
        key: Box<TypeKind>,

        /// The type of the array values.
        value: Box<TypeKind>,

        /// The size of the array, if known.
        known_size: Option<usize>,
    },

    /// A list (array with integer keys starting from zero) with a specified value type.
    /// For example, `list<string>` represents a list of strings.
    List {
        /// Indicates whether the list is non-empty.
        non_empty: bool,

        /// The type of the list elements.
        value: Box<TypeKind>,

        /// The size of the list, if known.
        known_size: Option<usize>,
    },

    /// A callable array, representing an array that can be called as a function.
    CallableArray,

    /// An array shape with specified properties and optional additional properties.
    /// For example, `shape{ foo: string, bar: int, ... }`.
    Shape(ArrayShape),
}

/// Represents a parameter in a callable type, including its type and attributes.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct CallableParameter {
    /// The type of the parameter.
    pub kind: TypeKind,

    /// Indicates whether the parameter is optional.
    pub optional: bool,

    /// Indicates whether the parameter is variadic (i.e., accepts multiple values).
    pub variadic: bool,
}

/// Represents callable types, including functions, methods, and closures, with specified parameters and return types.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CallableTypeKind {
    /// A callable type with specified parameters and return type.
    /// For example, `callable(string, int): bool` represents a callable that accepts a `string` and an `int` and returns a `bool`.
    Callable { pure: bool, templates: Vec<Template>, parameters: Vec<CallableParameter>, return_kind: Box<TypeKind> },

    /// A closure type with specified parameters and return type.
    /// For example, `Closure(string, int): bool`.
    Closure { pure: bool, templates: Vec<Template>, parameters: Vec<CallableParameter>, return_kind: Box<TypeKind> },
}

/// Represents value types, including literal values and class constants.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ValueTypeKind {
    /// A literal string value.
    /// For example, `'foo'`.
    String {
        value: StringIdentifier,
        length: usize,
        is_uppercase: Trinary,
        is_lowercase: Trinary,
        is_ascii_lowercase: Trinary,
        is_ascii_uppercase: Trinary,
    },

    /// A literal integer value.
    /// For example, `42`.
    Integer { value: i64 },

    /// A literal float value.
    /// For example, `3.14`.
    Float { value: OrderedFloat<f64> },

    /// The `null` value.
    Null,

    /// The `true` boolean value.
    True,

    /// The `false` boolean value.
    False,

    /// A class-like constant.
    /// For example, `Foo::BAR`, where `Foo` is the class and `BAR` is the constant.
    ClassLikeConstant { class_like: ClassLikeName, constant: StringIdentifier },
}

/// Represents a `class-string-map` type, mapping class strings to values of a specified type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ClassStringMapType {
    /// The template parameter representing the class name, with constraints.
    key: Template,

    /// The value type associated with the class string.
    value: Box<TypeKind>,
}

/// Represents all possible types in the PHP static analyzer.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TypeKind {
    /// A union type, representing a value that can be any of the included types.
    /// For example, `T | U`.
    Union { kinds: Vec<TypeKind> },

    /// An intersection type, representing a value that satisfies all of the included types.
    /// For example, `T & U`.
    Intersection { kinds: Vec<TypeKind> },

    /// A scalar type, such as `int`, `string`, `bool`, or specialized scalar types.
    Scalar(ScalarTypeKind),

    /// An object type, including specific instances and generic types.
    Object(ObjectTypeKind),

    /// An array type, including lists, shapes, and arrays with specified key and value types.
    Array(ArrayTypeKind),

    /// A callable type, representing functions, methods, and closures.
    Callable(CallableTypeKind),

    /// A value type, including literals like strings, integers, and class constants.
    Value(ValueTypeKind),

    /// A conditional type, used for type inference based on a condition.
    /// For example, `T extends U ? V : W`.
    Conditional {
        /// The type being checked ( usually a generic parameter, or a variable ).
        parameter: Box<TypeKind>,

        /// The type used in the condition to compare against.
        condition: Box<TypeKind>,

        /// The type when the condition is true.
        then: Box<TypeKind>,

        /// The type when the condition is false.
        otherwise: Box<TypeKind>,
    },

    /// Represents the keys of a type.
    /// For example, `key-of<T>` extracts the keys from type `T`.
    KeyOf { kind: Box<TypeKind> },

    /// Represents the values of a type.
    /// For example, `value-of<T>` extracts the values from type `T`.
    ValueOf { kind: Box<TypeKind> },

    /// Represents the properties of a type.
    /// For example, `properties-of<T>` extracts the properties from type `T`.
    PropertiesOf { kind: Box<TypeKind> },

    /// A `class-string-map` type, mapping class strings to values of a specified type.
    /// For example, `class-string-map<T of Foo, T>` maps class names extending `Foo` to values of type `T`.
    ClassStringMap {
        /// The template parameter representing the class name, with constraints.
        key: Template,

        /// The value type associated with the class string.
        value_kind: Box<TypeKind>,
    },

    /// An indexed access type, representing the type at a specific key.
    /// For example, `T[K]` accesses the type of property `K` in type `T`.
    Index { base_kind: Box<TypeKind>, index_kind: Box<TypeKind> },

    /// A variable type, representing a type associated with a variable.
    /// For example, `$foo`.
    Variable { name: StringIdentifier },

    /// An iterable type with specified key and value types.
    /// For example, `iterable<string, int>`.
    Iterable { key: Box<TypeKind>, value: Box<TypeKind> },

    /// The `void` type, representing the absence of a value.
    Void,

    /// The `resource` type, representing a resource handle.
    Resource,

    /// The `closed-resource` type, representing a resource that has been closed (e.g., using `fclose()`).
    ClosedResource,

    /// The `mixed` type, representing any type.
    Mixed {
        /// Whether the `mixed` type is explicit, or inferred from context (e.g., a function with no return type).
        explicit: bool,
    },

    /// The `never` type, representing a type that never occurs (e.g., functions that always throw exceptions or exit).
    Never,

    /// A generic parameter type, representing a type parameter with constraints.
    GenericParameter { name: StringIdentifier, of: Box<TypeKind>, defined_in: StringIdentifier },
}

impl Template {
    pub fn get_key(&self, interner: &ThreadedInterner) -> String {
        let mut key = String::from(interner.lookup(&self.name));

        for constraint in &self.constraints {
            key.push_str(" of ");
            key.push_str(&constraint.get_key(interner));
        }

        key
    }
}

impl ArrayShapePropertyKey {
    pub fn get_key(&self, interner: &ThreadedInterner) -> String {
        match &self {
            ArrayShapePropertyKey::String(string_identifier) => interner.lookup(string_identifier).to_owned(),
            ArrayShapePropertyKey::Integer(i) => i.to_string(),
        }
    }
}

impl TypeKind {
    #[inline]
    pub fn is_nullable(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_nullable()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_nullable()).collect(),
            TypeKind::Value(ValueTypeKind::Null) => Trinary::True,
            TypeKind::Mixed { .. } => Trinary::Maybe,
            TypeKind::Scalar(_) => Trinary::False,
            TypeKind::Object(_) => todo!(),
            TypeKind::Array(_) => todo!(),
            TypeKind::Callable(_) => todo!(),
            TypeKind::Conditional { then, otherwise, .. } => then.is_nullable() & otherwise.is_nullable(),
            TypeKind::KeyOf { .. } => Trinary::False,
            TypeKind::ValueOf { .. } => Trinary::Maybe,
            TypeKind::PropertiesOf { .. } => Trinary::Maybe,
            TypeKind::ClassStringMap { .. } => Trinary::False,
            TypeKind::Index { .. } => Trinary::Maybe,
            TypeKind::Variable { .. } => Trinary::Maybe,
            TypeKind::Iterable { .. } => Trinary::False,
            TypeKind::Void => Trinary::True,
            TypeKind::Resource => Trinary::False,
            TypeKind::ClosedResource => Trinary::False,
            TypeKind::Never => Trinary::False,
            TypeKind::GenericParameter { of, .. } => of.is_nullable(),
            TypeKind::Value(_) => Trinary::False,
        }
    }

    #[inline]
    pub fn is_object(&self) -> bool {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().all(|k| k.is_object()),
            TypeKind::Intersection { kinds } => kinds.iter().any(|k| k.is_object()),
            TypeKind::Conditional { then, otherwise, .. } => then.is_object() && otherwise.is_object(),
            TypeKind::Callable(CallableTypeKind::Closure { .. }) => true,
            TypeKind::GenericParameter { of, .. } => of.is_object(),
            TypeKind::Object(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_resource(&self) -> bool {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().all(|k| k.is_resource()),
            TypeKind::Intersection { kinds } => kinds.iter().any(|k| k.is_resource()),
            TypeKind::Conditional { then, otherwise, .. } => then.is_resource() && otherwise.is_resource(),
            TypeKind::Resource | TypeKind::ClosedResource => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().all(|k| k.is_array()),
            TypeKind::Intersection { kinds } => kinds.iter().any(|k| k.is_array()),
            TypeKind::Conditional { then, otherwise, .. } => then.is_array() && otherwise.is_array(),
            TypeKind::Array(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_bool(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_bool()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_bool()).collect(),
            TypeKind::Conditional { then, otherwise, .. } => then.is_bool().and(otherwise.is_bool()),
            TypeKind::Value(ValueTypeKind::True)
            | TypeKind::Value(ValueTypeKind::False)
            | TypeKind::Scalar(ScalarTypeKind::Bool) => Trinary::True,
            TypeKind::Value(ValueTypeKind::ClassLikeConstant { .. }) => Trinary::Maybe,
            TypeKind::GenericParameter { of, .. } => of.is_bool(),
            _ => Trinary::False,
        }
    }

    #[inline]
    pub fn is_truthy(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_truthy()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_truthy()).collect(),
            TypeKind::Conditional { then, otherwise, .. } => then.is_truthy().and(otherwise.is_truthy()),
            TypeKind::Array(array_kind) => match array_kind {
                ArrayTypeKind::Array { non_empty, known_size, .. }
                    if *non_empty || known_size.map(|s| s > 0).unwrap_or(false) =>
                {
                    Trinary::True
                }
                ArrayTypeKind::List { non_empty, known_size, .. }
                    if *non_empty || known_size.map(|s| s > 0).unwrap_or(false) =>
                {
                    Trinary::True
                }
                ArrayTypeKind::CallableArray => Trinary::True,
                ArrayTypeKind::Shape(array_shape) => Trinary::from(!array_shape.properties.is_empty()),
                _ => Trinary::Maybe,
            },
            TypeKind::Scalar(scalar_type_kind) => match scalar_type_kind {
                ScalarTypeKind::Bool => Trinary::Maybe,
                ScalarTypeKind::Integer { min, max } => {
                    if min.map(|m| m > 0).unwrap_or(false) {
                        Trinary::True
                    } else if max.map(|m| m < 0).unwrap_or(false) {
                        Trinary::False
                    } else {
                        Trinary::Maybe
                    }
                }
                ScalarTypeKind::Float => Trinary::Maybe,
                ScalarTypeKind::String => Trinary::Maybe,
                ScalarTypeKind::IntegerMask(bits) => {
                    if bits.iter().all(|b| *b > 0) {
                        Trinary::True
                    } else if bits.iter().all(|b| *b < 0) {
                        Trinary::False
                    } else {
                        Trinary::Maybe
                    }
                }
                ScalarTypeKind::IntegerMaskOf(_, _) => Trinary::Maybe,
                ScalarTypeKind::ClassString(_)
                | ScalarTypeKind::TraitString
                | ScalarTypeKind::EnumString
                | ScalarTypeKind::CallableString => Trinary::True,
                ScalarTypeKind::NumericString => Trinary::Maybe, // `"0"` is a numeric string, but falsy
                ScalarTypeKind::LiteralString => Trinary::Maybe,
                ScalarTypeKind::LiteralInt => Trinary::Maybe,
                ScalarTypeKind::NonEmptyString => Trinary::Maybe, // `"0"` is a non-empty string, but falsy
                ScalarTypeKind::ArrayKey => Trinary::Maybe,
                ScalarTypeKind::Numeric => Trinary::Maybe,
                ScalarTypeKind::Scalar => Trinary::Maybe,
            },
            TypeKind::Object(_) => Trinary::True,
            TypeKind::Callable(_) => Trinary::True,
            TypeKind::Value(value_type_kind) => match &value_type_kind {
                ValueTypeKind::String { .. } => Trinary::Maybe,
                ValueTypeKind::Integer { value } => {
                    if *value > 0 {
                        Trinary::True
                    } else {
                        Trinary::False
                    }
                }
                ValueTypeKind::Float { value } => {
                    if *value > OrderedFloat(0.0) {
                        Trinary::True
                    } else {
                        Trinary::False
                    }
                }
                ValueTypeKind::Null => Trinary::False,
                ValueTypeKind::True => Trinary::True,
                ValueTypeKind::False => Trinary::False,
                ValueTypeKind::ClassLikeConstant { .. } => Trinary::Maybe,
            },
            TypeKind::Variable { .. } => Trinary::Maybe,
            TypeKind::Iterable { .. } => Trinary::Maybe,
            TypeKind::Void => Trinary::False,
            TypeKind::Resource => Trinary::True,
            TypeKind::ClosedResource => Trinary::True,
            TypeKind::Never => Trinary::False,
            TypeKind::GenericParameter { of, .. } => of.is_truthy(),
            _ => Trinary::Maybe,
        }
    }

    #[inline]
    pub fn is_falsy(&self) -> Trinary {
        self.is_truthy().negate()
    }

    #[inline]
    pub fn is_float(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_float()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_float()).collect(),
            TypeKind::Conditional { then, otherwise, .. } => then.is_float().and(otherwise.is_float()),
            TypeKind::Scalar(scalar_type_kind) => match scalar_type_kind {
                ScalarTypeKind::Float => Trinary::True,
                ScalarTypeKind::Integer { .. } => Trinary::False,
                ScalarTypeKind::IntegerMask(_) => Trinary::False,
                ScalarTypeKind::IntegerMaskOf(_, _) => Trinary::False,
                ScalarTypeKind::Numeric => Trinary::Maybe,
                _ => Trinary::False,
            },
            TypeKind::Value(ValueTypeKind::Float { .. }) => Trinary::True,
            TypeKind::Value(ValueTypeKind::ClassLikeConstant { .. }) => Trinary::Maybe,
            _ => Trinary::False,
        }
    }

    #[inline]
    pub fn is_integer(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_integer()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_integer()).collect(),
            TypeKind::Conditional { then, otherwise, .. } => then.is_integer().and(otherwise.is_integer()),
            TypeKind::Scalar(scalar_type_kind) => match scalar_type_kind {
                ScalarTypeKind::Integer { .. } => Trinary::True,
                ScalarTypeKind::IntegerMask(_) => Trinary::True,
                ScalarTypeKind::IntegerMaskOf(_, _) => Trinary::True,
                ScalarTypeKind::LiteralInt => Trinary::True,
                ScalarTypeKind::Numeric => Trinary::Maybe,
                ScalarTypeKind::ArrayKey => Trinary::Maybe,
                _ => Trinary::False,
            },
            TypeKind::Value(ValueTypeKind::Integer { .. }) => Trinary::True,
            TypeKind::Value(ValueTypeKind::ClassLikeConstant { .. }) => Trinary::Maybe,
            _ => Trinary::False,
        }
    }

    #[inline]
    pub fn is_string(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_string()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_string()).collect(),
            TypeKind::Conditional { then, otherwise, .. } => then.is_string().and(otherwise.is_string()),
            TypeKind::Scalar(scalar_type_kind) => match scalar_type_kind {
                ScalarTypeKind::String => Trinary::True,
                ScalarTypeKind::ClassString(_) => Trinary::True,
                ScalarTypeKind::TraitString => Trinary::True,
                ScalarTypeKind::EnumString => Trinary::True,
                ScalarTypeKind::CallableString => Trinary::True,
                ScalarTypeKind::NumericString => Trinary::True,
                ScalarTypeKind::LiteralString => Trinary::True,
                ScalarTypeKind::NonEmptyString => Trinary::True,
                _ => Trinary::False,
            },
            TypeKind::Value(ValueTypeKind::String { .. }) => Trinary::True,
            TypeKind::Value(ValueTypeKind::ClassLikeConstant { .. }) => Trinary::Maybe,
            _ => Trinary::False,
        }
    }

    #[inline]
    pub fn is_non_empty_string(&self) -> Trinary {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.is_non_empty_string()).collect(),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.is_non_empty_string()).collect(),
            TypeKind::Conditional { then, otherwise, .. } => {
                then.is_non_empty_string().and(otherwise.is_non_empty_string())
            }
            TypeKind::Scalar(scalar_type_kind) => match scalar_type_kind {
                ScalarTypeKind::String => Trinary::Maybe,
                ScalarTypeKind::ClassString(_) => Trinary::True,
                ScalarTypeKind::TraitString => Trinary::True,
                ScalarTypeKind::EnumString => Trinary::True,
                ScalarTypeKind::CallableString => Trinary::True,
                ScalarTypeKind::NumericString => Trinary::True,
                ScalarTypeKind::LiteralString => Trinary::Maybe,
                ScalarTypeKind::NonEmptyString => Trinary::True,
                _ => Trinary::False,
            },
            TypeKind::Value(ValueTypeKind::String { length, .. }) => (*length > 0).into(),
            TypeKind::Value(ValueTypeKind::ClassLikeConstant { .. }) => Trinary::Maybe,
            _ => Trinary::False,
        }
    }

    #[inline]
    pub fn is_value(&self) -> bool {
        matches!(self, TypeKind::Value(_))
    }

    #[inline]
    pub fn is_templated_as_object(&self) -> bool {
        matches!(self, TypeKind::GenericParameter { of, .. } if of.is_object())
    }

    #[inline]
    pub fn is_generator(&self) -> bool {
        matches!(self, TypeKind::Object(ObjectTypeKind::Generator { .. }))
    }

    pub fn get_key(&self, interner: &ThreadedInterner) -> String {
        match &self {
            TypeKind::Union { kinds } => kinds.iter().map(|k| k.get_key(interner)).collect::<Vec<_>>().join("|"),
            TypeKind::Intersection { kinds } => kinds.iter().map(|k| k.get_key(interner)).collect::<Vec<_>>().join("&"),
            TypeKind::Scalar(scalar_type_kind) => match &scalar_type_kind {
                ScalarTypeKind::Bool => "bool".to_string(),
                ScalarTypeKind::Float => "float".to_string(),
                ScalarTypeKind::String => "string".to_string(),
                ScalarTypeKind::Integer { min, max } => match (min, max) {
                    (None, None) => "int".to_string(),
                    (Some(min), None) => format!("int<{}, max>", min),
                    (None, Some(max)) => format!("int<min, {}>", max),
                    (Some(min), Some(max)) => format!("int<{}, {}>", min, max),
                },
                ScalarTypeKind::IntegerMask(vec) => {
                    let vec = vec.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");

                    format!("int-mask<{}>", vec)
                }
                ScalarTypeKind::IntegerMaskOf(string_identifier, string_identifier1) => {
                    format!(
                        "int-mask-of<{}, {}>",
                        interner.lookup(string_identifier),
                        interner.lookup(string_identifier1)
                    )
                }
                ScalarTypeKind::ClassString(string_identifier) => {
                    if let Some(string_identifier) = string_identifier {
                        format!("class-string<{}>", interner.lookup(string_identifier))
                    } else {
                        "class-string".to_string()
                    }
                }
                ScalarTypeKind::TraitString => "trait-string".to_string(),
                ScalarTypeKind::EnumString => "enum-string".to_string(),
                ScalarTypeKind::CallableString => "callable-string".to_string(),
                ScalarTypeKind::NumericString => "numeric-string".to_string(),
                ScalarTypeKind::LiteralString => "literal-string".to_string(),
                ScalarTypeKind::LiteralInt => "literal-int".to_string(),
                ScalarTypeKind::NonEmptyString => "non-empty-string".to_string(),
                ScalarTypeKind::ArrayKey => "array-key".to_string(),
                ScalarTypeKind::Numeric => "numeric".to_string(),
                ScalarTypeKind::Scalar => "scalar".to_string(),
            },
            TypeKind::Object(object_type_kind) => match &object_type_kind {
                ObjectTypeKind::AnyObject => "object".to_string(),
                ObjectTypeKind::TypedObject { properties } => {
                    let properties = properties
                        .iter()
                        .map(|property| {
                            let name = interner.lookup(&property.name);
                            let kind = property.kind.get_key(interner);

                            if property.optional {
                                format!("{}?: {}", name, kind)
                            } else {
                                format!("{}: {}", name, kind)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("object{{{}}}", properties)
                }
                ObjectTypeKind::NamedObject { name, type_parameters } => {
                    let name = interner.lookup(name);

                    if type_parameters.is_empty() {
                        name.to_string()
                    } else {
                        let type_parameters = type_parameters
                            .iter()
                            .map(|type_parameter| type_parameter.get_key(interner))
                            .collect::<Vec<_>>()
                            .join(", ");

                        format!("{}<{}>", name, type_parameters)
                    }
                }
                ObjectTypeKind::AnonymousObject { span } => {
                    format!(
                        "anonymous-class@{}:{}-{}",
                        interner.lookup(&span.start.source.0),
                        span.start.offset,
                        span.end.offset
                    )
                }
                ObjectTypeKind::Generator { key, value, send, r#return } => {
                    let key = key.get_key(interner);
                    let value = value.get_key(interner);
                    let send = send.get_key(interner);
                    let r#return = r#return.get_key(interner);

                    format!("Generator<{}, {}, {}, {}>", key, value, send, r#return)
                }
                ObjectTypeKind::Static { .. } => "static".to_string(),
                ObjectTypeKind::Parent { .. } => "parent".to_string(),
                ObjectTypeKind::Self_ { .. } => "self".to_string(),
                ObjectTypeKind::EnumCase { enum_name: name, case_name: case } => {
                    let name = interner.lookup(name);
                    let case = interner.lookup(case);

                    format!("enum({}::{})", name, case)
                }
            },
            TypeKind::Array(array_type_kind) => match &array_type_kind {
                ArrayTypeKind::Array { non_empty, key, value, .. } => {
                    let key = key.get_key(interner);
                    let value = value.get_key(interner);

                    if *non_empty {
                        format!("non-empty-array<{}, {}>", key, value)
                    } else {
                        format!("array<{}, {}>", key, value)
                    }
                }
                ArrayTypeKind::List { non_empty, value, .. } => {
                    let value = value.get_key(interner);

                    if *non_empty { format!("non-empty-list<{}>", value) } else { format!("list<{}>", value) }
                }
                ArrayTypeKind::CallableArray => "callable-array".to_string(),
                ArrayTypeKind::Shape(array_shape) => {
                    let mut properties = array_shape
                        .properties
                        .iter()
                        .map(|property| {
                            let kind = property.kind.get_key(interner);

                            if let Some(key) = property.key.as_ref() {
                                let key = key.get_key(interner);

                                if property.optional {
                                    format!("{}?: {}", key, kind)
                                } else {
                                    format!("{}: {}", key, kind)
                                }
                            } else {
                                kind
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    if let Some((key, value)) = &array_shape.additional_properties {
                        if matches!(
                            (key.as_ref(), value.as_ref()),
                            (TypeKind::Scalar(ScalarTypeKind::ArrayKey), TypeKind::Mixed { .. })
                        ) {
                            properties.push_str(", ...");
                        } else {
                            let key = key.get_key(interner);
                            let value = value.get_key(interner);

                            properties.push_str(&format!(", ...array<{}: {}>", key, value));
                        }
                    }

                    format!("array{{{}}}", properties)
                }
            },
            TypeKind::Callable(callable_type_kind) => match &callable_type_kind {
                CallableTypeKind::Callable { pure, templates, parameters, return_kind } => {
                    let parameters = parameters
                        .iter()
                        .map(|parameter| {
                            let mut kind = parameter.kind.get_key(interner);
                            if parameter.optional {
                                kind.push('=');
                            }

                            if parameter.variadic {
                                kind.push_str("...");
                            }

                            kind
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    let return_kind = return_kind.get_key(interner);

                    let templates =
                        templates.iter().map(|template| template.get_key(interner)).collect::<Vec<_>>().join(", ");
                    let templates = if !templates.is_empty() { format!("<{}>", templates) } else { "".to_string() };

                    if *pure {
                        format!("(pure-callable{}({}): {})", templates, parameters, return_kind)
                    } else {
                        format!("(callable{}({}): {})", templates, parameters, return_kind)
                    }
                }
                CallableTypeKind::Closure { pure, templates, parameters, return_kind } => {
                    let parameters = parameters
                        .iter()
                        .map(|parameter| {
                            let mut kind = parameter.kind.get_key(interner);
                            if parameter.optional {
                                kind.push('=');
                            }

                            if parameter.variadic {
                                kind.push_str("...");
                            }

                            kind
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    let return_kind = return_kind.get_key(interner);

                    let templates =
                        templates.iter().map(|template| template.get_key(interner)).collect::<Vec<_>>().join(", ");
                    let templates = if !templates.is_empty() { format!("<{}>", templates) } else { "".to_string() };

                    if *pure {
                        format!("(pure-Closure{}({}): {})", templates, parameters, return_kind)
                    } else {
                        format!("(Closure{}({}): {})", templates, parameters, return_kind)
                    }
                }
            },
            TypeKind::Value(value_type_kind) => match &value_type_kind {
                ValueTypeKind::String { value, .. } => {
                    format!("\"{}\"", value)
                }
                ValueTypeKind::Integer { value } => value.to_string(),
                ValueTypeKind::Float { value } => value.to_string(),
                ValueTypeKind::Null => "null".to_string(),
                ValueTypeKind::True => "true".to_string(),
                ValueTypeKind::False => "false".to_string(),
                ValueTypeKind::ClassLikeConstant { class_like, constant } => {
                    format!("{}::{}", class_like.get_key(interner), interner.lookup(constant))
                }
            },
            TypeKind::Conditional { parameter, condition, then, otherwise } => {
                let parameter = parameter.get_key(interner);
                let condition = condition.get_key(interner);
                let then = then.get_key(interner);
                let otherwise = otherwise.get_key(interner);

                format!("{} is {} ? {} : {}", parameter, condition, then, otherwise)
            }
            TypeKind::KeyOf { kind } => {
                let kind = kind.get_key(interner);

                format!("key-of<{}>", kind)
            }
            TypeKind::ValueOf { kind } => {
                let kind = kind.get_key(interner);

                format!("value-of<{}>", kind)
            }
            TypeKind::PropertiesOf { kind } => {
                let kind = kind.get_key(interner);

                format!("properties-of<{}>", kind)
            }
            TypeKind::ClassStringMap { key, value_kind } => {
                let mut template = interner.lookup(&key.name).to_owned();
                for constraint in &key.constraints {
                    template.push_str(&format!(" of {}", constraint.get_key(interner)));
                }

                let value_kind = value_kind.get_key(interner);

                format!("class-string-map<{}, {}>", template, value_kind)
            }
            TypeKind::Index { base_kind, index_kind } => {
                let base_kind = base_kind.get_key(interner);
                let index_kind = index_kind.get_key(interner);

                format!("{}[{}]", base_kind, index_kind)
            }
            TypeKind::Variable { name } => interner.lookup(name).to_owned(),
            TypeKind::Iterable { key, value } => {
                let key = key.get_key(interner);
                let value = value.get_key(interner);

                format!("iterable<{}, {}>", key, value)
            }
            TypeKind::Void => "void".to_string(),
            TypeKind::Resource => "resource".to_string(),
            TypeKind::ClosedResource => "closed-resource".to_string(),
            TypeKind::Mixed { explicit } => {
                if *explicit {
                    "mixed".to_string()
                } else {
                    "unknown".to_string()
                }
            }
            TypeKind::Never => "never".to_string(),
            TypeKind::GenericParameter { name, defined_in, .. } => {
                format!("{}:{}", interner.lookup(name), interner.lookup(defined_in))
            }
        }
    }
}

/// Creates a `TypeKind` representing the `bool` type.
pub fn bool_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Bool)
}

/// Creates a `TypeKind` representing the `int` type.
pub fn integer_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: None, max: None })
}

pub fn positive_integer_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: Some(1), max: None })
}

pub fn non_negative_integer_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: Some(0), max: None })
}

pub fn negative_integer_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: None, max: Some(-1) })
}

pub fn non_positive_integer_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: None, max: Some(0) })
}

pub fn minimum_integer_kind(min: isize) -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: Some(min), max: None })
}

pub fn maximum_integer_kind(max: isize) -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: None, max: Some(max) })
}

pub fn integer_range_kind(min: isize, max: isize) -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Integer { min: Some(min), max: Some(max) })
}

/// Creates a `TypeKind` representing the `float` type.
pub fn float_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::Float)
}

/// Creates a `TypeKind` representing the `string` type.
pub fn string_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::String)
}

/// Creates a `TypeKind` representing the `non-empty-string` type.
pub fn non_empty_string_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::NonEmptyString)
}

/// Creates a `TypeKind` representing a list of the given type.
pub fn list_kind(value: TypeKind, known_size: Option<usize>) -> TypeKind {
    TypeKind::Array(ArrayTypeKind::List { non_empty: false, value: Box::new(value), known_size })
}

/// Creates a `TypeKind` representing an array with the given key and value types.
pub fn array_kind(key: TypeKind, value: TypeKind, known_size: Option<usize>) -> TypeKind {
    TypeKind::Array(ArrayTypeKind::Array { non_empty: false, key: Box::new(key), value: Box::new(value), known_size })
}

/// Creates a `TypeKind` representing a non-empty list of the given type.
pub fn non_empty_list_kind(value: TypeKind, known_size: Option<usize>) -> TypeKind {
    TypeKind::Array(ArrayTypeKind::List { non_empty: true, value: Box::new(value), known_size })
}

/// Creates a `TypeKind` representing a non-empty array with the given key and value types.
pub fn non_empty_array_kind(key: TypeKind, value: TypeKind, known_size: Option<usize>) -> TypeKind {
    TypeKind::Array(ArrayTypeKind::Array { non_empty: true, key: Box::new(key), value: Box::new(value), known_size })
}

pub fn indexed_shape_property(kind: TypeKind, optional: bool) -> ArrayShapeProperty {
    ArrayShapeProperty { key: None, kind, optional }
}

pub fn string_shape_property(key: StringIdentifier, kind: TypeKind, optional: bool) -> ArrayShapeProperty {
    ArrayShapeProperty { key: Some(ArrayShapePropertyKey::String(key)), kind, optional }
}

pub fn integer_shape_property(key: isize, kind: TypeKind, optional: bool) -> ArrayShapeProperty {
    ArrayShapeProperty { key: Some(ArrayShapePropertyKey::Integer(key)), kind, optional }
}

pub fn array_shape_kind(
    properties: Vec<ArrayShapeProperty>,
    additional_properties: Option<(TypeKind, TypeKind)>,
) -> TypeKind {
    TypeKind::Array(ArrayTypeKind::Shape(ArrayShape {
        properties,
        additional_properties: additional_properties.map(|(k, v)| (Box::new(k), Box::new(v))),
    }))
}

/// Creates a `TypeKind` representing the `mixed` type.
pub fn mixed_kind(explicit: bool) -> TypeKind {
    TypeKind::Mixed { explicit }
}

/// Creates a `TypeKind` representing a union of the given types.
pub fn union_kind(kinds: Vec<TypeKind>) -> TypeKind {
    TypeKind::Union { kinds }
}

/// Creates a `TypeKind` representing an intersection of the given types.
pub fn intersection_kind(kinds: Vec<TypeKind>) -> TypeKind {
    TypeKind::Intersection { kinds }
}

/// Creates a `CallableParameter` with the given kind, optional flag, and variadic flag.
pub fn callable_parameter(kind: TypeKind, optional: bool, variadic: bool) -> CallableParameter {
    CallableParameter { kind, optional, variadic }
}

/// Creates a `TypeKind` representing a callable typewith an unknown number of parameters and
/// an implicit mixed return type.
pub fn any_callable_kind() -> TypeKind {
    callable_kind(false, vec![], vec![callable_parameter(mixed_kind(false), true, true)], mixed_kind(false))
}

/// Creates a `TypeKind` representing a callable type with the given parameters and return type.
pub fn callable_kind(
    pure: bool,
    templates: Vec<Template>,
    parameters: Vec<CallableParameter>,
    return_kind: TypeKind,
) -> TypeKind {
    TypeKind::Callable(CallableTypeKind::Callable { pure, templates, parameters, return_kind: Box::new(return_kind) })
}

/// Creates a `TypeKind` representing a closure type with an unknown number of parameters and
/// an implicit mixed return type.
pub fn any_closure_kind() -> TypeKind {
    closure_kind(false, vec![], vec![callable_parameter(mixed_kind(false), true, true)], mixed_kind(false))
}

/// Creates a `TypeKind` representing a closure type with the given parameters and return type.
pub fn closure_kind(
    pure: bool,
    templates: Vec<Template>,
    parameters: Vec<CallableParameter>,
    return_kind: TypeKind,
) -> TypeKind {
    TypeKind::Callable(CallableTypeKind::Closure { pure, templates, parameters, return_kind: Box::new(return_kind) })
}

/// Creates a `TypeKind` representing a variable type with the given name.
pub fn variable_kind(name: StringIdentifier) -> TypeKind {
    TypeKind::Variable { name }
}

/// Creates a `TypeKind` representing a value type for a literal string.
pub fn value_string_kind(
    value: StringIdentifier,
    length: usize,
    is_uppercase: Trinary,
    is_ascii_uppercase: Trinary,
    is_lowercase: Trinary,
    is_ascii_lowercase: Trinary,
) -> TypeKind {
    TypeKind::Value(ValueTypeKind::String {
        value,
        length,
        is_uppercase,
        is_lowercase,
        is_ascii_uppercase,
        is_ascii_lowercase,
    })
}

/// Creates a `TypeKind` representing a value type for a literal integer.
pub fn value_integer_kind(value: i64) -> TypeKind {
    TypeKind::Value(ValueTypeKind::Integer { value })
}

/// Creates a `TypeKind` representing a value type for a literal float.
pub fn value_float_kind(value: OrderedFloat<f64>) -> TypeKind {
    TypeKind::Value(ValueTypeKind::Float { value })
}

/// Creates a `TypeKind` representing the `null` value.
pub fn null_kind() -> TypeKind {
    TypeKind::Value(ValueTypeKind::Null)
}

/// Creates a `TypeKind` representing the `true` boolean value.
pub fn true_kind() -> TypeKind {
    TypeKind::Value(ValueTypeKind::True)
}

/// Creates a `TypeKind` representing the `false` boolean value.
pub fn false_kind() -> TypeKind {
    TypeKind::Value(ValueTypeKind::False)
}

/// Creates a `TypeKind` representing an iterable with the given key and value types.
pub fn iterable_kind(key_kind: TypeKind, value_kind: TypeKind) -> TypeKind {
    TypeKind::Iterable { key: Box::new(key_kind), value: Box::new(value_kind) }
}

/// Creates a `TypeKind` representing an object of any type.
pub fn any_object_kind() -> TypeKind {
    TypeKind::Object(ObjectTypeKind::AnyObject)
}

/// Creates a `TypeKind` representing the `static` type of the given class.
pub fn static_kind(scope: StringIdentifier) -> TypeKind {
    TypeKind::Object(ObjectTypeKind::Static { scope })
}

/// Creates a `TypeKind` representing the `parent` type of the given class.
pub fn parent_kind(scope: StringIdentifier) -> TypeKind {
    TypeKind::Object(ObjectTypeKind::Parent { scope })
}

/// Creates a `TypeKind` representing the `self` type of the given class.
pub fn self_kind(scope: StringIdentifier) -> TypeKind {
    TypeKind::Object(ObjectTypeKind::Self_ { scope })
}

/// Creates a `TypeKind` representing a named object with the given name and type parameters.
pub fn named_object_kind(name: StringIdentifier, type_parameters: Vec<TypeKind>) -> TypeKind {
    TypeKind::Object(ObjectTypeKind::NamedObject { name, type_parameters })
}

/// Creates a `TypeKind` representing an instance of an object with the given name and type parameters.
pub fn anonymous_object_kind(span: Span) -> TypeKind {
    TypeKind::Object(ObjectTypeKind::AnonymousObject { span })
}

pub fn enum_case_kind(enum_name: StringIdentifier, case_name: StringIdentifier) -> TypeKind {
    TypeKind::Object(ObjectTypeKind::EnumCase { enum_name, case_name })
}

/// Creates a `TypeKind` representing the `void` type.
pub fn void_kind() -> TypeKind {
    TypeKind::Void
}

/// Creates a `TypeKind` representing the `never` type.
pub fn never_kind() -> TypeKind {
    TypeKind::Never
}

/// Creates a `TypeKind` representing the `resource` type.
pub fn resource_kind() -> TypeKind {
    TypeKind::Resource
}

/// Creates a `TypeKind` representing the `closed-resource` type.
pub fn closed_resource_kind() -> TypeKind {
    TypeKind::ClosedResource
}

/// Creates a `TypeKind` representing a key-of type.
pub fn key_of_kind(kind: TypeKind) -> TypeKind {
    TypeKind::KeyOf { kind: Box::new(kind) }
}

/// Creates a `TypeKind` representing a value-of type.
pub fn value_of_kind(kind: TypeKind) -> TypeKind {
    TypeKind::ValueOf { kind: Box::new(kind) }
}

/// Creates a `TypeKind` representing a properties-of type.
pub fn properties_of_kind(kind: TypeKind) -> TypeKind {
    TypeKind::PropertiesOf { kind: Box::new(kind) }
}

/// Creates a `TypeKind` representing a conditional type.
pub fn conditional_kind(parameter: TypeKind, condition: TypeKind, then: TypeKind, otherwise: TypeKind) -> TypeKind {
    TypeKind::Conditional {
        parameter: Box::new(parameter),
        condition: Box::new(condition),
        then: Box::new(then),
        otherwise: Box::new(otherwise),
    }
}

/// Creates a `TypeKind` representing a class-string-map type.
pub fn class_string_map_kind(key_template: Template, value_kind: TypeKind) -> TypeKind {
    TypeKind::ClassStringMap { key: key_template, value_kind: Box::new(value_kind) }
}

/// Creates a `TypeKind` representing an index type.
pub fn index_kind(base_kind: TypeKind, index_kind: TypeKind) -> TypeKind {
    TypeKind::Index { base_kind: Box::new(base_kind), index_kind: Box::new(index_kind) }
}

/// Creates a `TypeKind` representing an array-key type.
pub fn array_key_kind() -> TypeKind {
    TypeKind::Scalar(ScalarTypeKind::ArrayKey)
}

impl From<&FunctionLikeReflection> for TypeKind {
    fn from(reflection: &FunctionLikeReflection) -> Self {
        let parameters: Vec<_> = reflection
            .parameters
            .iter()
            .map(|parameter| CallableParameter {
                optional: parameter.default.is_some(),
                kind: parameter.type_reflection.as_ref().map(|r| r.kind.clone()).unwrap_or_else(|| mixed_kind(false)),
                variadic: parameter.is_variadic,
            })
            .collect();

        let return_kind = reflection
            .return_type_reflection
            .as_ref()
            .map(|r| r.type_reflection.kind.clone())
            .unwrap_or_else(|| mixed_kind(false));

        TypeKind::Callable(CallableTypeKind::Closure {
            pure: reflection.is_pure,
            templates: reflection.templates.clone(),
            parameters,
            return_kind: Box::new(return_kind),
        })
    }
}
