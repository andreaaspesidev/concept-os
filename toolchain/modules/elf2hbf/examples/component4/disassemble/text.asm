
output/image.elf:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start>:
 8000000:	f3bf 8f4f 	dsb	sy
 8000004:	f3bf 8f6f 	isb	sy
 8000008:	f000 f81b 	bl	8000042 <main>
 800000c:	defe      	udf	#254	; 0xfe

0800000e <print>:
 800000e:	b580      	push	{r7, lr}
 8000010:	466f      	mov	r7, sp
 8000012:	b088      	sub	sp, #32
 8000014:	f240 02b2 	movw	r2, #178	; 0xb2
 8000018:	f10d 0c08 	add.w	ip, sp, #8
 800001c:	f2c0 0200 	movt	r2, #0
 8000020:	2300      	movs	r3, #0
 8000022:	447a      	add	r2, pc
 8000024:	e88c 000f 	stmia.w	ip, {r0, r1, r2, r3}
 8000028:	2001      	movs	r0, #1
 800002a:	f2c0 0004 	movt	r0, #4
 800002e:	e9cd 2306 	strd	r2, r3, [sp, #24]
 8000032:	9001      	str	r0, [sp, #4]
 8000034:	a801      	add	r0, sp, #4
 8000036:	f000 f841 	bl	80000bc <_ZN7userlib13sys_send_stub17h2444658aabf553a4E>
 800003a:	f000 f83e 	bl	80000ba <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h02b830a50f92d613E>
 800003e:	b008      	add	sp, #32
 8000040:	bd80      	pop	{r7, pc}

08000042 <main>:
 8000042:	    b580      	push	{r7, lr}
 8000044:	    466f      	mov	r7, sp
 8000046:	/-> f000 f801 	bl	800004c <example8>
 800004a:	\-- e7fc      	b.n	8000046 <main+0x4>

0800004c <example8>:
 800004c:	b5f0      	push	{r4, r5, r6, r7, lr}
 800004e:	af03      	add	r7, sp, #12
 8000050:	f84d bd04 	str.w	fp, [sp, #-4]!
 8000054:	f240 0400 	movw	r4, #0
 8000058:	f240 006a 	movw	r0, #106	; 0x6a
 800005c:	f2c0 0400 	movt	r4, #0
 8000060:	eb09 0604 	add.w	r6, r9, r4
 8000064:	f2c0 0000 	movt	r0, #0
 8000068:	2501      	movs	r5, #1
 800006a:	4478      	add	r0, pc
 800006c:	f809 5004 	strb.w	r5, [r9, r4]
 8000070:	60b0      	str	r0, [r6, #8]
 8000072:	7135      	strb	r5, [r6, #4]
 8000074:	f000 f80f 	bl	8000096 <print_enum>
 8000078:	f240 005d 	movw	r0, #93	; 0x5d
 800007c:	f2c0 0000 	movt	r0, #0
 8000080:	f809 5004 	strb.w	r5, [r9, r4]
 8000084:	4478      	add	r0, pc
 8000086:	60b0      	str	r0, [r6, #8]
 8000088:	7135      	strb	r5, [r6, #4]
 800008a:	f85d bb04 	ldr.w	fp, [sp], #4
 800008e:	e8bd 40f0 	ldmia.w	sp!, {r4, r5, r6, r7, lr}
 8000092:	f000 b800 	b.w	8000096 <print_enum>

08000096 <print_enum>:
 8000096:	    f240 0000 	movw	r0, #0
 800009a:	    f2c0 0000 	movt	r0, #0
 800009e:	    f819 1000 	ldrb.w	r1, [r9, r0]
 80000a2:	    2901      	cmp	r1, #1
 80000a4:	/-- d107      	bne.n	80000b6 <print_enum+0x20>
 80000a6:	|   4448      	add	r0, r9
 80000a8:	|   7901      	ldrb	r1, [r0, #4]
 80000aa:	|   6880      	ldr	r0, [r0, #8]
 80000ac:	|   2900      	cmp	r1, #0
 80000ae:	|   bf18      	it	ne
 80000b0:	|   210d      	movne	r1, #13
 80000b2:	|   f7ff bfac 	b.w	800000e <print>
 80000b6:	\-> defe      	udf	#254	; 0xfe
 80000b8:	    defe      	udf	#254	; 0xfe

080000ba <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h02b830a50f92d613E>:
 80000ba:	4770      	bx	lr

080000bc <_ZN7userlib13sys_send_stub17h2444658aabf553a4E>:
 80000bc:	e92d 0ff0 	stmdb	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 80000c0:	e890 07f0 	ldmia.w	r0, {r4, r5, r6, r7, r8, r9, sl}
 80000c4:	f04f 0b00 	mov.w	fp, #0
 80000c8:	df00      	svc	0
 80000ca:	4620      	mov	r0, r4
 80000cc:	4629      	mov	r1, r5
 80000ce:	e8bd 0ff0 	ldmia.w	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 80000d2:	4770      	bx	lr
 80000d4:	defe      	udf	#254	; 0xfe
 80000d6:	dede      	udf	#222	; 0xde
