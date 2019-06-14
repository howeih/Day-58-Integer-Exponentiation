Day 58: Integer Exponentiation
<br>
When computing x^y, base x is repetitively squared to get powers x¹, x², x⁴, x⁸, … Since exponent y is an integer, it can be expressed as binary number which tells us what powers have to be multiplied.
<br>
For example, x⁹⁹ = x¹x²x³²x⁶⁴.
<br>
We are usually limited to modular arithmetic or fixed point arithmetic, so we are allowed to say that the algorithm runs in O(log y) time.
<br>
run:
```

fn main() {
    let z = power(2, 100);
    let result = BigInt::parse_bytes(b"1267650600228229401496703205376", 10).unwrap();
    assert_eq!(z, result);
}


```