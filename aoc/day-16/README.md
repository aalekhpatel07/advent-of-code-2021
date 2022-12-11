## Notes


#### BITS Packet

```
packet {
    packets
}...000

packet := {
    header,

}

header := {
    version(3) packet_type_id(3)
}
```

There are two types of packets := `Literal` and `Operator`.

If `packet_type_id` is `4`, what follows is a `Literal` encoded as:
 
> Literal value packets encode a single binary number. To do this, the binary number is padded with leading zeroes until its length is a multiple of four bits, and then it is broken into groups of four bits. Each group is prefixed by a `1` bit except the last group, which is prefixed by a `0` bit. These groups of five bits immediately follow the packet header. 

For example, the hexadecimal string `D2FE28` becomes:
```
110100101111111000101000
VVVTTTAAAAABBBBBCCCCC
```

Otherwise, what follows is an `Operator` that performs calculation on one or more sub-packets contained within.

An `Operator` packet contains one or more packets.