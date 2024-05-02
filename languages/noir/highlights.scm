; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; types
(generic_type) @type
(single_type) @type.builtin
(array_type) @type


;; functions
(function_definition
    (identifier) @function)
(function_call
  (identifier) @function)


;; let
(let_declaration
    (identifier) @variable)


;; structs
(struct_definition
    (identifier) @type)

;; imports
(import
    (import_identifier) @include)

;; comments
(comment) @comment

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(generic
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(generic
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

"as" @keyword
"global" @keyword
"else" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"mod" @keyword
"struct" @keyword
"use" @keyword

(mutable) @keyword
(viewer) @keyword
(return) @keyword

(self) @variable.builtin

(character) @string
(string_literal) @string

(boolean) @constant.builtin
(integer) @constant.builtin
(float) @constant.builtin


"*" @operator
"&" @operator
