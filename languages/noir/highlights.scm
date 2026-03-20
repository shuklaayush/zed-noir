((identifier) @constant
 (#match? @constant "^[A-Z][A-Z0-9_]*$"))

(comment) @comment
(string_literal) @string
(character) @string
(boolean) @boolean
(integer) @number
(float) @number

(single_type) @type.builtin
(generic_type
  (identifier) @type)
(array_type) @type

(struct_definition
  name: (identifier) @type)
(trait_definition
  (identifier) @type)
(trait_alias
  (identifier) @type)
(module
  (identifier) @namespace)
(impl_block
  (identifier) @type)

(struct_definition
  var: (identifier) @property)

(function_definition
  (identifier) @function)
(trait_function_signature
  (identifier) @function)
(function_call
  (identifier) @function)

(function_definition
  (parameter
    (typed_identifier
      var: (identifier) @variable.parameter)))
(trait_function_signature
  (parameter
    (typed_identifier
      var: (identifier) @variable.parameter)))
(function_definition
  (parameter
    (identifier) @variable.parameter))
(trait_function_signature
  (parameter
    (identifier) @variable.parameter))

(let_declaration
  (binary_expression
    left: (identifier) @variable))
(let_declaration
  (binary_expression
    left: (typed_identifier
      var: (identifier) @variable)))
(let_declaration
  (binary_expression
    left: (struct_initialization
      var: (identifier) @property)))

(import
  (import_identifier
    (identifier) @namespace))
(import
  (import_identifier
    (crate) @keyword))
(import
  (import_identifier
    (super) @keyword))
(import
  (identifier) @namespace)
(import_variable
  (import_identifier
    (identifier) @namespace))
(import_variable
  (identifier) @namespace)

(macro
  (identifier) @attribute)

(self) @variable.builtin

"assert" @function.builtin
"constrain" @function.builtin
"as" @keyword
"break" @keyword
"continue" @keyword
"else" @keyword
"fn" @keyword
"for" @keyword
"global" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"mod" @keyword
"struct" @keyword
"trait" @keyword
"use" @keyword
"where" @keyword
"while" @keyword

(mutable) @keyword
(comptime) @keyword
(return) @keyword
(viewer) @keyword
(crate) @keyword
(unconstrained) @keyword

"=" @operator
"==" @operator
"!=" @operator
"<" @operator
"<=" @operator
">" @operator
">=" @operator
"+" @operator
"+=" @operator
"-" @operator
"-=" @operator
"*" @operator
"/" @operator
"%" @operator
"&" @operator
"&=" @operator
"|" @operator
"|=" @operator
"^" @operator
"&&" @operator
"||" @operator
"!" @operator
".." @operator
"<<" @operator
">>" @operator
"->" @operator

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(generic
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(generic_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
