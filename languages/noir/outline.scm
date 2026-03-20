(module
  "mod" @context
  (identifier) @name) @item

(struct_definition
  "struct" @context
  name: (identifier) @name) @item

(trait_definition
  "trait" @context
  (identifier) @name) @item

(trait_alias
  "trait" @context
  .
  (identifier) @name) @item

(function_definition
  "fn" @context
  (identifier) @name) @item

(trait_function_signature
  "fn" @context
  (identifier) @name) @item

(global
  "global" @context
  (binary_expression
    left: (typed_identifier
      var: (identifier) @name))) @item

(global
  "global" @context
  (binary_expression
    left: (identifier) @name)) @item
