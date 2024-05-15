# Aderyn Analysis Report

This report was generated by [Aderyn](https://github.com/Cyfrin/aderyn), a static analysis tool built by [Cyfrin](https://cyfrin.io), a blockchain security company. This report is not a substitute for manual audit or security review. It should not be relied upon for any purpose other than to assist in the identification of potential security vulnerabilities.
# Table of Contents

- [Summary](#summary)
  - [Files Summary](#files-summary)
  - [Files Details](#files-details)
  - [Issue Summary](#issue-summary)
- [High Issues](#high-issues)
  - [H-1: Using `delegatecall` in loop](#h-1-using-delegatecall-in-loop)
- [Low Issues](#low-issues)
  - [L-1: Centralization Risk for trusted owners](#l-1-centralization-risk-for-trusted-owners)
  - [L-2: `ecrecover` is susceptible to signature malleability](#l-2-ecrecover-is-susceptible-to-signature-malleability)
  - [L-3: Solidity pragma should be specific, not wide](#l-3-solidity-pragma-should-be-specific-not-wide)
  - [L-4: Missing checks for `address(0)` when assigning values to address state variables](#l-4-missing-checks-for-address0-when-assigning-values-to-address-state-variables)
  - [L-5: `public` functions not used internally could be marked `external`](#l-5-public-functions-not-used-internally-could-be-marked-external)
  - [L-6: Event is missing `indexed` fields](#l-6-event-is-missing-indexed-fields)
  - [L-7: PUSH0 is not supported by all chains](#l-7-push0-is-not-supported-by-all-chains)
  - [L-8: Modifiers invoked only once can be shoe-horned into the function](#l-8-modifiers-invoked-only-once-can-be-shoe-horned-into-the-function)
  - [L-9: Empty Block](#l-9-empty-block)
  - [L-10: Internal functions called only once can be inlined](#l-10-internal-functions-called-only-once-can-be-inlined)
  - [L-11: Contract still has TODOs](#l-11-contract-still-has-todos)
  - [L-12: Inconsistency in declaring uint256/uint (or) int256/int variables within a contract](#l-12-inconsistency-in-declaring-uint256uint-or-int256int-variables-within-a-contract)


# Summary

## Files Summary

| Key | Value |
| --- | --- |
| .sol Files | 8 |
| Total nSLOC | 154 |


## Files Details

| Filepath | nSLOC |
| --- | --- |
| Counter.sol | 20 |
| InconsistentUints.sol | 17 |
| InternalFunctions.sol | 22 |
| OnceModifierExample.sol | 8 |
| StateVariables.sol | 58 |
| inheritance/ExtendedInheritance.sol | 17 |
| inheritance/IContractInheritance.sol | 4 |
| inheritance/InheritanceBase.sol | 8 |
| **Total** | **154** |


## Issue Summary

| Category | No. of Issues |
| --- | --- |
| High | 1 |
| Low | 12 |


# High Issues

## H-1: Using `delegatecall` in loop

When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.

- Found in inheritance/ExtendedInheritance.sol [Line: 16](../tests/adhoc-sol-files/inheritance/ExtendedInheritance.sol#L16)

	```solidity
	            target.delegatecall(abi.encodeWithSignature("doSomething(uint256)", i));
	```



# Low Issues

## L-1: Centralization Risk for trusted owners

Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.

- Found in InternalFunctions.sol [Line: 12](../tests/adhoc-sol-files/InternalFunctions.sol#L12)

	```solidity
	    function setValue(uint256 _newValue) external onlyOwner {
	```



## L-2: `ecrecover` is susceptible to signature malleability

The `ecrecover` function is susceptible to signature malleability. This means that the same message can be signed in multiple ways, allowing an attacker to change the message signature without invalidating it. This can lead to unexpected behavior in smart contracts, such as the loss of funds or the ability to bypass access control. Consider using OpenZeppelin's ECDSA library instead of the built-in function.

- Found in inheritance/ExtendedInheritance.sol [Line: 21](../tests/adhoc-sol-files/inheritance/ExtendedInheritance.sol#L21)

	```solidity
	        return ecrecover(theHash, v, r, s);
	```



## L-3: Solidity pragma should be specific, not wide

Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`

- Found in Counter.sol [Line: 2](../tests/adhoc-sol-files/Counter.sol#L2)

	```solidity
	pragma solidity ^0.8.13;
	```

- Found in InconsistentUints.sol [Line: 1](../tests/adhoc-sol-files/InconsistentUints.sol#L1)

	```solidity
	pragma solidity ^0.8.24;
	```

- Found in inheritance/IContractInheritance.sol [Line: 2](../tests/adhoc-sol-files/inheritance/IContractInheritance.sol#L2)

	```solidity
	pragma solidity >=0.8.0;
	```

- Found in inheritance/InheritanceBase.sol [Line: 2](../tests/adhoc-sol-files/inheritance/InheritanceBase.sol#L2)

	```solidity
	pragma solidity ^0.8.0;
	```



## L-4: Missing checks for `address(0)` when assigning values to address state variables

Check for `address(0)` when assigning values to address state variables.

- Found in StateVariables.sol [Line: 58](../tests/adhoc-sol-files/StateVariables.sol#L58)

	```solidity
	        addr = newAddr;
	```



## L-5: `public` functions not used internally could be marked `external`

Instead of marking a function as `public`, consider marking it as `external` if it is not used internally.

- Found in Counter.sol [Line: 7](../tests/adhoc-sol-files/Counter.sol#L7)

	```solidity
	    function setNumber(uint256 newNumber) public {
	```

- Found in StateVariables.sol [Line: 47](../tests/adhoc-sol-files/StateVariables.sol#L47)

	```solidity
	    function setAddrNoZeroError(address newAddr) public {
	```

- Found in StateVariables.sol [Line: 52](../tests/adhoc-sol-files/StateVariables.sol#L52)

	```solidity
	    function setAddrNoZeroRequire(address newAddr) public {
	```

- Found in StateVariables.sol [Line: 57](../tests/adhoc-sol-files/StateVariables.sol#L57)

	```solidity
	    function setAddrNoCheck(address newAddr) public {
	```

- Found in StateVariables.sol [Line: 61](../tests/adhoc-sol-files/StateVariables.sol#L61)

	```solidity
	    function setEmptyAlteredNumbers(
	```

- Found in StateVariables.sol [Line: 71](../tests/adhoc-sol-files/StateVariables.sol#L71)

	```solidity
	    function setNonEmptyAlteredNumbers(
	```



## L-6: Event is missing `indexed` fields

Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.

- Found in inheritance/ExtendedInheritance.sol [Line: 7](../tests/adhoc-sol-files/inheritance/ExtendedInheritance.sol#L7)

	```solidity
	    event DoSomethingElse(uint256 somethingElse);
	```

- Found in inheritance/InheritanceBase.sol [Line: 7](../tests/adhoc-sol-files/inheritance/InheritanceBase.sol#L7)

	```solidity
	    event Do(uint256 something);
	```



## L-7: PUSH0 is not supported by all chains

Solc compiler version 0.8.20 switches the default target EVM version to Shanghai, which means that the generated bytecode will include PUSH0 opcodes. Be sure to select the appropriate EVM version in case you intend to deploy on a chain other than mainnet like L2 chains that may not support PUSH0, otherwise deployment of your contracts will fail.

- Found in Counter.sol [Line: 2](../tests/adhoc-sol-files/Counter.sol#L2)

	```solidity
	pragma solidity ^0.8.13;
	```

- Found in InconsistentUints.sol [Line: 1](../tests/adhoc-sol-files/InconsistentUints.sol#L1)

	```solidity
	pragma solidity ^0.8.24;
	```

- Found in StateVariables.sol [Line: 2](../tests/adhoc-sol-files/StateVariables.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

- Found in inheritance/ExtendedInheritance.sol [Line: 2](../tests/adhoc-sol-files/inheritance/ExtendedInheritance.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

- Found in inheritance/IContractInheritance.sol [Line: 2](../tests/adhoc-sol-files/inheritance/IContractInheritance.sol#L2)

	```solidity
	pragma solidity >=0.8.0;
	```

- Found in inheritance/InheritanceBase.sol [Line: 2](../tests/adhoc-sol-files/inheritance/InheritanceBase.sol#L2)

	```solidity
	pragma solidity ^0.8.0;
	```



## L-8: Modifiers invoked only once can be shoe-horned into the function



- Found in InternalFunctions.sol [Line: 18](../tests/adhoc-sol-files/InternalFunctions.sol#L18)

	```solidity
	    modifier onlyOwner() {
	```

- Found in OnceModifierExample.sol [Line: 6](../tests/adhoc-sol-files/OnceModifierExample.sol#L6)

	```solidity
	    modifier onlyOnce() {
	```



## L-9: Empty Block

Consider removing empty blocks.

- Found in OnceModifierExample.sol [Line: 10](../tests/adhoc-sol-files/OnceModifierExample.sol#L10)

	```solidity
	    function perform() external onlyOnce {
	```



## L-10: Internal functions called only once can be inlined

Instead of separating the logic into a separate function, consider inlining the logic into the calling function. This can reduce the number of function calls and improve readability.

- Found in InternalFunctions.sol [Line: 28](../tests/adhoc-sol-files/InternalFunctions.sol#L28)

	```solidity
	    function internalSet2(uint256 _newValue) internal {
	```



## L-11: Contract still has TODOs

Contract contains comments with TODOS

- Found in Counter.sol [Line: 4](../tests/adhoc-sol-files/Counter.sol#L4)

	```solidity
	contract Counter {
	```



## L-12: Inconsistency in declaring uint256/uint (or) int256/int variables within a contract

Consider keeping the naming convention consistent in a given contract

- Found in InconsistentUints.sol [Line: 3](../tests/adhoc-sol-files/InconsistentUints.sol#L3)

	```solidity
	contract InconsistentStateVariablesContract {
	```


