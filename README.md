# arm-hello

This is an Hello-World-OS in Rust running on the `aarch64` architecture.
This project is written purely in Rust using inline assembly for startup.

## Build and run
This project depends on 
- [Rust](https://www.rust-lang.org)
- [Xargo](https://github.com/japaric/xargo)
- [QEMU](https://www.qemu.org/)

If you have installed these, just run
```
> make run
```

## Resources
Because there is no such thing as a VGA-Buffer for this architecture, output is printed to the serial console through the port UART0, which QEMU supports through the `-nographic` or `-serial stdio` options. 
This [post][1] for ARM bare metal coding in `C` was really helpful for getting a grasp of it.

This [bootloader][2] project from Philipp Oppermann helped finding out how to embed assembly in rust. For more OS-stuff there is his awesome series [Writing an OS in Rust][2]. 

[1]: https://balau82.wordpress.com/2010/02/28/hello-world-for-bare-metal-arm-using-qemu/

[2]: https://github.com/phil-opp/bootloader

[3]: https://os.phil-opp.com/
