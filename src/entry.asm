    .section .text.entry
    .globl _start
_start:
    li      t1, 0
    li      t2, 4096 * 16
    mv      t0, a0
3:
    beqz    t0, 2f
    addi    t0, t0, -1
    add     t1, t1, t2
    j       3b
2:
    la      sp, boot_stack_top_0
    add     sp, sp, t1
    j       rust_main

    .globl _start_secondary
_start_secondary:
    li      t1, 0
    li      t2, 4096 * 16
    mv      t0, a0
3:
    beqz    t0, 2f
    addi    t0, t0, -1
    add     t1, t1, t2
    j       3b
2:
    la      sp, boot_stack_top_0
    add     sp, sp, t1
    j       rust_main_secondary

    .section .bss.heap
    .globl heap_base
heap_base:
    .space 4096 * 16 * 256
    .globl heap_upper_bound
heap_upper_bound:

    .section .bss.stack0
    .globl boot_stack_lower_bound_0
boot_stack_lower_bound_0:
    .space 4096 * 16
    .globl boot_stack_top_0
boot_stack_top_0:

    .section .bss.stack1
    .globl boot_stack_lower_bound_1
boot_stack_lower_bound_1:
    .space 4096 * 16
    .globl boot_stack_top_1
boot_stack_top_1:

    .section .bss.stack2
    .globl boot_stack_lower_bound_2
boot_stack_lower_bound_2:
    .space 4096 * 16
    .globl boot_stack_top_2
boot_stack_top_2:

    .section .bss.stack3
    .globl boot_stack_lower_bound_3
boot_stack_lower_bound_3:
    .space 4096 * 16
    .globl boot_stack_top_3
boot_stack_top_3:

    .section .bss.stack4
    .globl boot_stack_lower_bound_4
boot_stack_lower_bound_4:
    .space 4096 * 16
    .globl boot_stack_top_4
boot_stack_top_4:

    .section .bss.stack5
    .globl boot_stack_lower_bound_5
boot_stack_lower_bound_5:
    .space 4096 * 16
    .globl boot_stack_top_5
boot_stack_top_5:

    .section .bss.stack6
    .globl boot_stack_lower_bound_6
boot_stack_lower_bound_6:
    .space 4096 * 16
    .globl boot_stack_top_6
boot_stack_top_6:

    .section .bss.stack7
    .globl boot_stack_lower_bound_7
boot_stack_lower_bound_7:
    .space 4096 * 16
    .globl boot_stack_top_7
boot_stack_top_7: