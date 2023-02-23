
output/image.elf:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start>:
 8000000:	f3bf 8f4f 	dsb	sy
 8000004:	f3bf 8f6f 	isb	sy
 8000008:	f000 f85d 	bl	80000c6 <main>
			8000008: R_ARM_THM_CALL	main
 800000c:	defe      	udf	#254	; 0xfe

0800000e <function1>:
 800000e:	b5d0      	push	{r4, r6, r7, lr}
 8000010:	af02      	add	r7, sp, #8
 8000012:	f240 0400 	movw	r4, #0
			8000012: R_ARM_THM_MOVW_ABS_NC	_ZN10component35file16F1_STR17ha6ec4355f580b9c4E.0
 8000016:	2108      	movs	r1, #8
 8000018:	f2c2 0400 	movt	r4, #8192	; 0x2000
			8000018: R_ARM_THM_MOVT_ABS	_ZN10component35file16F1_STR17ha6ec4355f580b9c4E.0
 800001c:	6820      	ldr	r0, [r4, #0]
 800001e:	f000 f83b 	bl	8000098 <print>
			800001e: R_ARM_THM_CALL	print
 8000022:	f240 1028 	movw	r0, #296	; 0x128
			8000022: R_ARM_THM_MOVW_ABS_NC	.L__unnamed_1
 8000026:	2108      	movs	r1, #8
 8000028:	f6c0 0000 	movt	r0, #2048	; 0x800
			8000028: R_ARM_THM_MOVT_ABS	.L__unnamed_1
 800002c:	6020      	str	r0, [r4, #0]
 800002e:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 8000032:	f000 b831 	b.w	8000098 <print>
			8000032: R_ARM_THM_JUMP24	print

08000036 <function2>:
 8000036:	b5d0      	push	{r4, r6, r7, lr}
 8000038:	af02      	add	r7, sp, #8
 800003a:	f240 0408 	movw	r4, #8
			800003a: R_ARM_THM_MOVW_ABS_NC	.L_MergedGlobals
 800003e:	f2c2 0400 	movt	r4, #8192	; 0x2000
			800003e: R_ARM_THM_MOVT_ABS	.L_MergedGlobals
 8000042:	e9d4 0100 	ldrd	r0, r1, [r4]
 8000046:	f000 f827 	bl	8000098 <print>
			8000046: R_ARM_THM_CALL	print
 800004a:	e9d4 0102 	ldrd	r0, r1, [r4, #8]
 800004e:	f000 f823 	bl	8000098 <print>
			800004e: R_ARM_THM_CALL	print
 8000052:	e9d4 0102 	ldrd	r0, r1, [r4, #8]
 8000056:	e9d4 2300 	ldrd	r2, r3, [r4]
 800005a:	c40f      	stmia	r4!, {r0, r1, r2, r3}
 800005c:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 8000060:	f000 b81a 	b.w	8000098 <print>
			8000060: R_ARM_THM_JUMP24	print

08000064 <function3>:
 8000064:	b5d0      	push	{r4, r6, r7, lr}
 8000066:	af02      	add	r7, sp, #8
 8000068:	f240 0408 	movw	r4, #8
			8000068: R_ARM_THM_MOVW_ABS_NC	.L_MergedGlobals
 800006c:	f2c2 0400 	movt	r4, #8192	; 0x2000
			800006c: R_ARM_THM_MOVT_ABS	.L_MergedGlobals
 8000070:	e9d4 0104 	ldrd	r0, r1, [r4, #16]
 8000074:	f000 f810 	bl	8000098 <print>
			8000074: R_ARM_THM_CALL	print
 8000078:	e9d4 0106 	ldrd	r0, r1, [r4, #24]
 800007c:	f000 f80c 	bl	8000098 <print>
			800007c: R_ARM_THM_CALL	print
 8000080:	e9d4 0106 	ldrd	r0, r1, [r4, #24]
 8000084:	f104 0c10 	add.w	ip, r4, #16
 8000088:	e9d4 2304 	ldrd	r2, r3, [r4, #16]
 800008c:	e88c 000f 	stmia.w	ip, {r0, r1, r2, r3}
 8000090:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 8000094:	f000 b800 	b.w	8000098 <print>
			8000094: R_ARM_THM_JUMP24	print

08000098 <print>:
 8000098:	b580      	push	{r7, lr}
 800009a:	466f      	mov	r7, sp
 800009c:	b088      	sub	sp, #32
 800009e:	f240 1350 	movw	r3, #336	; 0x150
			800009e: R_ARM_THM_MOVW_ABS_NC	.L__unnamed_2
 80000a2:	2200      	movs	r2, #0
 80000a4:	f6c0 0300 	movt	r3, #2048	; 0x800
			80000a4: R_ARM_THM_MOVT_ABS	.L__unnamed_2
 80000a8:	9207      	str	r2, [sp, #28]
 80000aa:	e9cd 2305 	strd	r2, r3, [sp, #20]
 80000ae:	aa02      	add	r2, sp, #8
 80000b0:	c20b      	stmia	r2!, {r0, r1, r3}
 80000b2:	f04f 1001 	mov.w	r0, #65537	; 0x10001
 80000b6:	9001      	str	r0, [sp, #4]
 80000b8:	a801      	add	r0, sp, #4
 80000ba:	f000 f824 	bl	8000106 <_ZN7userlib13sys_send_stub17h888361065a580dfcE>
			80000ba: R_ARM_THM_CALL	_ZN7userlib13sys_send_stub17h888361065a580dfcE
 80000be:	f000 f821 	bl	8000104 <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h1e460157d6ea36b0E>
			80000be: R_ARM_THM_CALL	_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h1e460157d6ea36b0E
 80000c2:	b008      	add	sp, #32
 80000c4:	bd80      	pop	{r7, pc}

080000c6 <main>:
 80000c6:	    b580      	push	{r7, lr}
 80000c8:	    466f      	mov	r7, sp
 80000ca:	/-> f000 f801 	bl	80000d0 <example7>
			80000ca: R_ARM_THM_CALL	example7
 80000ce:	\-- e7fc      	b.n	80000ca <main+0x4>

080000d0 <example7>:
 80000d0:	b5d0      	push	{r4, r6, r7, lr}
 80000d2:	af02      	add	r7, sp, #8
 80000d4:	f240 0404 	movw	r4, #4
			80000d4: R_ARM_THM_MOVW_ABS_NC	_ZN10component310TEXT_FIELD17h588157b2ed640222E.0
 80000d8:	2118      	movs	r1, #24
 80000da:	f2c2 0400 	movt	r4, #8192	; 0x2000
			80000da: R_ARM_THM_MOVT_ABS	_ZN10component310TEXT_FIELD17h588157b2ed640222E.0
 80000de:	6820      	ldr	r0, [r4, #0]
 80000e0:	f7ff ffda 	bl	8000098 <print>
			80000e0: R_ARM_THM_CALL	print
 80000e4:	f240 1068 	movw	r0, #360	; 0x168
			80000e4: R_ARM_THM_MOVW_ABS_NC	.L__unnamed_3
 80000e8:	2118      	movs	r1, #24
 80000ea:	f6c0 0000 	movt	r0, #2048	; 0x800
			80000ea: R_ARM_THM_MOVT_ABS	.L__unnamed_3
 80000ee:	6020      	str	r0, [r4, #0]
 80000f0:	f7ff ffd2 	bl	8000098 <print>
			80000f0: R_ARM_THM_CALL	print
 80000f4:	f7ff ff8b 	bl	800000e <function1>
			80000f4: R_ARM_THM_CALL	function1
 80000f8:	f7ff ff9d 	bl	8000036 <function2>
			80000f8: R_ARM_THM_CALL	function2
 80000fc:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 8000100:	f7ff bfb0 	b.w	8000064 <function3>
			8000100: R_ARM_THM_JUMP24	function3

08000104 <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h1e460157d6ea36b0E>:
 8000104:	4770      	bx	lr

08000106 <_ZN7userlib13sys_send_stub17h888361065a580dfcE>:
 8000106:	e92d 0ff0 	stmdb	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 800010a:	e890 07f0 	ldmia.w	r0, {r4, r5, r6, r7, r8, r9, sl}
 800010e:	f04f 0b00 	mov.w	fp, #0
 8000112:	df00      	svc	0
 8000114:	4620      	mov	r0, r4
 8000116:	4629      	mov	r1, r5
 8000118:	e8bd 0ff0 	ldmia.w	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 800011c:	4770      	bx	lr
 800011e:	defe      	udf	#254	; 0xfe
