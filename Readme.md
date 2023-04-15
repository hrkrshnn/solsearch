# SolSearch

## What?

Solsearch is a tool for semantically searching over Solidity code. Search queries can be written in
a custom Domain Specific Language (DSL) that closely resembles the Abstract Syntax Tree (AST)
representation that's used internally by the Solidity compiler.

## Why?

There are three different use-cases for the tool:

1. Highly customizable linters: the DSL can encode simple linting rules.
2. Highly extensible static analysis tools: the DSL can be used to write rules on the fly. Current
   static-analysis tools require working with the AST directly in a high-level language like C++,
   Rust or Python. The learning curve and feedback-loop for existing static-analysis tool is bad.
3. Security research:
   - A researcher can build their own internal set of rules, for common vulnerabilities and patterns
     that they notice, and run it againt solidity contracts.
  - When a new security bug is found, this search tool can be used to query against all verified
    contracts on chain to find out affected contracts.
4. Language designers: when existing features for solidity gets updated, the compiler team often
   needs to understand the impact of the changes. This search tool can be used to precisely located
   open-source contracts that use certain language patterns.

## What's built at ETHGlobal Tokyo?

The tool is currently an MVP that currently has the following features:

1. A simple design for the DSL
2. Can parse the DSL built in YAML.
3. The DSL currenly only supports expressions (FunctionCalls and MemberAccess)
4. The matching engine currently only supports FunctionCalls.


## DSL

### Example: linting rule for style of variables

Here's an example of a bad naming convention.
```solidity
contract C {
    // Bad convention: names that start with underscore for public state variables
    uint public _name;
}
```

This rule can be expressed by the following script:

```yaml
id: 'low-level-call'
message: 'Low level calls bad'
metadata:
  references: ['https://docs.soliditylang.org/en/latest/']
  category: 'low-level'
  tags: ['low-level']
pattern:
  nodeType: VariableDeclaration
  stateVariable: true
  # A regex rule that matches names that start with _
  name: "_*"
  visibility: public
severity: 'lint'
```

### Example: usage of `delegatecalls`

```yaml
id: 'low-level-call'
message: "Using `delegatecall` can be dangerous, as it gives control of contract's state."
metadata:
  references: ['...']
  category: 'lint'
  tags: ['low-level']
pattern:
  nodeType: Expression
  expression:
    functionCall:
      expression:
        memberAccess:
          memberName: 'delegatecall'
severity: 'lint'
```

TODO: extend to the rule to show a linting rule where it's a `delegatecall` to a contract that's supplied in a `public` / `external` function. (This would require extending the DSL to have some notion of equality between two sub-patterns).

### Example: searching for non-pure function calls

Even though this is a trivial query, finding this information manually is painful. This is an
example of a query that can be used for doing security research. This example can be queried on the MVP.

```yaml
id: 'non-pure-call'
message: "Found a call to a function that's not pure"
metadata:
  references: ['https://docs.soliditylang.org/en/latest/']
  category: 'informational'
  tags: ['informational', 'function calls']
pattern:
  nodeType: Expression
  expression:
    functionCall:
      isPure: false
severity: 'informational'
```

# Demo

```bash
cargo run -- --help
```

```bash
# Searches for function calls to functions that not-pure
# this includes, internal function, external functions, and
# functions that are part of the core-language.
Cargo run -- search test/examples/raw_call.json test/examples/NonPureCalls.yaml
# In the current form, it prints out the internal represenation of the AST node
# In the future, this can be the lines of code from the original contract
# See `src` field, which already gives the line numbers.
```

```txt
Found a match.
FunctionCall {
    kind: FunctionCall,
    try_call: Some(
        false,
    ),
    names: [],
    arguments: [
        Literal(
            Literal {
                hex_value: Some(
                    "",
                ),
                value: Some(
                    "",
                ),
                subdenomination: None,
                kind: String,
                argument_types: None,
                is_constant: false,
                is_l_value: false,
                is_pure: true,
                l_value_requested: false,
                type_descriptions: TypeDescriptions {
                    type_identifier: Some(
                        "t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
                    ),
                    type_string: Some(
                        "literal_string \"\"",
                    ),
                },
                src: "122:2:0",
                id: 8,
            },
        ),
    ],
    expression: MemberAccess(
        MemberAccess {
            member_name: "call",
            expression: Identifier(
                Identifier {
                    argument_types: None,
                    name: "a",
                    overloaded_declarations: [],
                    referenced_declaration: 2,
                    type_descriptions: TypeDescriptions {
                        type_identifier: Some(
                            "t_address",
                        ),
                        type_string: Some(
                            "address",
                        ),
                    },
                    src: "115:1:0",
                    id: 5,
                },
            ),
            referenced_declaration: None,
            argument_types: Some(
                [
                    TypeDescriptions {
                        type_identifier: Some(
                            "t_stringliteral_c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
                        ),
                        type_string: Some(
                            "literal_string \"\"",
                        ),
                    },
                ],
            ),
            is_constant: false,
            is_l_value: false,
            is_pure: false,
            l_value_requested: false,
            type_descriptions: TypeDescriptions {
                type_identifier: Some(
                    "t_function_barecall_payable$_t_bytes_memory_ptr_$returns$_t_bool_$_t_bytes_memory_ptr_$",
                ),
                type_string: Some(
                    "function (bytes memory) payable returns (bool,bytes memory)",
                ),
            },
            src: "115:6:0",
            id: 7,
        },
    ),
    argument_types: None,
    is_constant: false,
    is_l_value: false,
    is_pure: false,
    l_value_requested: false,
    type_descriptions: TypeDescriptions {
        type_identifier: Some(
            "t_tuple$_t_bool_$_t_bytes_memory_ptr_$",
        ),
        type_string: Some(
            "tuple(bool,bytes memory)",
        ),
    },
    src: "115:10:0",
    id: 9,
}
```

# Future plans

- **DSL**:
  1. Extend the DSL to fully support all nodes of the AST.
  2. Support complex search logic, that passes state between different match paths.
- **Matching Engine**:
  1. Extend the matching engine to fully support all nodes of the AST.
  2. The matching engine should display the original lines affected by solidity, not just the internal representation.
- **UX**: Work directly with frameworks like foundry. Searching for patterns should be as simple as `solsearch --pattern-folder /file/to/patterns .` inside a foundry project.
- **DB**: Easily query against all verified contracts on Etherscan / Sourcify, by building a cache of the compiled AST.
