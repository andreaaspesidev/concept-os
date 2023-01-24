
output/image.elf:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start>:
 8000000:	f3bf 8f4f 	dsb	sy
 8000004:	f3bf 8f6f 	isb	sy
 8000008:	f000 f86c 	bl	80000e4 <main>
 800000c:	defe      	udf	#254	; 0xfe

0800000e <function1>:
 800000e:	b5d0      	push	{r4, r6, r7, lr}
 8000010:	af02      	add	r7, sp, #8
 8000012:	f240 0400 	movw	r4, #0
 8000016:	2108      	movs	r1, #8
 8000018:	f2c0 0400 	movt	r4, #0
 800001c:	f859 0004 	ldr.w	r0, [r9, r4]
 8000020:	f000 f847 	bl	80000b2 <print>
 8000024:	f240 101a 	movw	r0, #282	; 0x11a
 8000028:	2108      	movs	r1, #8
 800002a:	f2c0 0000 	movt	r0, #0
 800002e:	4478      	add	r0, pc
 8000030:	f849 0004 	str.w	r0, [r9, r4]
 8000034:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 8000038:	f000 b83b 	b.w	80000b2 <print>

0800003c <function2>:
 800003c:	b5b0      	push	{r4, r5, r7, lr}
 800003e:	af02      	add	r7, sp, #8
 8000040:	f240 0408 	movw	r4, #8
 8000044:	f2c0 0400 	movt	r4, #0
 8000048:	eb09 0504 	add.w	r5, r9, r4
 800004c:	f859 0004 	ldr.w	r0, [r9, r4]
 8000050:	6869      	ldr	r1, [r5, #4]
 8000052:	f000 f82e 	bl	80000b2 <print>
 8000056:	e9d5 0102 	ldrd	r0, r1, [r5, #8]
 800005a:	f000 f82a 	bl	80000b2 <print>
 800005e:	f859 3004 	ldr.w	r3, [r9, r4]
 8000062:	e9d5 2001 	ldrd	r2, r0, [r5, #4]
 8000066:	68e9      	ldr	r1, [r5, #12]
 8000068:	f849 0004 	str.w	r0, [r9, r4]
 800006c:	e9c5 1301 	strd	r1, r3, [r5, #4]
 8000070:	60ea      	str	r2, [r5, #12]
 8000072:	e8bd 40b0 	ldmia.w	sp!, {r4, r5, r7, lr}
 8000076:	f000 b81c 	b.w	80000b2 <print>

0800007a <function3>:
 800007a:	b5d0      	push	{r4, r6, r7, lr}
 800007c:	af02      	add	r7, sp, #8
 800007e:	f240 0008 	movw	r0, #8
 8000082:	f2c0 0000 	movt	r0, #0
 8000086:	eb09 0400 	add.w	r4, r9, r0
 800008a:	e9d4 0104 	ldrd	r0, r1, [r4, #16]
 800008e:	f000 f810 	bl	80000b2 <print>
 8000092:	e9d4 0106 	ldrd	r0, r1, [r4, #24]
 8000096:	f000 f80c 	bl	80000b2 <print>
 800009a:	e9d4 0106 	ldrd	r0, r1, [r4, #24]
 800009e:	f104 0c10 	add.w	ip, r4, #16
 80000a2:	e9d4 2304 	ldrd	r2, r3, [r4, #16]
 80000a6:	e88c 000f 	stmia.w	ip, {r0, r1, r2, r3}
 80000aa:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 80000ae:	f000 b800 	b.w	80000b2 <print>

080000b2 <print>:
 80000b2:	b580      	push	{r7, lr}
 80000b4:	466f      	mov	r7, sp
 80000b6:	b088      	sub	sp, #32
 80000b8:	f240 02aa 	movw	r2, #170	; 0xaa
 80000bc:	f10d 0c08 	add.w	ip, sp, #8
 80000c0:	f2c0 0200 	movt	r2, #0
 80000c4:	2300      	movs	r3, #0
 80000c6:	447a      	add	r2, pc
 80000c8:	e88c 000f 	stmia.w	ip, {r0, r1, r2, r3}
 80000cc:	f04f 1001 	mov.w	r0, #65537	; 0x10001
 80000d0:	9001      	str	r0, [sp, #4]
 80000d2:	a801      	add	r0, sp, #4
 80000d4:	e9cd 2306 	strd	r2, r3, [sp, #24]
 80000d8:	f000 f827 	bl	800012a <_ZN7userlib13sys_send_stub17h2444658aabf553a4E>
 80000dc:	f000 f824 	bl	8000128 <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h02b830a50f92d613E>
 80000e0:	b008      	add	sp, #32
 80000e2:	bd80      	pop	{r7, pc}

080000e4 <main>:
 80000e4:	    b580      	push	{r7, lr}
 80000e6:	    466f      	mov	r7, sp
 80000e8:	/-> f000 f801 	bl	80000ee <example7>
 80000ec:	\-- e7fc      	b.n	80000e8 <main+0x4>

080000ee <example7>:
 80000ee:	b5d0      	push	{r4, r6, r7, lr}
 80000f0:	af02      	add	r7, sp, #8
 80000f2:	f240 0404 	movw	r4, #4
 80000f6:	2118      	movs	r1, #24
 80000f8:	f2c0 0400 	movt	r4, #0
 80000fc:	f859 0004 	ldr.w	r0, [r9, r4]
 8000100:	f7ff ffd7 	bl	80000b2 <print>
 8000104:	f240 007a 	movw	r0, #122	; 0x7a
 8000108:	2118      	movs	r1, #24
 800010a:	f2c0 0000 	movt	r0, #0
 800010e:	4478      	add	r0, pc
 8000110:	f849 0004 	str.w	r0, [r9, r4]
 8000114:	f7ff ffcd 	bl	80000b2 <print>
 8000118:	f7ff ff79 	bl	800000e <function1>
 800011c:	f7ff ff8e 	bl	800003c <function2>
 8000120:	e8bd 40d0 	ldmia.w	sp!, {r4, r6, r7, lr}
 8000124:	f7ff bfa9 	b.w	800007a <function3>

08000128 <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h02b830a50f92d613E>:
 8000128:	4770      	bx	lr

0800012a <_ZN7userlib13sys_send_stub17h2444658aabf553a4E>:
 800012a:	e92d 0ff0 	stmdb	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 800012e:	e890 07f0 	ldmia.w	r0, {r4, r5, r6, r7, r8, r9, sl}
 8000132:	f04f 0b00 	mov.w	fp, #0
 8000136:	df00      	svc	0
 8000138:	4620      	mov	r0, r4
 800013a:	4629      	mov	r1, r5
 800013c:	e8bd 0ff0 	ldmia.w	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 8000140:	4770      	bx	lr
 8000142:	defe      	udf	#254	; 0xfe
