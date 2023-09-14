# README
This documents several bugs in the `solang` compiler`, corresponding to issues:
+ [1521](https://github.com/hyperledger/solang/issues/1521)
+ [1523](https://github.com/hyperledger/solang/issues/1523)
+ [1524](https://github.com/hyperledger/solang/issues/1524)
+ [1525](https://github.com/hyperledger/solang/issues/1525)
+ [1526](https://github.com/hyperledger/solang/issues/1526)

## Running
```
$ cargo run
```
<pre>
Issue 1521: https://github.com/hyperledger/solang/issues/1521

   === Function: updateStorage() ===

Error: start: 185 >= end 184 in expr _totalSupply += 1
    left: _totalSupply += 1
    right: 1
Error: start: 212 >= end 211 in expr _totalSupply *= 1
    left: _totalSupply *= 1
    right: 1
Error: start: 239 >= end 238 in expr _totalSupply /= 1
    left: _totalSupply /= 1
    right: 1
Error: start: 266 >= end 265 in expr _totalSupply -= 1
    left: _totalSupply -= 1
    right: 1
Error: start: 293 >= end 292 in expr _totalSupply &= 1
    left: _totalSupply &= 1
    right: 1
Error: start: 320 >= end 319 in expr _totalSupply |= 1
    left: _totalSupply |= 1
    right: 1
   === Function: updateVariable() ===

Success
Success
Success
Success
Success
Success

Issue 1523: https://github.com/hyperledger/solang/issues/1523

Error: issue1522.sol
Solang diagnostics:
error: -57896044618658097711785492504343953926634992332820282019728792003956564819968 is too large
  ┌─ /Users/benku/Playground/issue1523.sol:6:21
  │
6 │         return x <= type(int256).min;
  │                     ^^^^^^^^^^^^^^^^

error: -57896044618658097711785492504343953926634992332820282019728792003956564819968 is too large
   ┌─ /Users/benku/Playground/issue1523.sol:10:21
   │
10 │         return x <= type(int256).min + 1;
   │                     ^^^^^^^^^^^^^^^^

error: -57896044618658097711785492504343953926634992332820282019728792003956564819968 is too large
   ┌─ /Users/benku/Playground/issue1523.sol:15:21
   │
15 │         return x <= -57896044618658097711785492504343953926634992332820282019728792003956564819968;
   │                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


Issue 1524: https://github.com/hyperledger/solang/issues/1524

Error: issue1524.sol
Solang diagnostics:
error: tag '@custom:experimental' is not valid for contract
   ┌─ /Users/benku/Playground/issue1524.sol:13:6
   │
13 │ /// @custom:experimental This is an experimental contract.
   │      ^^^^^^^^^^^^^^^^^^^^


Issue 1525: https://github.com/hyperledger/solang/issues/1525

Error: issue1525.sol
Solang diagnostics:
error: 'f' not found
  ┌─ /Users/benku/Playground/issue1525.sol:5:12
  │
5 │     using {f} for int256;
  │            ^


Issue 1525: https://github.com/hyperledger/solang/issues/1526

Error: issue1526.sol
Solang diagnostics:
error: flag 'memory-safe' not supported
  ┌─ /Users/benku/Playground/issue1526.sol:6:19
  │
6 │         assembly ("memory-safe") {
  │                   ^^^^^^^^^^^^^

</pre>
