contract C {
    // Bad convention:
    // names that start with underscore for public state variables
    // The linting rule `StateVariableLint.yaml' would be able to encode this
    uint public _name;
}
