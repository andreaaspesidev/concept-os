
output/image.elf:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start>:
 8000000:	f3bf 8f4f 	dsb	sy
 8000004:	f3bf 8f6f 	isb	sy
 8000008:	f000 f819 	bl	800003e <main>
			8000008: R_ARM_THM_CALL	main
 800000c:	defe      	udf	#254	; 0xfe

0800000e <print>:
 800000e:	b580      	push	{r7, lr}
 8000010:	466f      	mov	r7, sp
 8000012:	b088      	sub	sp, #32
 8000014:	f240 03bc 	movw	r3, #188	; 0xbc
			8000014: R_ARM_THM_MOVW_ABS_NC	.L__unnamed_1
 8000018:	2200      	movs	r2, #0
 800001a:	f6c0 0300 	movt	r3, #2048	; 0x800
			800001a: R_ARM_THM_MOVT_ABS	.L__unnamed_1
 800001e:	9207      	str	r2, [sp, #28]
 8000020:	e9cd 2305 	strd	r2, r3, [sp, #20]
 8000024:	aa02      	add	r2, sp, #8
 8000026:	c20b      	stmia	r2!, {r0, r1, r3}
 8000028:	2001      	movs	r0, #1
 800002a:	f2c0 0004 	movt	r0, #4
 800002e:	9001      	str	r0, [sp, #4]
 8000030:	a801      	add	r0, sp, #4
 8000032:	f000 f835 	bl	80000a0 <_ZN7userlib13sys_send_stub17h4dbc88dc5110cf49E>
			8000032: R_ARM_THM_CALL	_ZN7userlib13sys_send_stub17h4dbc88dc5110cf49E
 8000036:	f000 f832 	bl	800009e <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h24635007299a7742E>
			8000036: R_ARM_THM_CALL	_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h24635007299a7742E
 800003a:	b008      	add	sp, #32
 800003c:	bd80      	pop	{r7, pc}

0800003e <main>:
 800003e:	    b580      	push	{r7, lr}
 8000040:	    466f      	mov	r7, sp
 8000042:	/-> f000 f801 	bl	8000048 <example8>
			8000042: R_ARM_THM_CALL	example8
 8000046:	\-- e7fc      	b.n	8000042 <main+0x4>

08000048 <example8>:
 8000048:	b5b0      	push	{r4, r5, r7, lr}
 800004a:	af02      	add	r7, sp, #8
 800004c:	f240 0400 	movw	r4, #0
			800004c: R_ARM_THM_MOVW_ABS_NC	.L_MergedGlobals
 8000050:	f240 00bc 	movw	r0, #188	; 0xbc
			8000050: R_ARM_THM_MOVW_ABS_NC	.L__unnamed_2
 8000054:	f2c2 0400 	movt	r4, #8192	; 0x2000
			8000054: R_ARM_THM_MOVT_ABS	.L_MergedGlobals
 8000058:	2501      	movs	r5, #1
 800005a:	f6c0 0000 	movt	r0, #2048	; 0x800
			800005a: R_ARM_THM_MOVT_ABS	.L__unnamed_2
 800005e:	7025      	strb	r5, [r4, #0]
 8000060:	60a0      	str	r0, [r4, #8]
 8000062:	7125      	strb	r5, [r4, #4]
 8000064:	f000 f80b 	bl	800007e <print_enum>
			8000064: R_ARM_THM_CALL	print_enum
 8000068:	f240 00c9 	movw	r0, #201	; 0xc9
			8000068: R_ARM_THM_MOVW_ABS_NC	.L__unnamed_3
 800006c:	7025      	strb	r5, [r4, #0]
 800006e:	f6c0 0000 	movt	r0, #2048	; 0x800
			800006e: R_ARM_THM_MOVT_ABS	.L__unnamed_3
 8000072:	7125      	strb	r5, [r4, #4]
 8000074:	60a0      	str	r0, [r4, #8]
 8000076:	e8bd 40b0 	ldmia.w	sp!, {r4, r5, r7, lr}
 800007a:	f000 b800 	b.w	800007e <print_enum>
			800007a: R_ARM_THM_JUMP24	print_enum

0800007e <print_enum>:
 800007e:	    f240 0000 	movw	r0, #0
			800007e: R_ARM_THM_MOVW_ABS_NC	.L_MergedGlobals
 8000082:	    f2c2 0000 	movt	r0, #8192	; 0x2000
			8000082: R_ARM_THM_MOVT_ABS	.L_MergedGlobals
 8000086:	    7801      	ldrb	r1, [r0, #0]
 8000088:	    2901      	cmp	r1, #1
 800008a:	/-- d106      	bne.n	800009a <print_enum+0x1c>
 800008c:	|   7901      	ldrb	r1, [r0, #4]
 800008e:	|   6880      	ldr	r0, [r0, #8]
 8000090:	|   2900      	cmp	r1, #0
 8000092:	|   bf18      	it	ne
 8000094:	|   210d      	movne	r1, #13
 8000096:	|   f7ff bfba 	b.w	800000e <print>
			8000096: R_ARM_THM_JUMP24	print
 800009a:	\-> defe      	udf	#254	; 0xfe
 800009c:	    defe      	udf	#254	; 0xfe

0800009e <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h24635007299a7742E>:
 800009e:	4770      	bx	lr

080000a0 <_ZN7userlib13sys_send_stub17h4dbc88dc5110cf49E>:
 80000a0:	e92d 0ff0 	stmdb	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 80000a4:	e890 07f0 	ldmia.w	r0, {r4, r5, r6, r7, r8, r9, sl}
 80000a8:	f04f 0b00 	mov.w	fp, #0
 80000ac:	df00      	svc	0
 80000ae:	4620      	mov	r0, r4
 80000b0:	4629      	mov	r1, r5
 80000b2:	e8bd 0ff0 	ldmia.w	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 80000b6:	4770      	bx	lr
 80000b8:	defe      	udf	#254	; 0xfe
 80000ba:	dede      	udf	#222	; 0xde
