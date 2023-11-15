# fast_deltas
A faster deltas library with a buffered read and write + smaller patches

```rust
    let source: u8 = 255;
    let target: u8 = 1;
    let patch = target.wrapping_sub(source);
    let new_target = source.wrapping_add(patch);
    
    let source = vec![0, 0, 0, 1, 1, 1];
    let target = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
    
    let v1_patch = vec![-, 3, |, 3, 0, 0, 0, + 6, 2, 2, 2, 3, 3, 3, 3];
    
    let v2_patch = vec![-, 3, |, 6, 0, 0, 0, 1, 1, 1, +, 3, 3, 3, 3, 3];
    
    let v3_patch = vec![-, 3, 6, 0, 0, 0, 1, 1, 1, 3, 3, 3, 3, 3]; //IMPOSSIBLE!!

    /// SPECIAL: CHECK NEXT BYTE.   
    /// If the next byte IS a 0 it is a copy instruction.
    /// If the next byte IS NOT a 0 it is a remove instruction.
    const INSTRUC_BYTE: u8 = 0;

    //vec![0, 3, 0, 0, 0, 0, 1, 1, 1, 5, 3, 3, 3, 3, 3];
    let v4_patch = vec![INSTRUC_BYTE, 3, INSTRUC_BYTE, 0, 0, 0, 1, 1, 1, add_instruction_len, 3, 3, 3, 3, 3];
    
```

```rust    
    let source = vec![1, 1, 1, 2, 2, 2,];
    let target = vec![1, 1, 1];
    
    let patch = vec![6, 0, 0, 0, 254, 254, 254];

    let constructed_target = vec![1, 1, 1, 2, 2, 2];
```