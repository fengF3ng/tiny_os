# qemu启动问题

Unable to init server: Could not connect: Connection refused
gtk initialization failed

1. 更新wsl
    ```console
    wsl --update
    ```
2. 重启主机

# 找不到入口点

tests/basic_boot.rs

```console
process didn't exit successfully: `/root/tiny_os/target/bootimage/bootloader/release/build/bootloader-37077d276400e082/build-script-build` (exit status: 101)
  --- stderr
  thread 'main' panicked at 'Kernel executable has an empty text section. Perhaps the entry point was set incorrectly?
```

1. 没有no_mangle标签，_start名称被重整导致无法找到入口点