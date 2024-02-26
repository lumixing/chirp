# intermediate representation
label:
nop
cls
ret
jmp  label
call label
se   v0,64
sne  v0,64
se   v0,v15
mov  v0,64
add  v0,64
mov  v0,v15
or   v0,v15
and  v0,v15
xor  v0,v15
add  v0,v15
sub  v0,v15
shr  v0
subn v0,v15
shl  v0
sne  v0,v15
mov  i,512
jmpr 512
rnd  v0,64
drw  v0,v15,8
skp  v0
sknp v0
mov  v0,dt
wait v0
mov  dt,v0
mov  st,v0
add  i,v0
spr  v0
bcd  v0
save v0
load v0

$sprite 0x80 0x14 0x41 0x89
