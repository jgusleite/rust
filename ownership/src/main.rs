fn main() {
	println!("Hello, world\n");
}

/*
    Ownership Rules

    - Each value in Rust has an *owner*.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.

    String type

    - The memory must be requested from the memory allocator at runtime.
    - We need a way of returning this memory to the allocator when we're done
      with our String.
 */