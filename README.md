# fast_deltas
A faster deltas library with a buffered read and write + smaller patches

```rust
    let source: u8 = 255;
    let target: u8 = 1;
    let patch = target.wrapping_sub(source);
    let new_target = source.wrapping_add(patch);
    
    /// SPECIAL: CHECK NEXT BYTE.   
    /// If the next byte IS a 0 it is a copy instruction.
    /// If the next byte IS NOT a 0 it is a remove instruction.
    const INSTRUC_BYTE: u8 = 0;

    //vec![0, 3, 0, 0, 0, 0, 1, 1, 1, 5, 3, 3, 3, 3, 3];
    let patch = vec![INSTRUC_BYTE, 3, INSTRUC_BYTE, 0, 0, 0, 1, 1, 1, add_instruction_len, 3, 3, 3, 3, 3];
```