# README
This documents a minimal reproducible bug in the Solang compiler. See [the bug report](https://github.com/hyperledger/solang/issues/1521).

## Running
```
$ cargo run
```
<pre>
   === Function: updateStorage() ===

Error: start: 213 >= end 212 in expression _totalSupply += 1
    left: _totalSupply += 1
    right: 1

Error: start: 240 >= end 239 in expression _totalSupply *= 1
    left: _totalSupply *= 1
    right: 1

Error: start: 267 >= end 266 in expression _totalSupply /= 1
    left: _totalSupply /= 1
    right: 1

Error: start: 294 >= end 293 in expression _totalSupply -= 1
    left: _totalSupply -= 1
    right: 1

Error: start: 321 >= end 320 in expression _totalSupply &= 1
    left: _totalSupply &= 1
    right: 1

Error: start: 348 >= end 347 in expression _totalSupply |= 1
    left: _totalSupply |= 1
    right: 1

   === Function: updateVariable() ===

Success

Success

Success

Success

Success

Success
</pre>
