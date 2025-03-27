# Notes

-   When adding a 16bit register to an 8bit register,
    we take the lower 8bits of the 16bit reg then add them to
    the 8bit register.


-   | Code  | Color        |
    | ----- | ------------ |
    | 0b11  |  white       |
    | 0b10  |  dark-gray   |
    | 0b01  |  light-gray  |
    | 0b00  |  black       |

-   ROM name is stored between `0x0134` and `0x144` (15 characters bytes + 1 CGB Flag byte if used).


## Graphics

-   Tile Data is between `0x8000` and `0x97FF`. Can store 384 tiles.
-   Each tile is 16 bytes.
    -   Each tile is 8x8 pixels. And each pixel needs 2 bits. So: `tile size * pixel size = tile data size`
        or `(8 * 8) * 2 = 128b = 16B`

-   I'm aiming the DMG model of the GameBoy.


## Dumb Things I've done

-   I've ignored the PPU, the LCD Control register, the LCD Status registers,
    what a window, background and object is.