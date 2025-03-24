# TODO
-   Make a tool to view bytes as instruction names. EXAMPLE: Instead of displaying `0x04`, display `INC B`.
-   Using [this](https://gbdev.io/pandocs/CPU_Instruction_Set.html#cpu-instruction-set),
    make instructions handling in `instructions.rs` more, elegant and smart. Mathematical bit manipulation
    and all could make it easier.
-   The VRAM is independent from the memory bus. It is rather dependent 
    on the GPU's struct defined variable `vram`. I must put all the memory in one, single array of bytes.
    *(65,535 bytes to be exact :D)*.