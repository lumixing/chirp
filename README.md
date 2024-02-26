## chirp: chip-8 intermediate representation
written in rust using plex  
used by cr8

## usage
```console
$ cargo run --release -- /path/to/input /path/to/output.ch8
```

## language manual
comments are single-lined starting with `;`

### types
integers (n, nn, nnn) can be decimal or hexadecimal or binary  
registers (vx, vy) are decimal **only** (v0 thru v15, NOT vf)  
special registers:
- `i`: i register
- `dt`: delay timer
- `st`: sound timer

### labels
labels can be accessed even before declaration
```armasm
main:           ; declare label
cls
drw  v0,v1,5
jmp  main       ; jump to label
```

### sprites
sprites **must** be declared before access
```armasm
$player 0x13 0x83 0xad 0xa1 0xd3    ; declare sprite
mov  i,player                       ; set i to sprite location
drw  v0,v1,5                        ; draw sprite
```

both labels and sprites names must match `[a-zA-Z_]+` (lowercase, uppercase letters and underscores)

### instructions
| chirp                           | binary |
| ------------------------------- | ------ |
| `nop`                           | `0000` |
| `cls`                           | `00e0` |
| `ret`                           | `00ee` |
| `jmp  nnn`<br>`jmp  label`      | `1nnn` |
| `call nnn`<br>`call label`      | `2nnn` |
| `se   vx,nn`                    | `3xnn` |
| `sne  vx,nn`                    | `4xnn` |
| `se   vx,vy`                    | `5xy0` |
| `mov  vx,nn`                    | `6xnn` |
| `add  vx,nn`                    | `7xnn` |
| `mov  vx,vy`                    | `8xy0` |
| `or   vx,vy`                    | `8xy1` |
| `and  vx,vy`                    | `8xy2` |
| `xor  vx,vy`                    | `8xy3` |
| `add  vx,vy`                    | `8xy4` |
| `sub  vx,vy`                    | `8xy5` |
| `shr  vx`                       | `8xy6` |
| `subn vx,vy`                    | `8xy7` |
| `shl  vx`                       | `8xye` |
| `sne  vx,vy`                    | `9xy0` |
| `mov  i,nnn`<br>`mov  i,sprite` | `annn` |
| `jmpr nnn`                      | `bnnn` |
| `rnd  vx,nn`                    | `cxnn` |
| `drw  vx,vy,n`                  | `dxyn` |
| `skp  vx`                       | `ex9e` |
| `sknp vx`                       | `exa1` |
| `mov  vx,dt`                    | `fx07` |
| `wait vx`                       | `fx0a` |
| `mov  dt,vx`                    | `fx15` |
| `mov  st,vx`                    | `fx18` |
| `add  i,vx`                     | `fx1e` |
| `spr  vx`                       | `fx29` |
| `bcd  vx`                       | `fx33` |
| `save vx`                       | `fx55` |
| `load vx`                       | `fx65` |
