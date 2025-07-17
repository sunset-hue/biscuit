.text
.globl setGdt


gdtr DW 0;
     DQ 0;
;; this is from the gdt tutorial, will fix if bugs

setGdt:
     mov [gdtr], DI
     mov [gdtr+2], RSI
     lgdt [gdtr]
     ret

