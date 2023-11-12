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
    
    let v3_patch = vec![-, 3, 6, 0, 0, 0, 1, 1, 1, 3, 3, 3, 3, 3];
```