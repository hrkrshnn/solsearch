// The AST-JSON can be found from solc by
// `solc File --ast-compact-json`
contract C {
    function f(address a) external {
        // Example of a solidity expression that can
        // be semantically searched.
        //
        // The DSL-search for `NonPureCalls.yaml` will find this.
        // Also can be encoded using `LowLevelCalls.yaml'
        a.call("");
    }
}
