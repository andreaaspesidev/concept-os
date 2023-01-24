
output/image.elf:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start>:
 8000000:	f3bf 8f4f 	dsb	sy
 8000004:	f3bf 8f6f 	isb	sy
 8000008:	f000 f81a 	bl	8000040 <main>
 800000c:	defe      	udf	#254	; 0xfe

0800000e <print>:
 800000e:	b580      	push	{r7, lr}
 8000010:	466f      	mov	r7, sp
 8000012:	b088      	sub	sp, #32
 8000014:	f241 0276 	movw	r2, #4214	; 0x1076
 8000018:	f10d 0c08 	add.w	ip, sp, #8
 800001c:	f2c0 0200 	movt	r2, #0
 8000020:	2300      	movs	r3, #0
 8000022:	447a      	add	r2, pc
 8000024:	e88c 000f 	stmia.w	ip, {r0, r1, r2, r3}
 8000028:	f04f 1001 	mov.w	r0, #65537	; 0x10001
 800002c:	9001      	str	r0, [sp, #4]
 800002e:	a801      	add	r0, sp, #4
 8000030:	e9cd 2306 	strd	r2, r3, [sp, #24]
 8000034:	f000 fafc 	bl	8000630 <_ZN7userlib13sys_send_stub17h2444658aabf553a4E>
 8000038:	f000 faf9 	bl	800062e <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h02b830a50f92d613E>
 800003c:	b008      	add	sp, #32
 800003e:	bd80      	pop	{r7, pc}

08000040 <main>:
 8000040:	    b580      	push	{r7, lr}
 8000042:	    466f      	mov	r7, sp
 8000044:	/-> f000 f935 	bl	80002b2 <example12>
 8000048:	\-- e7fc      	b.n	8000044 <main+0x4>

0800004a <_ZN55_$LT$component2..Foo$u20$as$u20$component2..Frobber$GT$4frob17h0caa40aacf5bd97eE>:
 800004a:	       b5f0      	push	{r4, r5, r6, r7, lr}
 800004c:	       af03      	add	r7, sp, #12
 800004e:	       f84d bd04 	str.w	fp, [sp, #-4]!
 8000052:	       b088      	sub	sp, #32
 8000054:	       4604      	mov	r4, r0
 8000056:	       f241 003e 	movw	r0, #4158	; 0x103e
 800005a:	       f2c0 0000 	movt	r0, #0
 800005e:	       4615      	mov	r5, r2
 8000060:	       460e      	mov	r6, r1
 8000062:	       f640 61ad 	movw	r1, #3757	; 0xead
 8000066:	       4478      	add	r0, pc
 8000068:	       2202      	movs	r2, #2
 800006a:	       f2c0 0100 	movt	r1, #0
 800006e:	       e9cd 0202 	strd	r0, r2, [sp, #8]
 8000072:	       2001      	movs	r0, #1
 8000074:	       4479      	add	r1, pc
 8000076:	       9005      	str	r0, [sp, #20]
 8000078:	       a806      	add	r0, sp, #24
 800007a:	       9004      	str	r0, [sp, #16]
 800007c:	       2000      	movs	r0, #0
 800007e:	       9000      	str	r0, [sp, #0]
 8000080:	       4668      	mov	r0, sp
 8000082:	       e9cd 6106 	strd	r6, r1, [sp, #24]
 8000086:	       f000 fa8d 	bl	80005a4 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE>
 800008a:	       f241 0020 	movw	r0, #4128	; 0x1020
 800008e:	       210a      	movs	r1, #10
 8000090:	       f2c0 0000 	movt	r0, #0
 8000094:	       4478      	add	r0, pc
 8000096:	       f7ff ffba 	bl	800000e <print>
 800009a:	       206f      	movs	r0, #111	; 0x6f
 800009c:	       f244 615b 	movw	r1, #18011	; 0x465b
 80000a0:	       71a0      	strb	r0, [r4, #6]
 80000a2:	       6830      	ldr	r0, [r6, #0]
 80000a4:	       80a1      	strh	r1, [r4, #4]
 80000a6:	       4428      	add	r0, r5
 80000a8:	       280a      	cmp	r0, #10
 80000aa:	   /-- d10a      	bne.n	80000c2 <_ZN55_$LT$component2..Foo$u20$as$u20$component2..Frobber$GT$4frob17h0caa40aacf5bd97eE+0x78>
 80000ac:	   |   f642 606f 	movw	r0, #11887	; 0x2e6f
 80000b0:	   |   210b      	movs	r1, #11
 80000b2:	   |   f2c3 0031 	movt	r0, #12337	; 0x3031
 80000b6:	   |   f8c4 0007 	str.w	r0, [r4, #7]
 80000ba:	   |   2003      	movs	r0, #3
 80000bc:	   |   6020      	str	r0, [r4, #0]
 80000be:	   |   2008      	movs	r0, #8
 80000c0:	/--|-- e00d      	b.n	80000de <_ZN55_$LT$component2..Foo$u20$as$u20$component2..Frobber$GT$4frob17h0caa40aacf5bd97eE+0x94>
 80000c2:	|  \-> f646 4065 	movw	r0, #27749	; 0x6c65
 80000c6:	|      210d      	movs	r1, #13
 80000c8:	|      f2c6 5073 	movt	r0, #25971	; 0x6573
 80000cc:	|      f8c4 0009 	str.w	r0, [r4, #9]
 80000d0:	|      2005      	movs	r0, #5
 80000d2:	|      6020      	str	r0, [r4, #0]
 80000d4:	|      f642 606f 	movw	r0, #11887	; 0x2e6f
 80000d8:	|      f8a4 0007 	strh.w	r0, [r4, #7]
 80000dc:	|      200a      	movs	r0, #10
 80000de:	\----> 225d      	movs	r2, #93	; 0x5d
 80000e0:	       6020      	str	r0, [r4, #0]
 80000e2:	       5462      	strb	r2, [r4, r1]
 80000e4:	       b008      	add	sp, #32
 80000e6:	       f85d bb04 	ldr.w	fp, [sp], #4
 80000ea:	       bdf0      	pop	{r4, r5, r6, r7, pc}

080000ec <_ZN55_$LT$component2..Bar$u20$as$u20$component2..Frobber$GT$4frob17h98e5dfcfc2ed7f3aE>:
 80000ec:	    b5f0      	push	{r4, r5, r6, r7, lr}
 80000ee:	    af03      	add	r7, sp, #12
 80000f0:	    f84d bd04 	str.w	fp, [sp, #-4]!
 80000f4:	    b088      	sub	sp, #32
 80000f6:	    4604      	mov	r4, r0
 80000f8:	    f640 70d4 	movw	r0, #4052	; 0xfd4
 80000fc:	    f2c0 0000 	movt	r0, #0
 8000100:	    4615      	mov	r5, r2
 8000102:	    460e      	mov	r6, r1
 8000104:	    f640 41ff 	movw	r1, #3327	; 0xcff
 8000108:	    4478      	add	r0, pc
 800010a:	    2202      	movs	r2, #2
 800010c:	    f2c0 0100 	movt	r1, #0
 8000110:	    e9cd 0202 	strd	r0, r2, [sp, #8]
 8000114:	    2001      	movs	r0, #1
 8000116:	    4479      	add	r1, pc
 8000118:	    9005      	str	r0, [sp, #20]
 800011a:	    a806      	add	r0, sp, #24
 800011c:	    9004      	str	r0, [sp, #16]
 800011e:	    2000      	movs	r0, #0
 8000120:	    9000      	str	r0, [sp, #0]
 8000122:	    4668      	mov	r0, sp
 8000124:	    e9cd 6106 	strd	r6, r1, [sp, #24]
 8000128:	    f000 fa3c 	bl	80005a4 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE>
 800012c:	    f640 70b6 	movw	r0, #4022	; 0xfb6
 8000130:	    210a      	movs	r1, #10
 8000132:	    f2c0 0000 	movt	r0, #0
 8000136:	    4478      	add	r0, pc
 8000138:	    f7ff ff69 	bl	800000e <print>
 800013c:	    6830      	ldr	r0, [r6, #0]
 800013e:	    f244 215b 	movw	r1, #16987	; 0x425b
 8000142:	    f2c7 2161 	movt	r1, #29281	; 0x7261
 8000146:	    4428      	add	r0, r5
 8000148:	    6061      	str	r1, [r4, #4]
 800014a:	    300a      	adds	r0, #10
 800014c:	/-- d10e      	bne.n	800016c <_ZN55_$LT$component2..Bar$u20$as$u20$component2..Frobber$GT$4frob17h98e5dfcfc2ed7f3aE+0x80>
 800014e:	|   205d      	movs	r0, #93	; 0x5d
 8000150:	|   7320      	strb	r0, [r4, #12]
 8000152:	|   f642 502e 	movw	r0, #11566	; 0x2d2e
 8000156:	|   f2c3 0031 	movt	r0, #12337	; 0x3031
 800015a:	|   60a0      	str	r0, [r4, #8]
 800015c:	|   2004      	movs	r0, #4
 800015e:	|   6020      	str	r0, [r4, #0]
 8000160:	|   2009      	movs	r0, #9
 8000162:	|   6020      	str	r0, [r4, #0]
 8000164:	|   b008      	add	sp, #32
 8000166:	|   f85d bb04 	ldr.w	fp, [sp], #4
 800016a:	|   bdf0      	pop	{r4, r5, r6, r7, pc}
 800016c:	\-> f645 5065 	movw	r0, #23909	; 0x5d65
 8000170:	    81a0      	strh	r0, [r4, #12]
 8000172:	    2005      	movs	r0, #5
 8000174:	    6020      	str	r0, [r4, #0]
 8000176:	    f246 502e 	movw	r0, #25902	; 0x652e
 800017a:	    f2c7 306c 	movt	r0, #29548	; 0x736c
 800017e:	    60a0      	str	r0, [r4, #8]
 8000180:	    200a      	movs	r0, #10
 8000182:	    6020      	str	r0, [r4, #0]
 8000184:	    b008      	add	sp, #32
 8000186:	    f85d bb04 	ldr.w	fp, [sp], #4
 800018a:	    bdf0      	pop	{r4, r5, r6, r7, pc}

0800018c <_ZN10component214frob_it_static17h1223580fb40151bbE>:
 800018c:	       b5b0      	push	{r4, r5, r7, lr}
 800018e:	       af02      	add	r7, sp, #8
 8000190:	       b08a      	sub	sp, #40	; 0x28
 8000192:	       4604      	mov	r4, r0
 8000194:	       f640 4073 	movw	r0, #3187	; 0xc73
 8000198:	       f2c0 0000 	movt	r0, #0
 800019c:	       4615      	mov	r5, r2
 800019e:	       f640 7230 	movw	r2, #3888	; 0xf30
 80001a2:	       4478      	add	r0, pc
 80001a4:	       f2c0 0200 	movt	r2, #0
 80001a8:	       9101      	str	r1, [sp, #4]
 80001aa:	       2102      	movs	r1, #2
 80001ac:	       447a      	add	r2, pc
 80001ae:	       e9cd 2104 	strd	r2, r1, [sp, #16]
 80001b2:	       2101      	movs	r1, #1
 80001b4:	       9009      	str	r0, [sp, #36]	; 0x24
 80001b6:	       a801      	add	r0, sp, #4
 80001b8:	       9107      	str	r1, [sp, #28]
 80001ba:	       a908      	add	r1, sp, #32
 80001bc:	       9008      	str	r0, [sp, #32]
 80001be:	       a802      	add	r0, sp, #8
 80001c0:	       9106      	str	r1, [sp, #24]
 80001c2:	       2100      	movs	r1, #0
 80001c4:	       9102      	str	r1, [sp, #8]
 80001c6:	       f000 f9ed 	bl	80005a4 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE>
 80001ca:	       f640 7018 	movw	r0, #3864	; 0xf18
 80001ce:	       210a      	movs	r1, #10
 80001d0:	       f2c0 0000 	movt	r0, #0
 80001d4:	       4478      	add	r0, pc
 80001d6:	       f7ff ff1a 	bl	800000e <print>
 80001da:	       9801      	ldr	r0, [sp, #4]
 80001dc:	       f244 215b 	movw	r1, #16987	; 0x425b
 80001e0:	       f2c7 2161 	movt	r1, #29281	; 0x7261
 80001e4:	       225d      	movs	r2, #93	; 0x5d
 80001e6:	       4428      	add	r0, r5
 80001e8:	       6061      	str	r1, [r4, #4]
 80001ea:	       300a      	adds	r0, #10
 80001ec:	   /-- d104      	bne.n	80001f8 <_ZN10component214frob_it_static17h1223580fb40151bbE+0x6c>
 80001ee:	   |   2009      	movs	r0, #9
 80001f0:	   |   2130      	movs	r1, #48	; 0x30
 80001f2:	   |   2331      	movs	r3, #49	; 0x31
 80001f4:	   |   252d      	movs	r5, #45	; 0x2d
 80001f6:	/--|-- e005      	b.n	8000204 <_ZN10component214frob_it_static17h1223580fb40151bbE+0x78>
 80001f8:	|  \-> 7362      	strb	r2, [r4, #13]
 80001fa:	|      200a      	movs	r0, #10
 80001fc:	|      2173      	movs	r1, #115	; 0x73
 80001fe:	|      236c      	movs	r3, #108	; 0x6c
 8000200:	|      2565      	movs	r5, #101	; 0x65
 8000202:	|      2265      	movs	r2, #101	; 0x65
 8000204:	\----> 72e1      	strb	r1, [r4, #11]
 8000206:	       212e      	movs	r1, #46	; 0x2e
 8000208:	       7322      	strb	r2, [r4, #12]
 800020a:	       72a3      	strb	r3, [r4, #10]
 800020c:	       7265      	strb	r5, [r4, #9]
 800020e:	       7221      	strb	r1, [r4, #8]
 8000210:	       6020      	str	r0, [r4, #0]
 8000212:	       b00a      	add	sp, #40	; 0x28
 8000214:	       bdb0      	pop	{r4, r5, r7, pc}

08000216 <_ZN10component214frob_it_static17h94599052a0401ea9E>:
 8000216:	       b5b0      	push	{r4, r5, r7, lr}
 8000218:	       af02      	add	r7, sp, #8
 800021a:	       b08a      	sub	sp, #40	; 0x28
 800021c:	       4604      	mov	r4, r0
 800021e:	       f640 40f5 	movw	r0, #3317	; 0xcf5
 8000222:	       f2c0 0000 	movt	r0, #0
 8000226:	       4615      	mov	r5, r2
 8000228:	       f640 626e 	movw	r2, #3694	; 0xe6e
 800022c:	       4478      	add	r0, pc
 800022e:	       f2c0 0200 	movt	r2, #0
 8000232:	       9101      	str	r1, [sp, #4]
 8000234:	       2102      	movs	r1, #2
 8000236:	       447a      	add	r2, pc
 8000238:	       e9cd 2104 	strd	r2, r1, [sp, #16]
 800023c:	       2101      	movs	r1, #1
 800023e:	       9009      	str	r0, [sp, #36]	; 0x24
 8000240:	       a801      	add	r0, sp, #4
 8000242:	       9107      	str	r1, [sp, #28]
 8000244:	       a908      	add	r1, sp, #32
 8000246:	       9008      	str	r0, [sp, #32]
 8000248:	       a802      	add	r0, sp, #8
 800024a:	       9106      	str	r1, [sp, #24]
 800024c:	       2100      	movs	r1, #0
 800024e:	       9102      	str	r1, [sp, #8]
 8000250:	       f000 f9a8 	bl	80005a4 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE>
 8000254:	       f640 6056 	movw	r0, #3670	; 0xe56
 8000258:	       210a      	movs	r1, #10
 800025a:	       f2c0 0000 	movt	r0, #0
 800025e:	       4478      	add	r0, pc
 8000260:	       f7ff fed5 	bl	800000e <print>
 8000264:	       206f      	movs	r0, #111	; 0x6f
 8000266:	       f244 615b 	movw	r1, #18011	; 0x465b
 800026a:	       71a0      	strb	r0, [r4, #6]
 800026c:	       9801      	ldr	r0, [sp, #4]
 800026e:	       80a1      	strh	r1, [r4, #4]
 8000270:	       4428      	add	r0, r5
 8000272:	       280a      	cmp	r0, #10
 8000274:	   /-- d104      	bne.n	8000280 <_ZN10component214frob_it_static17h94599052a0401ea9E+0x6a>
 8000276:	   |   2008      	movs	r0, #8
 8000278:	   |   210b      	movs	r1, #11
 800027a:	   |   2230      	movs	r2, #48	; 0x30
 800027c:	   |   2331      	movs	r3, #49	; 0x31
 800027e:	/--|-- e007      	b.n	8000290 <_ZN10component214frob_it_static17h94599052a0401ea9E+0x7a>
 8000280:	|  \-> f246 5073 	movw	r0, #25971	; 0x6573
 8000284:	|      210d      	movs	r1, #13
 8000286:	|      f8a4 000b 	strh.w	r0, [r4, #11]
 800028a:	|      200a      	movs	r0, #10
 800028c:	|      226c      	movs	r2, #108	; 0x6c
 800028e:	|      2365      	movs	r3, #101	; 0x65
 8000290:	\----> 72a2      	strb	r2, [r4, #10]
 8000292:	       225d      	movs	r2, #93	; 0x5d
 8000294:	       7263      	strb	r3, [r4, #9]
 8000296:	       5462      	strb	r2, [r4, r1]
 8000298:	       f642 616f 	movw	r1, #11887	; 0x2e6f
 800029c:	       f8a4 1007 	strh.w	r1, [r4, #7]
 80002a0:	       6020      	str	r0, [r4, #0]
 80002a2:	       b00a      	add	sp, #40	; 0x28
 80002a4:	       bdb0      	pop	{r4, r5, r7, pc}

080002a6 <_ZN10component215frob_it_dynamic17h22211adbaf86d08aE>:
 80002a6:	b580      	push	{r7, lr}
 80002a8:	466f      	mov	r7, sp
 80002aa:	4694      	mov	ip, r2
 80002ac:	461a      	mov	r2, r3
 80002ae:	47e0      	blx	ip
 80002b0:	bd80      	pop	{r7, pc}

080002b2 <example12>:
 80002b2:	       b5d0      	push	{r4, r6, r7, lr}
 80002b4:	       af02      	add	r7, sp, #8
 80002b6:	       b08e      	sub	sp, #56	; 0x38
 80002b8:	       466c      	mov	r4, sp
 80002ba:	       2109      	movs	r1, #9
 80002bc:	       4620      	mov	r0, r4
 80002be:	       2201      	movs	r2, #1
 80002c0:	       f7ff ffa9 	bl	8000216 <_ZN10component214frob_it_static17h94599052a0401ea9E>
 80002c4:	       9800      	ldr	r0, [sp, #0]
 80002c6:	       2808      	cmp	r0, #8
 80002c8:	/----- f040 80be 	bne.w	8000448 <example12+0x196>
 80002cc:	|      f640 51f0 	movw	r1, #3568	; 0xdf0
 80002d0:	|      1d20      	adds	r0, r4, #4
 80002d2:	|      f2c0 0100 	movt	r1, #0
 80002d6:	|      2208      	movs	r2, #8
 80002d8:	|      4479      	add	r1, pc
 80002da:	|      f000 fedc 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 80002de:	|      2800      	cmp	r0, #0
 80002e0:	+----- f040 80b2 	bne.w	8000448 <example12+0x196>
 80002e4:	|      466c      	mov	r4, sp
 80002e6:	|      210b      	movs	r1, #11
 80002e8:	|      4620      	mov	r0, r4
 80002ea:	|      2202      	movs	r2, #2
 80002ec:	|      f7ff ff93 	bl	8000216 <_ZN10component214frob_it_static17h94599052a0401ea9E>
 80002f0:	|      9800      	ldr	r0, [sp, #0]
 80002f2:	|      280a      	cmp	r0, #10
 80002f4:	+----- f040 80a8 	bne.w	8000448 <example12+0x196>
 80002f8:	|      f640 51ba 	movw	r1, #3514	; 0xdba
 80002fc:	|      1d20      	adds	r0, r4, #4
 80002fe:	|      f2c0 0100 	movt	r1, #0
 8000302:	|      220a      	movs	r2, #10
 8000304:	|      4479      	add	r1, pc
 8000306:	|      f000 fec6 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 800030a:	|      2800      	cmp	r0, #0
 800030c:	+----- f040 809c 	bne.w	8000448 <example12+0x196>
 8000310:	|      466c      	mov	r4, sp
 8000312:	|      f06f 010a 	mvn.w	r1, #10
 8000316:	|      4620      	mov	r0, r4
 8000318:	|      2201      	movs	r2, #1
 800031a:	|      f7ff ff37 	bl	800018c <_ZN10component214frob_it_static17h1223580fb40151bbE>
 800031e:	|      9800      	ldr	r0, [sp, #0]
 8000320:	|      2809      	cmp	r0, #9
 8000322:	+----- f040 8091 	bne.w	8000448 <example12+0x196>
 8000326:	|      f640 51ce 	movw	r1, #3534	; 0xdce
 800032a:	|      1d20      	adds	r0, r4, #4
 800032c:	|      f2c0 0100 	movt	r1, #0
 8000330:	|      2209      	movs	r2, #9
 8000332:	|      4479      	add	r1, pc
 8000334:	|      f000 feaf 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 8000338:	|      2800      	cmp	r0, #0
 800033a:	+----- f040 8085 	bne.w	8000448 <example12+0x196>
 800033e:	|      466c      	mov	r4, sp
 8000340:	|      210a      	movs	r1, #10
 8000342:	|      4620      	mov	r0, r4
 8000344:	|      2203      	movs	r2, #3
 8000346:	|      f7ff ff21 	bl	800018c <_ZN10component214frob_it_static17h1223580fb40151bbE>
 800034a:	|      9800      	ldr	r0, [sp, #0]
 800034c:	|      280a      	cmp	r0, #10
 800034e:	+----- d17b      	bne.n	8000448 <example12+0x196>
 8000350:	|      f640 519a 	movw	r1, #3482	; 0xd9a
 8000354:	|      1d20      	adds	r0, r4, #4
 8000356:	|      f2c0 0100 	movt	r1, #0
 800035a:	|      220a      	movs	r2, #10
 800035c:	|      4479      	add	r1, pc
 800035e:	|      f000 fe9a 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 8000362:	|      2800      	cmp	r0, #0
 8000364:	+----- d170      	bne.n	8000448 <example12+0x196>
 8000366:	|      f640 51a0 	movw	r1, #3488	; 0xda0
 800036a:	|      466c      	mov	r4, sp
 800036c:	|      f2c0 0100 	movt	r1, #0
 8000370:	|      f64f 42cd 	movw	r2, #64717	; 0xfccd
 8000374:	|      f6cf 72ff 	movt	r2, #65535	; 0xffff
 8000378:	|      4479      	add	r1, pc
 800037a:	|      447a      	add	r2, pc
 800037c:	|      4620      	mov	r0, r4
 800037e:	|      2301      	movs	r3, #1
 8000380:	|      f7ff ff91 	bl	80002a6 <_ZN10component215frob_it_dynamic17h22211adbaf86d08aE>
 8000384:	|      9800      	ldr	r0, [sp, #0]
 8000386:	|      2808      	cmp	r0, #8
 8000388:	+----- d15e      	bne.n	8000448 <example12+0x196>
 800038a:	|      f640 5132 	movw	r1, #3378	; 0xd32
 800038e:	|      1d20      	adds	r0, r4, #4
 8000390:	|      f2c0 0100 	movt	r1, #0
 8000394:	|      2208      	movs	r2, #8
 8000396:	|      4479      	add	r1, pc
 8000398:	|      f000 fe7d 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 800039c:	|      2800      	cmp	r0, #0
 800039e:	+----- d153      	bne.n	8000448 <example12+0x196>
 80003a0:	|      f640 515e 	movw	r1, #3422	; 0xd5e
 80003a4:	|      466c      	mov	r4, sp
 80003a6:	|      f2c0 0100 	movt	r1, #0
 80003aa:	|      f64f 4293 	movw	r2, #64659	; 0xfc93
 80003ae:	|      f6cf 72ff 	movt	r2, #65535	; 0xffff
 80003b2:	|      4479      	add	r1, pc
 80003b4:	|      447a      	add	r2, pc
 80003b6:	|      4620      	mov	r0, r4
 80003b8:	|      2302      	movs	r3, #2
 80003ba:	|      f7ff ff74 	bl	80002a6 <_ZN10component215frob_it_dynamic17h22211adbaf86d08aE>
 80003be:	|      9800      	ldr	r0, [sp, #0]
 80003c0:	|      280a      	cmp	r0, #10
 80003c2:	+----- d141      	bne.n	8000448 <example12+0x196>
 80003c4:	|      f640 41ee 	movw	r1, #3310	; 0xcee
 80003c8:	|      1d20      	adds	r0, r4, #4
 80003ca:	|      f2c0 0100 	movt	r1, #0
 80003ce:	|      220a      	movs	r2, #10
 80003d0:	|      4479      	add	r1, pc
 80003d2:	|      f000 fe60 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 80003d6:	+----- bbb8      	cbnz	r0, 8000448 <example12+0x196>
 80003d8:	|      f640 512a 	movw	r1, #3370	; 0xd2a
 80003dc:	|      466c      	mov	r4, sp
 80003de:	|      f2c0 0100 	movt	r1, #0
 80003e2:	|      f64f 42fd 	movw	r2, #64765	; 0xfcfd
 80003e6:	|      f6cf 72ff 	movt	r2, #65535	; 0xffff
 80003ea:	|      4479      	add	r1, pc
 80003ec:	|      447a      	add	r2, pc
 80003ee:	|      4620      	mov	r0, r4
 80003f0:	|      2301      	movs	r3, #1
 80003f2:	|      f7ff ff58 	bl	80002a6 <_ZN10component215frob_it_dynamic17h22211adbaf86d08aE>
 80003f6:	|      9800      	ldr	r0, [sp, #0]
 80003f8:	|      2809      	cmp	r0, #9
 80003fa:	+----- d125      	bne.n	8000448 <example12+0x196>
 80003fc:	|      f640 41f8 	movw	r1, #3320	; 0xcf8
 8000400:	|      1d20      	adds	r0, r4, #4
 8000402:	|      f2c0 0100 	movt	r1, #0
 8000406:	|      2209      	movs	r2, #9
 8000408:	|      4479      	add	r1, pc
 800040a:	|      f000 fe44 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 800040e:	+----- b9d8      	cbnz	r0, 8000448 <example12+0x196>
 8000410:	|      f640 41ea 	movw	r1, #3306	; 0xcea
 8000414:	|      466c      	mov	r4, sp
 8000416:	|      f2c0 0100 	movt	r1, #0
 800041a:	|      f64f 42c5 	movw	r2, #64709	; 0xfcc5
 800041e:	|      f6cf 72ff 	movt	r2, #65535	; 0xffff
 8000422:	|      4479      	add	r1, pc
 8000424:	|      447a      	add	r2, pc
 8000426:	|      4620      	mov	r0, r4
 8000428:	|      2303      	movs	r3, #3
 800042a:	|      f7ff ff3c 	bl	80002a6 <_ZN10component215frob_it_dynamic17h22211adbaf86d08aE>
 800042e:	|      9800      	ldr	r0, [sp, #0]
 8000430:	|      280a      	cmp	r0, #10
 8000432:	+----- d109      	bne.n	8000448 <example12+0x196>
 8000434:	|      f640 41b6 	movw	r1, #3254	; 0xcb6
 8000438:	|      1d20      	adds	r0, r4, #4
 800043a:	|      f2c0 0100 	movt	r1, #0
 800043e:	|      220a      	movs	r2, #10
 8000440:	|      4479      	add	r1, pc
 8000442:	|      f000 fe28 	bl	8001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>
 8000446:	|  /-- b110      	cbz	r0, 800044e <example12+0x19c>
 8000448:	\--|-> 2000      	movs	r0, #0
 800044a:	   |   b00e      	add	sp, #56	; 0x38
 800044c:	   |   bdd0      	pop	{r4, r6, r7, pc}
 800044e:	   \-> 2001      	movs	r0, #1
 8000450:	       b00e      	add	sp, #56	; 0x38
 8000452:	       bdd0      	pop	{r4, r6, r7, pc}

08000454 <_ZN4core3ptr70drop_in_place$LT$$RF$mut$u20$cortex_m_semihosting..hio..HostStream$GT$17h8314a63891215b2bE>:
 8000454:	4770      	bx	lr

08000456 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE>:
 8000456:	          b5d0      	push	{r4, r6, r7, lr}
 8000458:	          af02      	add	r7, sp, #8
 800045a:	          b084      	sub	sp, #16
 800045c:	          6800      	ldr	r0, [r0, #0]
 800045e:	          2980      	cmp	r1, #128	; 0x80
 8000460:	          f8d0 c000 	ldr.w	ip, [r0]
 8000464:	          f04f 0000 	mov.w	r0, #0
 8000468:	          9000      	str	r0, [sp, #0]
 800046a:	      /-- d204      	bcs.n	8000476 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0x20>
 800046c:	      |   f04f 0e01 	mov.w	lr, #1
 8000470:	      |   f88d 1000 	strb.w	r1, [sp]
 8000474:	/-----|-- e037      	b.n	80004e6 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0x90>
 8000476:	|     \-> f5b1 6f00 	cmp.w	r1, #2048	; 0x800
 800047a:	|     /-- d20a      	bcs.n	8000492 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0x3c>
 800047c:	|     |   f04f 0e02 	mov.w	lr, #2
 8000480:	|     |   4608      	mov	r0, r1
 8000482:	|     |   f36e 109f 	bfi	r0, lr, #6, #26
 8000486:	|     |   f88d 0001 	strb.w	r0, [sp, #1]
 800048a:	|     |   20c0      	movs	r0, #192	; 0xc0
 800048c:	|     |   ea40 1091 	orr.w	r0, r0, r1, lsr #6
 8000490:	|  /--|-- e027      	b.n	80004e2 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0x8c>
 8000492:	|  |  \-> 2002      	movs	r0, #2
 8000494:	|  |      460a      	mov	r2, r1
 8000496:	|  |      f360 129f 	bfi	r2, r0, #6, #26
 800049a:	|  |      f5b1 3f80 	cmp.w	r1, #65536	; 0x10000
 800049e:	|  |  /-- d20e      	bcs.n	80004be <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0x68>
 80004a0:	|  |  |   f88d 2002 	strb.w	r2, [sp, #2]
 80004a4:	|  |  |   098a      	lsrs	r2, r1, #6
 80004a6:	|  |  |   f360 129f 	bfi	r2, r0, #6, #26
 80004aa:	|  |  |   20e0      	movs	r0, #224	; 0xe0
 80004ac:	|  |  |   ea40 3011 	orr.w	r0, r0, r1, lsr #12
 80004b0:	|  |  |   f88d 2001 	strb.w	r2, [sp, #1]
 80004b4:	|  |  |   f88d 0000 	strb.w	r0, [sp]
 80004b8:	|  |  |   f04f 0e03 	mov.w	lr, #3
 80004bc:	+--|--|-- e013      	b.n	80004e6 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0x90>
 80004be:	|  |  \-> f88d 2003 	strb.w	r2, [sp, #3]
 80004c2:	|  |      098a      	lsrs	r2, r1, #6
 80004c4:	|  |      f360 129f 	bfi	r2, r0, #6, #26
 80004c8:	|  |      f88d 2002 	strb.w	r2, [sp, #2]
 80004cc:	|  |      0b0a      	lsrs	r2, r1, #12
 80004ce:	|  |      f04f 0e04 	mov.w	lr, #4
 80004d2:	|  |      f360 129f 	bfi	r2, r0, #6, #26
 80004d6:	|  |      0c88      	lsrs	r0, r1, #18
 80004d8:	|  |      211e      	movs	r1, #30
 80004da:	|  |      f88d 2001 	strb.w	r2, [sp, #1]
 80004de:	|  |      f361 00df 	bfi	r0, r1, #3, #29
 80004e2:	|  \----> f88d 0000 	strb.w	r0, [sp]
 80004e6:	\-------> 4668      	mov	r0, sp
 80004e8:	          f8cd e00c 	str.w	lr, [sp, #12]
 80004ec:	          e9cd c001 	strd	ip, r0, [sp, #4]
 80004f0:	          a901      	add	r1, sp, #4
 80004f2:	          2005      	movs	r0, #5
 80004f4:	          beab      	bkpt	0x00ab
 80004f6:	          1e41      	subs	r1, r0, #1
 80004f8:	          4571      	cmp	r1, lr
 80004fa:	   /----- d20e      	bcs.n	800051a <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0xc4>
 80004fc:	   |      a901      	add	r1, sp, #4
 80004fe:	   |      466c      	mov	r4, sp
 8000500:	   |  /-> 4603      	mov	r3, r0
 8000502:	   |  |   9003      	str	r0, [sp, #12]
 8000504:	   |  |   ebae 0000 	sub.w	r0, lr, r0
 8000508:	   |  |   469e      	mov	lr, r3
 800050a:	   |  |   4404      	add	r4, r0
 800050c:	   |  |   2005      	movs	r0, #5
 800050e:	   |  |   e9cd c401 	strd	ip, r4, [sp, #4]
 8000512:	   |  |   beab      	bkpt	0x00ab
 8000514:	   |  |   1e42      	subs	r2, r0, #1
 8000516:	   |  |   429a      	cmp	r2, r3
 8000518:	   |  \-- d3f2      	bcc.n	8000500 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h23f13ecd698af87bE+0xaa>
 800051a:	   \----> 2800      	cmp	r0, #0
 800051c:	          bf18      	it	ne
 800051e:	          2001      	movne	r0, #1
 8000520:	          b004      	add	sp, #16
 8000522:	          bdd0      	pop	{r4, r6, r7, pc}

08000524 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h57ac53495b90c12eE>:
 8000524:	b5f0      	push	{r4, r5, r6, r7, lr}
 8000526:	af03      	add	r7, sp, #12
 8000528:	f84d 8d04 	str.w	r8, [sp, #-4]!
 800052c:	b088      	sub	sp, #32
 800052e:	6800      	ldr	r0, [r0, #0]
 8000530:	f640 3ce0 	movw	ip, #3040	; 0xbe0
 8000534:	f2c0 0c00 	movt	ip, #0
 8000538:	9001      	str	r0, [sp, #4]
 800053a:	aa02      	add	r2, sp, #8
 800053c:	e891 4178 	ldmia.w	r1, {r3, r4, r5, r6, r8, lr}
 8000540:	44fc      	add	ip, pc
 8000542:	4610      	mov	r0, r2
 8000544:	e880 4178 	stmia.w	r0, {r3, r4, r5, r6, r8, lr}
 8000548:	a801      	add	r0, sp, #4
 800054a:	4661      	mov	r1, ip
 800054c:	f000 f87f 	bl	800064e <_ZN4core3fmt5write17h19a390c2d0bed8bbE>
 8000550:	b008      	add	sp, #32
 8000552:	f85d 8b04 	ldr.w	r8, [sp], #4
 8000556:	bdf0      	pop	{r4, r5, r6, r7, pc}

08000558 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h465d4268c7f26d19E>:
 8000558:	       2a00      	cmp	r2, #0
 800055a:	       bf04      	itt	eq
 800055c:	       2000      	moveq	r0, #0
 800055e:	       4770      	bxeq	lr
 8000560:	       b580      	push	{r7, lr}
 8000562:	       466f      	mov	r7, sp
 8000564:	       b083      	sub	sp, #12
 8000566:	       6800      	ldr	r0, [r0, #0]
 8000568:	       468e      	mov	lr, r1
 800056a:	       9202      	str	r2, [sp, #8]
 800056c:	       f8d0 c000 	ldr.w	ip, [r0]
 8000570:	       2005      	movs	r0, #5
 8000572:	       e9cd c100 	strd	ip, r1, [sp]
 8000576:	       4669      	mov	r1, sp
 8000578:	       beab      	bkpt	0x00ab
 800057a:	       1e41      	subs	r1, r0, #1
 800057c:	       4291      	cmp	r1, r2
 800057e:	/----- d20c      	bcs.n	800059a <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h465d4268c7f26d19E+0x42>
 8000580:	|      4669      	mov	r1, sp
 8000582:	|  /-> 4603      	mov	r3, r0
 8000584:	|  |   9002      	str	r0, [sp, #8]
 8000586:	|  |   1a10      	subs	r0, r2, r0
 8000588:	|  |   4486      	add	lr, r0
 800058a:	|  |   2005      	movs	r0, #5
 800058c:	|  |   e9cd ce00 	strd	ip, lr, [sp]
 8000590:	|  |   beab      	bkpt	0x00ab
 8000592:	|  |   1e42      	subs	r2, r0, #1
 8000594:	|  |   429a      	cmp	r2, r3
 8000596:	|  |   461a      	mov	r2, r3
 8000598:	|  \-- d3f3      	bcc.n	8000582 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h465d4268c7f26d19E+0x2a>
 800059a:	\----> 2800      	cmp	r0, #0
 800059c:	       bf18      	it	ne
 800059e:	       2001      	movne	r0, #1
 80005a0:	       b003      	add	sp, #12
 80005a2:	       bd80      	pop	{r7, pc}

080005a4 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE>:
 80005a4:	       b5f0      	push	{r4, r5, r6, r7, lr}
 80005a6:	       af03      	add	r7, sp, #12
 80005a8:	       e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 80005ac:	       b088      	sub	sp, #32
 80005ae:	       f3ef 8510 	mrs	r5, PRIMASK
 80005b2:	       b672      	cpsid	i
 80005b4:	       4682      	mov	sl, r0
 80005b6:	       f000 f839 	bl	800062c <_ZN10bare_metal15CriticalSection3new17h66b3e307384ac6b0E>
 80005ba:	       f240 0200 	movw	r2, #0
 80005be:	       f005 0801 	and.w	r8, r5, #1
 80005c2:	       f2c0 0200 	movt	r2, #0
 80005c6:	       f859 0002 	ldr.w	r0, [r9, r2]
 80005ca:	       2801      	cmp	r0, #1
 80005cc:	   /-- d014      	beq.n	80005f8 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE+0x54>
 80005ce:	   |   f640 3042 	movw	r0, #2882	; 0xb42
 80005d2:	   |   2103      	movs	r1, #3
 80005d4:	   |   f2c0 0000 	movt	r0, #0
 80005d8:	   |   9104      	str	r1, [sp, #16]
 80005da:	   |   4478      	add	r0, pc
 80005dc:	   |   2104      	movs	r1, #4
 80005de:	   |   e9cd 0102 	strd	r0, r1, [sp, #8]
 80005e2:	   |   a902      	add	r1, sp, #8
 80005e4:	   |   2001      	movs	r0, #1
 80005e6:	   |   2301      	movs	r3, #1
 80005e8:	   |   beab      	bkpt	0x00ab
 80005ea:	   |   1c41      	adds	r1, r0, #1
 80005ec:	/--|-- d016      	beq.n	800061c <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE+0x78>
 80005ee:	|  |   eb09 0102 	add.w	r1, r9, r2
 80005f2:	|  |   f849 3002 	str.w	r3, [r9, r2]
 80005f6:	|  |   6048      	str	r0, [r1, #4]
 80005f8:	|  \-> f640 3112 	movw	r1, #2834	; 0xb12
 80005fc:	|      eb09 0002 	add.w	r0, r9, r2
 8000600:	|      f2c0 0100 	movt	r1, #0
 8000604:	|      3004      	adds	r0, #4
 8000606:	|      9001      	str	r0, [sp, #4]
 8000608:	|      aa02      	add	r2, sp, #8
 800060a:	|      e89a 5078 	ldmia.w	sl, {r3, r4, r5, r6, ip, lr}
 800060e:	|      4479      	add	r1, pc
 8000610:	|      4610      	mov	r0, r2
 8000612:	|      e880 5078 	stmia.w	r0, {r3, r4, r5, r6, ip, lr}
 8000616:	|      a801      	add	r0, sp, #4
 8000618:	|      f000 f819 	bl	800064e <_ZN4core3fmt5write17h19a390c2d0bed8bbE>
 800061c:	\----> f1b8 0f00 	cmp.w	r8, #0
 8000620:	   /-- d100      	bne.n	8000624 <_ZN20cortex_m_semihosting6export11hstdout_fmt17hfcbeaeddd8276f2eE+0x80>
 8000622:	   |   b662      	cpsie	i
 8000624:	   \-> b008      	add	sp, #32
 8000626:	       e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 800062a:	       bdf0      	pop	{r4, r5, r6, r7, pc}

0800062c <_ZN10bare_metal15CriticalSection3new17h66b3e307384ac6b0E>:
 800062c:	4770      	bx	lr

0800062e <_ZN7userlib91_$LT$impl$u20$core..convert..From$LT$userlib..RcLen$GT$$u20$for$u20$$LP$u32$C$usize$RP$$GT$4from17h02b830a50f92d613E>:
 800062e:	4770      	bx	lr

08000630 <_ZN7userlib13sys_send_stub17h2444658aabf553a4E>:
 8000630:	e92d 0ff0 	stmdb	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 8000634:	e890 07f0 	ldmia.w	r0, {r4, r5, r6, r7, r8, r9, sl}
 8000638:	f04f 0b00 	mov.w	fp, #0
 800063c:	df00      	svc	0
 800063e:	4620      	mov	r0, r4
 8000640:	4629      	mov	r1, r5
 8000642:	e8bd 0ff0 	ldmia.w	sp!, {r4, r5, r6, r7, r8, r9, sl, fp}
 8000646:	4770      	bx	lr
 8000648:	defe      	udf	#254	; 0xfe

0800064a <_ZN4core3ops8function6FnOnce9call_once17hf29a6adbaf16d366E>:
 800064a:	    6800      	ldr	r0, [r0, #0]
 800064c:	/-- e7fe      	b.n	800064c <_ZN4core3ops8function6FnOnce9call_once17hf29a6adbaf16d366E+0x2>

0800064e <_ZN4core3fmt5write17h19a390c2d0bed8bbE>:
 800064e:	                         b5f0      	push	{r4, r5, r6, r7, lr}
 8000650:	                         af03      	add	r7, sp, #12
 8000652:	                         e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 8000656:	                         b08c      	sub	sp, #48	; 0x30
 8000658:	                         2303      	movs	r3, #3
 800065a:	                         6816      	ldr	r6, [r2, #0]
 800065c:	                         f88d 302c 	strb.w	r3, [sp, #44]	; 0x2c
 8000660:	                         2320      	movs	r3, #32
 8000662:	                         930a      	str	r3, [sp, #40]	; 0x28
 8000664:	                         2300      	movs	r3, #0
 8000666:	                         2e00      	cmp	r6, #0
 8000668:	                         9309      	str	r3, [sp, #36]	; 0x24
 800066a:	                         9307      	str	r3, [sp, #28]
 800066c:	                         e9cd 1304 	strd	r1, r3, [sp, #16]
 8000670:	                         9003      	str	r0, [sp, #12]
 8000672:	      /----------------- d075      	beq.n	8000760 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x112>
 8000674:	      |                  6850      	ldr	r0, [r2, #4]
 8000676:	      |                  2800      	cmp	r0, #0
 8000678:	/-----|----------------- f000 809a 	beq.w	80007b0 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x162>
 800067c:	|     |                  f8d2 b008 	ldr.w	fp, [r2, #8]
 8000680:	|     |                  0141      	lsls	r1, r0, #5
 8000682:	|     |                  3801      	subs	r0, #1
 8000684:	|     |                  f04f 0800 	mov.w	r8, #0
 8000688:	|     |                  f020 4078 	bic.w	r0, r0, #4160749568	; 0xf8000000
 800068c:	|     |                  f04f 0a00 	mov.w	sl, #0
 8000690:	|     |                  3001      	adds	r0, #1
 8000692:	|     |                  e9cd 1201 	strd	r1, r2, [sp, #4]
 8000696:	|     |                  9000      	str	r0, [sp, #0]
 8000698:	|     |  /-------------> eb0b 000a 	add.w	r0, fp, sl
 800069c:	|     |  |               6842      	ldr	r2, [r0, #4]
 800069e:	|     |  |           /-- b142      	cbz	r2, 80006b2 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x64>
 80006a0:	|     |  |           |   e9dd 0303 	ldrd	r0, r3, [sp, #12]
 80006a4:	|     |  |           |   f85b 100a 	ldr.w	r1, [fp, sl]
 80006a8:	|     |  |           |   68db      	ldr	r3, [r3, #12]
 80006aa:	|     |  |           |   4798      	blx	r3
 80006ac:	|     |  |           |   2800      	cmp	r0, #0
 80006ae:	|  /--|--|-----------|-- f040 8090 	bne.w	80007d2 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x184>
 80006b2:	|  |  |  |           \-> eb06 008a 	add.w	r0, r6, sl, lsl #2
 80006b6:	|  |  |  |               9902      	ldr	r1, [sp, #8]
 80006b8:	|  |  |  |               e9d0 2405 	ldrd	r2, r4, [r0, #20]
 80006bc:	|  |  |  |               68c3      	ldr	r3, [r0, #12]
 80006be:	|  |  |  |               7f05      	ldrb	r5, [r0, #28]
 80006c0:	|  |  |  |               9209      	str	r2, [sp, #36]	; 0x24
 80006c2:	|  |  |  |               eb06 0208 	add.w	r2, r6, r8
 80006c6:	|  |  |  |               940a      	str	r4, [sp, #40]	; 0x28
 80006c8:	|  |  |  |               f88d 502c 	strb.w	r5, [sp, #44]	; 0x2c
 80006cc:	|  |  |  |               6909      	ldr	r1, [r1, #16]
 80006ce:	|  |  |  |               6912      	ldr	r2, [r2, #16]
 80006d0:	|  |  |  |     /-------- b1a3      	cbz	r3, 80006fc <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xae>
 80006d2:	|  |  |  |     |         2b01      	cmp	r3, #1
 80006d4:	|  |  |  |     |  /----- d109      	bne.n	80006ea <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x9c>
 80006d6:	|  |  |  |     |  |      eb01 02c2 	add.w	r2, r1, r2, lsl #3
 80006da:	|  |  |  |     |  |      6853      	ldr	r3, [r2, #4]
 80006dc:	|  |  |  |     |  |      f64f 7563 	movw	r5, #65379	; 0xff63
 80006e0:	|  |  |  |     |  |      f6cf 75ff 	movt	r5, #65535	; 0xffff
 80006e4:	|  |  |  |     |  |      447d      	add	r5, pc
 80006e6:	|  |  |  |     |  |      42ab      	cmp	r3, r5
 80006e8:	|  |  |  |     |  |  /-- d006      	beq.n	80006f8 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xaa>
 80006ea:	|  |  |  |     |  \--|-> 2300      	movs	r3, #0
 80006ec:	|  |  |  |     |     |   e9cd 3205 	strd	r3, r2, [sp, #20]
 80006f0:	|  |  |  |     |     |   e9d0 2001 	ldrd	r2, r0, [r0, #4]
 80006f4:	|  |  |  |     |  /--|-- b942      	cbnz	r2, 8000708 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xba>
 80006f6:	|  |  |  |  /--|--|--|-- e017      	b.n	8000728 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xda>
 80006f8:	|  |  |  |  |  |  |  \-> 6812      	ldr	r2, [r2, #0]
 80006fa:	|  |  |  |  |  |  |      6812      	ldr	r2, [r2, #0]
 80006fc:	|  |  |  |  |  \--|----> 2301      	movs	r3, #1
 80006fe:	|  |  |  |  |     |      e9cd 3205 	strd	r3, r2, [sp, #20]
 8000702:	|  |  |  |  |     |      e9d0 2001 	ldrd	r2, r0, [r0, #4]
 8000706:	|  |  |  |  +-----|----- b17a      	cbz	r2, 8000728 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xda>
 8000708:	|  |  |  |  |     \----> 2a01      	cmp	r2, #1
 800070a:	|  |  |  |  |     /----- d109      	bne.n	8000720 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xd2>
 800070c:	|  |  |  |  |     |      eb01 00c0 	add.w	r0, r1, r0, lsl #3
 8000710:	|  |  |  |  |     |      6842      	ldr	r2, [r0, #4]
 8000712:	|  |  |  |  |     |      f64f 732d 	movw	r3, #65325	; 0xff2d
 8000716:	|  |  |  |  |     |      f6cf 73ff 	movt	r3, #65535	; 0xffff
 800071a:	|  |  |  |  |     |      447b      	add	r3, pc
 800071c:	|  |  |  |  |     |      429a      	cmp	r2, r3
 800071e:	|  |  |  |  |     |  /-- d001      	beq.n	8000724 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xd6>
 8000720:	|  |  |  |  |     \--|-> 2200      	movs	r2, #0
 8000722:	|  |  |  |  |     /--|-- e002      	b.n	800072a <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0xdc>
 8000724:	|  |  |  |  |     |  \-> 6800      	ldr	r0, [r0, #0]
 8000726:	|  |  |  |  |     |      6800      	ldr	r0, [r0, #0]
 8000728:	|  |  |  |  \-----|----> 2201      	movs	r2, #1
 800072a:	|  |  |  |        \----> f856 3008 	ldr.w	r3, [r6, r8]
 800072e:	|  |  |  |               9008      	str	r0, [sp, #32]
 8000730:	|  |  |  |               f851 0033 	ldr.w	r0, [r1, r3, lsl #3]
 8000734:	|  |  |  |               eb01 01c3 	add.w	r1, r1, r3, lsl #3
 8000738:	|  |  |  |               684b      	ldr	r3, [r1, #4]
 800073a:	|  |  |  |               a903      	add	r1, sp, #12
 800073c:	|  |  |  |               9207      	str	r2, [sp, #28]
 800073e:	|  |  |  |               4798      	blx	r3
 8000740:	|  |  |  |               2800      	cmp	r0, #0
 8000742:	|  +--|--|-------------- d146      	bne.n	80007d2 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x184>
 8000744:	|  |  |  |               9801      	ldr	r0, [sp, #4]
 8000746:	|  |  |  |               f108 0820 	add.w	r8, r8, #32
 800074a:	|  |  |  |               f10a 0a08 	add.w	sl, sl, #8
 800074e:	|  |  |  |               4540      	cmp	r0, r8
 8000750:	|  |  |  \-------------- d1a2      	bne.n	8000698 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x4a>
 8000752:	|  |  |                  9a02      	ldr	r2, [sp, #8]
 8000754:	|  |  |                  f8dd b000 	ldr.w	fp, [sp]
 8000758:	|  |  |                  68d0      	ldr	r0, [r2, #12]
 800075a:	|  |  |                  4583      	cmp	fp, r0
 800075c:	|  |  |        /-------- d32d      	bcc.n	80007ba <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x16c>
 800075e:	|  |  |     /--|-------- e03d      	b.n	80007dc <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x18e>
 8000760:	|  |  \-----|--|-------> 6950      	ldr	r0, [r2, #20]
 8000762:	+--|--------|--|-------- b328      	cbz	r0, 80007b0 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x162>
 8000764:	|  |        |  |         6911      	ldr	r1, [r2, #16]
 8000766:	|  |        |  |         00c5      	lsls	r5, r0, #3
 8000768:	|  |        |  |         3801      	subs	r0, #1
 800076a:	|  |        |  |         f10d 080c 	add.w	r8, sp, #12
 800076e:	|  |        |  |         1d0e      	adds	r6, r1, #4
 8000770:	|  |        |  |         6891      	ldr	r1, [r2, #8]
 8000772:	|  |        |  |         f020 4060 	bic.w	r0, r0, #3758096384	; 0xe0000000
 8000776:	|  |        |  |         4692      	mov	sl, r2
 8000778:	|  |        |  |         1d0c      	adds	r4, r1, #4
 800077a:	|  |        |  |         f100 0b01 	add.w	fp, r0, #1
 800077e:	|  |        |  |  /----> 6822      	ldr	r2, [r4, #0]
 8000780:	|  |        |  |  |  /-- b132      	cbz	r2, 8000790 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x142>
 8000782:	|  |        |  |  |  |   e9dd 0303 	ldrd	r0, r3, [sp, #12]
 8000786:	|  |        |  |  |  |   f854 1c04 	ldr.w	r1, [r4, #-4]
 800078a:	|  |        |  |  |  |   68db      	ldr	r3, [r3, #12]
 800078c:	|  |        |  |  |  |   4798      	blx	r3
 800078e:	|  +--------|--|--|--|-- bb00      	cbnz	r0, 80007d2 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x184>
 8000790:	|  |        |  |  |  \-> f856 0c04 	ldr.w	r0, [r6, #-4]
 8000794:	|  |        |  |  |      4641      	mov	r1, r8
 8000796:	|  |        |  |  |      6832      	ldr	r2, [r6, #0]
 8000798:	|  |        |  |  |      4790      	blx	r2
 800079a:	|  +--------|--|--|----- b9d0      	cbnz	r0, 80007d2 <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x184>
 800079c:	|  |        |  |  |      3608      	adds	r6, #8
 800079e:	|  |        |  |  |      3d08      	subs	r5, #8
 80007a0:	|  |        |  |  |      f104 0408 	add.w	r4, r4, #8
 80007a4:	|  |        |  |  \----- d1eb      	bne.n	800077e <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x130>
 80007a6:	|  |        |  |         4652      	mov	r2, sl
 80007a8:	|  |        |  |         68d0      	ldr	r0, [r2, #12]
 80007aa:	|  |        |  |         4583      	cmp	fp, r0
 80007ac:	|  |        |  +-------- d305      	bcc.n	80007ba <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x16c>
 80007ae:	|  |        +--|-------- e015      	b.n	80007dc <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x18e>
 80007b0:	\--|--------|--|-------> f04f 0b00 	mov.w	fp, #0
 80007b4:	   |        |  |         68d0      	ldr	r0, [r2, #12]
 80007b6:	   |        |  |         4583      	cmp	fp, r0
 80007b8:	   |        +--|-------- d210      	bcs.n	80007dc <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x18e>
 80007ba:	   |        |  \-------> 6892      	ldr	r2, [r2, #8]
 80007bc:	   |        |            465e      	mov	r6, fp
 80007be:	   |        |            e9dd 0103 	ldrd	r0, r1, [sp, #12]
 80007c2:	   |        |            68cb      	ldr	r3, [r1, #12]
 80007c4:	   |        |            f852 103b 	ldr.w	r1, [r2, fp, lsl #3]
 80007c8:	   |        |            eb02 02cb 	add.w	r2, r2, fp, lsl #3
 80007cc:	   |        |            6852      	ldr	r2, [r2, #4]
 80007ce:	   |        |            4798      	blx	r3
 80007d0:	   |        +----------- b120      	cbz	r0, 80007dc <_ZN4core3fmt5write17h19a390c2d0bed8bbE+0x18e>
 80007d2:	   \--------|----------> 2001      	movs	r0, #1
 80007d4:	            |            b00c      	add	sp, #48	; 0x30
 80007d6:	            |            e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 80007da:	            |            bdf0      	pop	{r4, r5, r6, r7, pc}
 80007dc:	            \----------> 2000      	movs	r0, #0
 80007de:	                         b00c      	add	sp, #48	; 0x30
 80007e0:	                         e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 80007e4:	                         bdf0      	pop	{r4, r5, r6, r7, pc}

080007e6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE>:
 80007e6:	                                        b5f0      	push	{r4, r5, r6, r7, lr}
 80007e8:	                                        af03      	add	r7, sp, #12
 80007ea:	                                        e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 80007ee:	                                        b088      	sub	sp, #32
 80007f0:	                                        68fc      	ldr	r4, [r7, #12]
 80007f2:	                                        461e      	mov	r6, r3
 80007f4:	                                        4696      	mov	lr, r2
 80007f6:	                                        4605      	mov	r5, r0
 80007f8:	                              /-------- b1f9      	cbz	r1, 800083a <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x54>
 80007fa:	                              |         69ab      	ldr	r3, [r5, #24]
 80007fc:	                              |         f04f 0b2b 	mov.w	fp, #43	; 0x2b
 8000800:	                              |         f013 0001 	ands.w	r0, r3, #1
 8000804:	                              |         bf08      	it	eq
 8000806:	                              |         f44f 1b88 	moveq.w	fp, #1114112	; 0x110000
 800080a:	                              |         eb00 0804 	add.w	r8, r0, r4
 800080e:	                              |         f8d7 a008 	ldr.w	sl, [r7, #8]
 8000812:	                              |         0758      	lsls	r0, r3, #29
 8000814:	                              |  /----- d51a      	bpl.n	800084c <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x66>
 8000816:	                              |  |  /-> 2e10      	cmp	r6, #16
 8000818:	                           /--|--|--|-- d249      	bcs.n	80008ae <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0xc8>
 800081a:	                           |  |  |  |   2e00      	cmp	r6, #0
 800081c:	                     /-----|--|--|--|-- f000 8084 	beq.w	8000928 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x142>
 8000820:	                     |     |  |  |  |   1e70      	subs	r0, r6, #1
 8000822:	                     |     |  |  |  |   f006 0c03 	and.w	ip, r6, #3
 8000826:	                     |     |  |  |  |   2803      	cmp	r0, #3
 8000828:	                     |  /--|--|--|--|-- f080 8080 	bcs.w	800092c <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x146>
 800082c:	                     |  |  |  |  |  |   2000      	movs	r0, #0
 800082e:	                     |  |  |  |  |  |   4672      	mov	r2, lr
 8000830:	                     |  |  |  |  |  |   f1bc 0f00 	cmp.w	ip, #0
 8000834:	                  /--|--|--|--|--|--|-- f040 80a7 	bne.w	8000986 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1a0>
 8000838:	         /--------|--|--|--|--|--|--|-- e0bd      	b.n	80009b6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1d0>
 800083a:	         |        |  |  |  |  \--|--|-> 69ab      	ldr	r3, [r5, #24]
 800083c:	         |        |  |  |  |     |  |   f104 0801 	add.w	r8, r4, #1
 8000840:	         |        |  |  |  |     |  |   f04f 0b2d 	mov.w	fp, #45	; 0x2d
 8000844:	         |        |  |  |  |     |  |   f8d7 a008 	ldr.w	sl, [r7, #8]
 8000848:	         |        |  |  |  |     |  |   0758      	lsls	r0, r3, #29
 800084a:	         |        |  |  |  |     |  \-- d4e4      	bmi.n	8000816 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x30>
 800084c:	         |        |  |  |  |     \----> f04f 0e00 	mov.w	lr, #0
 8000850:	         |        |  |  |  |            68a8      	ldr	r0, [r5, #8]
 8000852:	         |        |  |  |  |            2800      	cmp	r0, #0
 8000854:	         |  /-----|--|--|--|----------- f000 80b4 	beq.w	80009c0 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1da>
 8000858:	         |  |  /--|--|--|--|----------> e9cd b606 	strd	fp, r6, [sp, #24]
 800085c:	         |  |  |  |  |  |  |            f8d5 b00c 	ldr.w	fp, [r5, #12]
 8000860:	         |  |  |  |  |  |  |            45c3      	cmp	fp, r8
 8000862:	         |  |  |  |  |  |  |        /-- d915      	bls.n	8000890 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0xaa>
 8000864:	         |  |  |  |  |  |  |        |   0718      	lsls	r0, r3, #28
 8000866:	         |  |  |  |  |  |  |        |   f8cd a014 	str.w	sl, [sp, #20]
 800086a:	         |  |  |  |  |  |  |     /--|-- d433      	bmi.n	80008d4 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0xee>
 800086c:	         |  |  |  |  |  |  |     |  |   f895 0020 	ldrb.w	r0, [r5, #32]
 8000870:	         |  |  |  |  |  |  |     |  |   ebab 0108 	sub.w	r1, fp, r8
 8000874:	         |  |  |  |  |  |  |     |  |   46f3      	mov	fp, lr
 8000876:	         |  |  |  |  |  |  |     |  |   2803      	cmp	r0, #3
 8000878:	         |  |  |  |  |  |  |     |  |   bf08      	it	eq
 800087a:	         |  |  |  |  |  |  |     |  |   2001      	moveq	r0, #1
 800087c:	         |  |  |  |  |  |  |     |  |   0782      	lsls	r2, r0, #30
 800087e:	         |  |  |  |  |  |  |     |  |   9404      	str	r4, [sp, #16]
 8000880:	/--------|--|--|--|--|--|--|-----|--|-- f000 80c1 	beq.w	8000a06 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x220>
 8000884:	|        |  |  |  |  |  |  |     |  |   2801      	cmp	r0, #1
 8000886:	|  /-----|--|--|--|--|--|--|-----|--|-- f040 80c0 	bne.w	8000a0a <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x224>
 800088a:	|  |     |  |  |  |  |  |  |     |  |   2400      	movs	r4, #0
 800088c:	|  |     |  |  |  |  |  |  |     |  |   4608      	mov	r0, r1
 800088e:	|  |  /--|--|--|--|--|--|--|-----|--|-- e0bf      	b.n	8000a10 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x22a>
 8000890:	|  |  |  |  |  |  |  |  |  |     |  \-> 9807      	ldr	r0, [sp, #28]
 8000892:	|  |  |  |  |  |  |  |  |  |     |      4626      	mov	r6, r4
 8000894:	|  |  |  |  |  |  |  |  |  |     |      e9d5 4500 	ldrd	r4, r5, [r5]
 8000898:	|  |  |  |  |  |  |  |  |  |     |      4673      	mov	r3, lr
 800089a:	|  |  |  |  |  |  |  |  |  |     |      9000      	str	r0, [sp, #0]
 800089c:	|  |  |  |  |  |  |  |  |  |     |      4629      	mov	r1, r5
 800089e:	|  |  |  |  |  |  |  |  |  |     |      9a06      	ldr	r2, [sp, #24]
 80008a0:	|  |  |  |  |  |  |  |  |  |     |      4620      	mov	r0, r4
 80008a2:	|  |  |  |  |  |  |  |  |  |     |      f000 f909 	bl	8000ab8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE>
 80008a6:	|  |  |  |  |  |  |  |  |  |  /--|----- b3a8      	cbz	r0, 8000914 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x12e>
 80008a8:	|  |  |  |  |  |  |  |  |  |  |  |      f04f 0801 	mov.w	r8, #1
 80008ac:	|  |  |  |  |  |  |  |  |  |  |  |  /-- e02d      	b.n	800090a <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x124>
 80008ae:	|  |  |  |  |  |  |  |  |  \--|--|--|-> 4670      	mov	r0, lr
 80008b0:	|  |  |  |  |  |  |  |  |     |  |  |   4631      	mov	r1, r6
 80008b2:	|  |  |  |  |  |  |  |  |     |  |  |   f8cd 801c 	str.w	r8, [sp, #28]
 80008b6:	|  |  |  |  |  |  |  |  |     |  |  |   4698      	mov	r8, r3
 80008b8:	|  |  |  |  |  |  |  |  |     |  |  |   f8cd a014 	str.w	sl, [sp, #20]
 80008bc:	|  |  |  |  |  |  |  |  |     |  |  |   46a2      	mov	sl, r4
 80008be:	|  |  |  |  |  |  |  |  |     |  |  |   4674      	mov	r4, lr
 80008c0:	|  |  |  |  |  |  |  |  |     |  |  |   f000 f91d 	bl	8000afe <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E>
 80008c4:	|  |  |  |  |  |  |  |  |     |  |  |   4643      	mov	r3, r8
 80008c6:	|  |  |  |  |  |  |  |  |     |  |  |   46a6      	mov	lr, r4
 80008c8:	|  |  |  |  |  |  |  |  |     |  |  |   4654      	mov	r4, sl
 80008ca:	|  |  |  |  |  |  |  |  |     |  |  |   f8dd a014 	ldr.w	sl, [sp, #20]
 80008ce:	|  |  |  |  |  |  |  |  |     |  |  |   f8dd 801c 	ldr.w	r8, [sp, #28]
 80008d2:	|  |  |  +--|--|--|--|--|-----|--|--|-- e070      	b.n	80009b6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1d0>
 80008d4:	|  |  |  |  |  |  |  |  |     |  \--|-> 4629      	mov	r1, r5
 80008d6:	|  |  |  |  |  |  |  |  |     |     |   4646      	mov	r6, r8
 80008d8:	|  |  |  |  |  |  |  |  |     |     |   69c8      	ldr	r0, [r1, #28]
 80008da:	|  |  |  |  |  |  |  |  |     |     |   f04f 0801 	mov.w	r8, #1
 80008de:	|  |  |  |  |  |  |  |  |     |     |   f891 2020 	ldrb.w	r2, [r1, #32]
 80008e2:	|  |  |  |  |  |  |  |  |     |     |   4673      	mov	r3, lr
 80008e4:	|  |  |  |  |  |  |  |  |     |     |   9002      	str	r0, [sp, #8]
 80008e6:	|  |  |  |  |  |  |  |  |     |     |   2030      	movs	r0, #48	; 0x30
 80008e8:	|  |  |  |  |  |  |  |  |     |     |   9201      	str	r2, [sp, #4]
 80008ea:	|  |  |  |  |  |  |  |  |     |     |   9103      	str	r1, [sp, #12]
 80008ec:	|  |  |  |  |  |  |  |  |     |     |   61c8      	str	r0, [r1, #28]
 80008ee:	|  |  |  |  |  |  |  |  |     |     |   9807      	ldr	r0, [sp, #28]
 80008f0:	|  |  |  |  |  |  |  |  |     |     |   f8d5 a000 	ldr.w	sl, [r5]
 80008f4:	|  |  |  |  |  |  |  |  |     |     |   686d      	ldr	r5, [r5, #4]
 80008f6:	|  |  |  |  |  |  |  |  |     |     |   f881 8020 	strb.w	r8, [r1, #32]
 80008fa:	|  |  |  |  |  |  |  |  |     |     |   9000      	str	r0, [sp, #0]
 80008fc:	|  |  |  |  |  |  |  |  |     |     |   4650      	mov	r0, sl
 80008fe:	|  |  |  |  |  |  |  |  |     |     |   9a06      	ldr	r2, [sp, #24]
 8000900:	|  |  |  |  |  |  |  |  |     |     |   4629      	mov	r1, r5
 8000902:	|  |  |  |  |  |  |  |  |     |     |   f000 f8d9 	bl	8000ab8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE>
 8000906:	|  |  |  |  |  |  |  |  |     |     |   2800      	cmp	r0, #0
 8000908:	|  |  |  |  |  |  |  |  |     |  /--|-- d070      	beq.n	80009ec <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x206>
 800090a:	|  |  |  |  |  |  |  |  |     |  |  \-> 4640      	mov	r0, r8
 800090c:	|  |  |  |  |  |  |  |  |     |  |      b008      	add	sp, #32
 800090e:	|  |  |  |  |  |  |  |  |     |  |      e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000912:	|  |  |  |  |  |  |  |  |     |  |      bdf0      	pop	{r4, r5, r6, r7, pc}
 8000914:	|  |  |  |  |  |  |  |  |     \--|----> 68eb      	ldr	r3, [r5, #12]
 8000916:	|  |  |  |  |  |  |  |  |        |      4620      	mov	r0, r4
 8000918:	|  |  |  |  |  |  |  |  |        |      4651      	mov	r1, sl
 800091a:	|  |  |  |  |  |  |  |  |        |      4632      	mov	r2, r6
 800091c:	|  |  |  |  |  |  |  |  |        |      b008      	add	sp, #32
 800091e:	|  |  |  |  |  |  |  |  |        |      e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000922:	|  |  |  |  |  |  |  |  |        |      e8bd 40f0 	ldmia.w	sp!, {r4, r5, r6, r7, lr}
 8000926:	|  |  |  |  |  |  |  |  |        |      4718      	bx	r3
 8000928:	|  |  |  |  |  |  |  \--|--------|----> 2000      	movs	r0, #0
 800092a:	|  |  |  +--|--|--|-----|--------|----- e044      	b.n	80009b6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1d0>
 800092c:	|  |  |  |  |  |  |     \--------|----> e9cd 3502 	strd	r3, r5, [sp, #8]
 8000930:	|  |  |  |  |  |  |              |      f026 0303 	bic.w	r3, r6, #3
 8000934:	|  |  |  |  |  |  |              |      9404      	str	r4, [sp, #16]
 8000936:	|  |  |  |  |  |  |              |      2000      	movs	r0, #0
 8000938:	|  |  |  |  |  |  |              |      e9cd b606 	strd	fp, r6, [sp, #24]
 800093c:	|  |  |  |  |  |  |              |      46f3      	mov	fp, lr
 800093e:	|  |  |  |  |  |  |              |      4672      	mov	r2, lr
 8000940:	|  |  |  |  |  |  |              |  /-> f992 5002 	ldrsb.w	r5, [r2, #2]
 8000944:	|  |  |  |  |  |  |              |  |   f992 1003 	ldrsb.w	r1, [r2, #3]
 8000948:	|  |  |  |  |  |  |              |  |   f912 4b04 	ldrsb.w	r4, [r2], #4
 800094c:	|  |  |  |  |  |  |              |  |   f912 6c03 	ldrsb.w	r6, [r2, #-3]
 8000950:	|  |  |  |  |  |  |              |  |   f114 0f41 	cmn.w	r4, #65	; 0x41
 8000954:	|  |  |  |  |  |  |              |  |   bfc8      	it	gt
 8000956:	|  |  |  |  |  |  |              |  |   3001      	addgt	r0, #1
 8000958:	|  |  |  |  |  |  |              |  |   f116 0f41 	cmn.w	r6, #65	; 0x41
 800095c:	|  |  |  |  |  |  |              |  |   bfc8      	it	gt
 800095e:	|  |  |  |  |  |  |              |  |   3001      	addgt	r0, #1
 8000960:	|  |  |  |  |  |  |              |  |   f115 0f41 	cmn.w	r5, #65	; 0x41
 8000964:	|  |  |  |  |  |  |              |  |   bfc8      	it	gt
 8000966:	|  |  |  |  |  |  |              |  |   3001      	addgt	r0, #1
 8000968:	|  |  |  |  |  |  |              |  |   f111 0f41 	cmn.w	r1, #65	; 0x41
 800096c:	|  |  |  |  |  |  |              |  |   bfc8      	it	gt
 800096e:	|  |  |  |  |  |  |              |  |   3001      	addgt	r0, #1
 8000970:	|  |  |  |  |  |  |              |  |   3b04      	subs	r3, #4
 8000972:	|  |  |  |  |  |  |              |  \-- d1e5      	bne.n	8000940 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x15a>
 8000974:	|  |  |  |  |  |  |              |      46de      	mov	lr, fp
 8000976:	|  |  |  |  |  |  |              |      e9dd 5403 	ldrd	r5, r4, [sp, #12]
 800097a:	|  |  |  |  |  |  |              |      e9dd b606 	ldrd	fp, r6, [sp, #24]
 800097e:	|  |  |  |  |  |  |              |      9b02      	ldr	r3, [sp, #8]
 8000980:	|  |  |  |  |  |  |              |      f1bc 0f00 	cmp.w	ip, #0
 8000984:	|  |  |  +--|--|--|--------------|----- d017      	beq.n	80009b6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1d0>
 8000986:	|  |  |  |  |  |  \--------------|----> f992 1000 	ldrsb.w	r1, [r2]
 800098a:	|  |  |  |  |  |                 |      f111 0f41 	cmn.w	r1, #65	; 0x41
 800098e:	|  |  |  |  |  |                 |      bfc8      	it	gt
 8000990:	|  |  |  |  |  |                 |      3001      	addgt	r0, #1
 8000992:	|  |  |  |  |  |                 |      f1bc 0f01 	cmp.w	ip, #1
 8000996:	|  |  |  +--|--|-----------------|----- d00e      	beq.n	80009b6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1d0>
 8000998:	|  |  |  |  |  |                 |      f992 1001 	ldrsb.w	r1, [r2, #1]
 800099c:	|  |  |  |  |  |                 |      f111 0f41 	cmn.w	r1, #65	; 0x41
 80009a0:	|  |  |  |  |  |                 |      bfc8      	it	gt
 80009a2:	|  |  |  |  |  |                 |      3001      	addgt	r0, #1
 80009a4:	|  |  |  |  |  |                 |      f1bc 0f02 	cmp.w	ip, #2
 80009a8:	|  |  |  +--|--|-----------------|----- d005      	beq.n	80009b6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x1d0>
 80009aa:	|  |  |  |  |  |                 |      f992 1002 	ldrsb.w	r1, [r2, #2]
 80009ae:	|  |  |  |  |  |                 |      f111 0f41 	cmn.w	r1, #65	; 0x41
 80009b2:	|  |  |  |  |  |                 |      bfc8      	it	gt
 80009b4:	|  |  |  |  |  |                 |      3001      	addgt	r0, #1
 80009b6:	|  |  |  \--|--|-----------------|----> 4480      	add	r8, r0
 80009b8:	|  |  |     |  |                 |      68a8      	ldr	r0, [r5, #8]
 80009ba:	|  |  |     |  |                 |      2800      	cmp	r0, #0
 80009bc:	|  |  |     |  \-----------------|----- f47f af4c 	bne.w	8000858 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x72>
 80009c0:	|  |  |     \--------------------|----> 46a0      	mov	r8, r4
 80009c2:	|  |  |                          |      e9d5 4500 	ldrd	r4, r5, [r5]
 80009c6:	|  |  |                          |      4629      	mov	r1, r5
 80009c8:	|  |  |                          |      465a      	mov	r2, fp
 80009ca:	|  |  |                          |      4673      	mov	r3, lr
 80009cc:	|  |  |                          |      9600      	str	r6, [sp, #0]
 80009ce:	|  |  |                          |      4620      	mov	r0, r4
 80009d0:	|  |  |                          |      f000 f872 	bl	8000ab8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE>
 80009d4:	|  |  |                          |      2800      	cmp	r0, #0
 80009d6:	|  |  |                 /--------|----- d14f      	bne.n	8000a78 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x292>
 80009d8:	|  |  |                 |        |      68eb      	ldr	r3, [r5, #12]
 80009da:	|  |  |                 |        |      4620      	mov	r0, r4
 80009dc:	|  |  |                 |        |      4651      	mov	r1, sl
 80009de:	|  |  |                 |        |      4642      	mov	r2, r8
 80009e0:	|  |  |                 |        |      b008      	add	sp, #32
 80009e2:	|  |  |                 |        |      e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 80009e6:	|  |  |                 |        |      e8bd 40f0 	ldmia.w	sp!, {r4, r5, r6, r7, lr}
 80009ea:	|  |  |                 |        |      4718      	bx	r3
 80009ec:	|  |  |                 |        \----> ebab 0006 	sub.w	r0, fp, r6
 80009f0:	|  |  |                 |               46a0      	mov	r8, r4
 80009f2:	|  |  |                 |               1c44      	adds	r4, r0, #1
 80009f4:	|  |  |                 |           /-> 3c01      	subs	r4, #1
 80009f6:	|  |  |                 |  /--------|-- d039      	beq.n	8000a6c <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x286>
 80009f8:	|  |  |                 |  |        |   692a      	ldr	r2, [r5, #16]
 80009fa:	|  |  |                 |  |        |   4650      	mov	r0, sl
 80009fc:	|  |  |                 |  |        |   2130      	movs	r1, #48	; 0x30
 80009fe:	|  |  |                 |  |        |   4790      	blx	r2
 8000a00:	|  |  |                 |  |        |   2800      	cmp	r0, #0
 8000a02:	|  |  |                 |  |        \-- d0f7      	beq.n	80009f4 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x20e>
 8000a04:	|  |  |                 +--|----------- e038      	b.n	8000a78 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x292>
 8000a06:	\--|--|-----------------|--|----------> 460c      	mov	r4, r1
 8000a08:	   |  +-----------------|--|----------- e002      	b.n	8000a10 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x22a>
 8000a0a:	   \--|-----------------|--|----------> 0848      	lsrs	r0, r1, #1
 8000a0c:	      |                 |  |            3101      	adds	r1, #1
 8000a0e:	      |                 |  |            084c      	lsrs	r4, r1, #1
 8000a10:	      \-----------------|--|----------> e9d5 8a00 	ldrd	r8, sl, [r5]
 8000a14:	                        |  |            69ee      	ldr	r6, [r5, #28]
 8000a16:	                        |  |            1c45      	adds	r5, r0, #1
 8000a18:	                        |  |        /-> 3d01      	subs	r5, #1
 8000a1a:	                        |  |     /--|-- d007      	beq.n	8000a2c <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x246>
 8000a1c:	                        |  |     |  |   f8da 2010 	ldr.w	r2, [sl, #16]
 8000a20:	                        |  |     |  |   4640      	mov	r0, r8
 8000a22:	                        |  |     |  |   4631      	mov	r1, r6
 8000a24:	                        |  |     |  |   4790      	blx	r2
 8000a26:	                        |  |     |  |   2800      	cmp	r0, #0
 8000a28:	                        |  |     |  \-- d0f6      	beq.n	8000a18 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x232>
 8000a2a:	                        +--|-----|----- e025      	b.n	8000a78 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x292>
 8000a2c:	                        |  |     \----> f5b6 1f88 	cmp.w	r6, #1114112	; 0x110000
 8000a30:	                        +--|----------- d022      	beq.n	8000a78 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x292>
 8000a32:	                        |  |            9807      	ldr	r0, [sp, #28]
 8000a34:	                        |  |            4651      	mov	r1, sl
 8000a36:	                        |  |            9000      	str	r0, [sp, #0]
 8000a38:	                        |  |            4640      	mov	r0, r8
 8000a3a:	                        |  |            9a06      	ldr	r2, [sp, #24]
 8000a3c:	                        |  |            465b      	mov	r3, fp
 8000a3e:	                        |  |            f000 f83b 	bl	8000ab8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE>
 8000a42:	                        +--|----------- b9c8      	cbnz	r0, 8000a78 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x292>
 8000a44:	                        |  |            f8da 300c 	ldr.w	r3, [sl, #12]
 8000a48:	                        |  |            4640      	mov	r0, r8
 8000a4a:	                        |  |            e9dd 2104 	ldrd	r2, r1, [sp, #16]
 8000a4e:	                        |  |            4798      	blx	r3
 8000a50:	                        +--|----------- b990      	cbnz	r0, 8000a78 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x292>
 8000a52:	                        |  |            2500      	movs	r5, #0
 8000a54:	                        |  |        /-> 42ac      	cmp	r4, r5
 8000a56:	                        |  |  /-----|-- d023      	beq.n	8000aa0 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x2ba>
 8000a58:	                        |  |  |     |   f8da 2010 	ldr.w	r2, [sl, #16]
 8000a5c:	                        |  |  |     |   4640      	mov	r0, r8
 8000a5e:	                        |  |  |     |   4631      	mov	r1, r6
 8000a60:	                        |  |  |     |   4790      	blx	r2
 8000a62:	                        |  |  |     |   3501      	adds	r5, #1
 8000a64:	                        |  |  |     |   2800      	cmp	r0, #0
 8000a66:	                        |  |  |     \-- d0f5      	beq.n	8000a54 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x26e>
 8000a68:	                        |  |  |         1e68      	subs	r0, r5, #1
 8000a6a:	                        |  |  |  /----- e01a      	b.n	8000aa2 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x2bc>
 8000a6c:	                        |  \--|--|----> 68eb      	ldr	r3, [r5, #12]
 8000a6e:	                        |     |  |      4650      	mov	r0, sl
 8000a70:	                        |     |  |      9905      	ldr	r1, [sp, #20]
 8000a72:	                        |     |  |      4642      	mov	r2, r8
 8000a74:	                        |     |  |      4798      	blx	r3
 8000a76:	                        |     |  |  /-- b130      	cbz	r0, 8000a86 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE+0x2a0>
 8000a78:	                        \-----|--|--|-> f04f 0801 	mov.w	r8, #1
 8000a7c:	                              |  |  |   4640      	mov	r0, r8
 8000a7e:	                              |  |  |   b008      	add	sp, #32
 8000a80:	                              |  |  |   e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000a84:	                              |  |  |   bdf0      	pop	{r4, r5, r6, r7, pc}
 8000a86:	                              |  |  \-> 9803      	ldr	r0, [sp, #12]
 8000a88:	                              |  |      f04f 0800 	mov.w	r8, #0
 8000a8c:	                              |  |      9901      	ldr	r1, [sp, #4]
 8000a8e:	                              |  |      f880 1020 	strb.w	r1, [r0, #32]
 8000a92:	                              |  |      9902      	ldr	r1, [sp, #8]
 8000a94:	                              |  |      61c1      	str	r1, [r0, #28]
 8000a96:	                              |  |      4640      	mov	r0, r8
 8000a98:	                              |  |      b008      	add	sp, #32
 8000a9a:	                              |  |      e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000a9e:	                              |  |      bdf0      	pop	{r4, r5, r6, r7, pc}
 8000aa0:	                              \--|----> 4620      	mov	r0, r4
 8000aa2:	                                 \----> f04f 0800 	mov.w	r8, #0
 8000aa6:	                                        42a0      	cmp	r0, r4
 8000aa8:	                                        bf38      	it	cc
 8000aaa:	                                        f04f 0801 	movcc.w	r8, #1
 8000aae:	                                        4640      	mov	r0, r8
 8000ab0:	                                        b008      	add	sp, #32
 8000ab2:	                                        e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000ab6:	                                        bdf0      	pop	{r4, r5, r6, r7, pc}

08000ab8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE>:
 8000ab8:	       b5f0      	push	{r4, r5, r6, r7, lr}
 8000aba:	       af03      	add	r7, sp, #12
 8000abc:	       f84d 8d04 	str.w	r8, [sp, #-4]!
 8000ac0:	       f8d7 8008 	ldr.w	r8, [r7, #8]
 8000ac4:	       461c      	mov	r4, r3
 8000ac6:	       460d      	mov	r5, r1
 8000ac8:	       4606      	mov	r6, r0
 8000aca:	       f5b2 1f88 	cmp.w	r2, #1114112	; 0x110000
 8000ace:	   /-- d008      	beq.n	8000ae2 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE+0x2a>
 8000ad0:	   |   692b      	ldr	r3, [r5, #16]
 8000ad2:	   |   4630      	mov	r0, r6
 8000ad4:	   |   4611      	mov	r1, r2
 8000ad6:	   |   4798      	blx	r3
 8000ad8:	   +-- b118      	cbz	r0, 8000ae2 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE+0x2a>
 8000ada:	   |   2001      	movs	r0, #1
 8000adc:	   |   f85d 8b04 	ldr.w	r8, [sp], #4
 8000ae0:	   |   bdf0      	pop	{r4, r5, r6, r7, pc}
 8000ae2:	/--\-X b144      	cbz	r4, 8000af6 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9893618a3bfbcabbE+0x3e>
 8000ae4:	|      68eb      	ldr	r3, [r5, #12]
 8000ae6:	|      4630      	mov	r0, r6
 8000ae8:	|      4621      	mov	r1, r4
 8000aea:	|      4642      	mov	r2, r8
 8000aec:	|      f85d 8b04 	ldr.w	r8, [sp], #4
 8000af0:	|      e8bd 40f0 	ldmia.w	sp!, {r4, r5, r6, r7, lr}
 8000af4:	|      4718      	bx	r3
 8000af6:	\----> 2000      	movs	r0, #0
 8000af8:	       f85d 8b04 	ldr.w	r8, [sp], #4
 8000afc:	       bdf0      	pop	{r4, r5, r6, r7, pc}

08000afe <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E>:
 8000afe:	                b5f0      	push	{r4, r5, r6, r7, lr}
 8000b00:	                af03      	add	r7, sp, #12
 8000b02:	                e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 8000b06:	                4602      	mov	r2, r0
 8000b08:	                3003      	adds	r0, #3
 8000b0a:	                f020 0003 	bic.w	r0, r0, #3
 8000b0e:	                eba0 0a02 	sub.w	sl, r0, r2
 8000b12:	                458a      	cmp	sl, r1
 8000b14:	                bf98      	it	ls
 8000b16:	                f1ba 0f04 	cmpls.w	sl, #4
 8000b1a:	         /----- d90b      	bls.n	8000b34 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x36>
 8000b1c:	      /--|--/-X b131      	cbz	r1, 8000b2c <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x2e>
 8000b1e:	      |  |  |   1e48      	subs	r0, r1, #1
 8000b20:	      |  |  |   f001 0c03 	and.w	ip, r1, #3
 8000b24:	      |  |  |   2803      	cmp	r0, #3
 8000b26:	   /--|--|--|-- d210      	bcs.n	8000b4a <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x4c>
 8000b28:	   |  |  |  |   2000      	movs	r0, #0
 8000b2a:	/--|--|--|--|-- e02b      	b.n	8000b84 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x86>
 8000b2c:	|  |  |  |  \-> 2000      	movs	r0, #0
 8000b2e:	|  |  |  |      e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000b32:	|  |  |  |      bdf0      	pop	{r4, r5, r6, r7, pc}
 8000b34:	|  |  |  \----> eba1 0c0a 	sub.w	ip, r1, sl
 8000b38:	|  |  |         f1bc 0f04 	cmp.w	ip, #4
 8000b3c:	|  |  \-------- d3ee      	bcc.n	8000b1c <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x1e>
 8000b3e:	|  |            f00c 0803 	and.w	r8, ip, #3
 8000b42:	|  |            4290      	cmp	r0, r2
 8000b44:	|  |     /----- d13c      	bne.n	8000bc0 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xc2>
 8000b46:	|  |     |      2000      	movs	r0, #0
 8000b48:	|  |  /--|----- e07c      	b.n	8000c44 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x146>
 8000b4a:	|  \--|--|----> f021 0103 	bic.w	r1, r1, #3
 8000b4e:	|     |  |      2000      	movs	r0, #0
 8000b50:	|     |  |  /-> f992 6002 	ldrsb.w	r6, [r2, #2]
 8000b54:	|     |  |  |   f992 5003 	ldrsb.w	r5, [r2, #3]
 8000b58:	|     |  |  |   f912 4b04 	ldrsb.w	r4, [r2], #4
 8000b5c:	|     |  |  |   f912 3c03 	ldrsb.w	r3, [r2, #-3]
 8000b60:	|     |  |  |   f114 0f41 	cmn.w	r4, #65	; 0x41
 8000b64:	|     |  |  |   bfc8      	it	gt
 8000b66:	|     |  |  |   3001      	addgt	r0, #1
 8000b68:	|     |  |  |   f113 0f41 	cmn.w	r3, #65	; 0x41
 8000b6c:	|     |  |  |   bfc8      	it	gt
 8000b6e:	|     |  |  |   3001      	addgt	r0, #1
 8000b70:	|     |  |  |   f116 0f41 	cmn.w	r6, #65	; 0x41
 8000b74:	|     |  |  |   bfc8      	it	gt
 8000b76:	|     |  |  |   3001      	addgt	r0, #1
 8000b78:	|     |  |  |   f115 0f41 	cmn.w	r5, #65	; 0x41
 8000b7c:	|     |  |  |   bfc8      	it	gt
 8000b7e:	|     |  |  |   3001      	addgt	r0, #1
 8000b80:	|     |  |  |   3904      	subs	r1, #4
 8000b82:	|     |  |  \-- d1e5      	bne.n	8000b50 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x52>
 8000b84:	\-----|--|----> f1bc 0f00 	cmp.w	ip, #0
 8000b88:	   /--|--|----- d017      	beq.n	8000bba <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xbc>
 8000b8a:	   |  |  |      f992 1000 	ldrsb.w	r1, [r2]
 8000b8e:	   |  |  |      f111 0f41 	cmn.w	r1, #65	; 0x41
 8000b92:	   |  |  |      bfc8      	it	gt
 8000b94:	   |  |  |      3001      	addgt	r0, #1
 8000b96:	   |  |  |      f1bc 0f01 	cmp.w	ip, #1
 8000b9a:	   +--|--|----- d00e      	beq.n	8000bba <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xbc>
 8000b9c:	   |  |  |      f992 1001 	ldrsb.w	r1, [r2, #1]
 8000ba0:	   |  |  |      f111 0f41 	cmn.w	r1, #65	; 0x41
 8000ba4:	   |  |  |      bfc8      	it	gt
 8000ba6:	   |  |  |      3001      	addgt	r0, #1
 8000ba8:	   |  |  |      f1bc 0f02 	cmp.w	ip, #2
 8000bac:	   +--|--|----- d005      	beq.n	8000bba <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xbc>
 8000bae:	   |  |  |      f992 1002 	ldrsb.w	r1, [r2, #2]
 8000bb2:	   |  |  |      f111 0f41 	cmn.w	r1, #65	; 0x41
 8000bb6:	   |  |  |      bfc8      	it	gt
 8000bb8:	   |  |  |      3001      	addgt	r0, #1
 8000bba:	   >--|--|----> e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000bbe:	   |  |  |      bdf0      	pop	{r4, r5, r6, r7, pc}
 8000bc0:	   |  |  \----> 43d5      	mvns	r5, r2
 8000bc2:	   |  |         4428      	add	r0, r5
 8000bc4:	   |  |         f00a 0e03 	and.w	lr, sl, #3
 8000bc8:	   |  |         2803      	cmp	r0, #3
 8000bca:	   |  |     /-- d202      	bcs.n	8000bd2 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xd4>
 8000bcc:	   |  |     |   2000      	movs	r0, #0
 8000bce:	   |  |     |   4614      	mov	r4, r2
 8000bd0:	   |  |  /--|-- e01d      	b.n	8000c0e <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x110>
 8000bd2:	   |  |  |  \-> f02a 0503 	bic.w	r5, sl, #3
 8000bd6:	   |  |  |      2000      	movs	r0, #0
 8000bd8:	   |  |  |      4614      	mov	r4, r2
 8000bda:	   |  |  |  /-> f994 6002 	ldrsb.w	r6, [r4, #2]
 8000bde:	   |  |  |  |   f994 b003 	ldrsb.w	fp, [r4, #3]
 8000be2:	   |  |  |  |   f914 3b04 	ldrsb.w	r3, [r4], #4
 8000be6:	   |  |  |  |   f914 1c03 	ldrsb.w	r1, [r4, #-3]
 8000bea:	   |  |  |  |   f113 0f41 	cmn.w	r3, #65	; 0x41
 8000bee:	   |  |  |  |   bfc8      	it	gt
 8000bf0:	   |  |  |  |   3001      	addgt	r0, #1
 8000bf2:	   |  |  |  |   f111 0f41 	cmn.w	r1, #65	; 0x41
 8000bf6:	   |  |  |  |   bfc8      	it	gt
 8000bf8:	   |  |  |  |   3001      	addgt	r0, #1
 8000bfa:	   |  |  |  |   f116 0f41 	cmn.w	r6, #65	; 0x41
 8000bfe:	   |  |  |  |   bfc8      	it	gt
 8000c00:	   |  |  |  |   3001      	addgt	r0, #1
 8000c02:	   |  |  |  |   f11b 0f41 	cmn.w	fp, #65	; 0x41
 8000c06:	   |  |  |  |   bfc8      	it	gt
 8000c08:	   |  |  |  |   3001      	addgt	r0, #1
 8000c0a:	   |  |  |  |   3d04      	subs	r5, #4
 8000c0c:	   |  |  |  \-- d1e5      	bne.n	8000bda <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xdc>
 8000c0e:	   |  |  \----> f1be 0f00 	cmp.w	lr, #0
 8000c12:	   |  +-------- d017      	beq.n	8000c44 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x146>
 8000c14:	   |  |         f994 1000 	ldrsb.w	r1, [r4]
 8000c18:	   |  |         f111 0f41 	cmn.w	r1, #65	; 0x41
 8000c1c:	   |  |         bfc8      	it	gt
 8000c1e:	   |  |         3001      	addgt	r0, #1
 8000c20:	   |  |         f1be 0f01 	cmp.w	lr, #1
 8000c24:	   |  +-------- d00e      	beq.n	8000c44 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x146>
 8000c26:	   |  |         f994 1001 	ldrsb.w	r1, [r4, #1]
 8000c2a:	   |  |         f111 0f41 	cmn.w	r1, #65	; 0x41
 8000c2e:	   |  |         bfc8      	it	gt
 8000c30:	   |  |         3001      	addgt	r0, #1
 8000c32:	   |  |         f1be 0f02 	cmp.w	lr, #2
 8000c36:	   |  +-------- d005      	beq.n	8000c44 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x146>
 8000c38:	   |  |         f994 1002 	ldrsb.w	r1, [r4, #2]
 8000c3c:	   |  |         f111 0f41 	cmn.w	r1, #65	; 0x41
 8000c40:	   |  |         bfc8      	it	gt
 8000c42:	   |  |         3001      	addgt	r0, #1
 8000c44:	   |  \-------> eb02 040a 	add.w	r4, r2, sl
 8000c48:	   |            ea4f 0a9c 	mov.w	sl, ip, lsr #2
 8000c4c:	   |            2200      	movs	r2, #0
 8000c4e:	   |            f1b8 0f00 	cmp.w	r8, #0
 8000c52:	   |        /-- d01a      	beq.n	8000c8a <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x18c>
 8000c54:	   |        |   f02c 0103 	bic.w	r1, ip, #3
 8000c58:	   |        |   1863      	adds	r3, r4, r1
 8000c5a:	   |        |   f993 1000 	ldrsb.w	r1, [r3]
 8000c5e:	   |        |   f111 0f41 	cmn.w	r1, #65	; 0x41
 8000c62:	   |        |   bfc8      	it	gt
 8000c64:	   |        |   2201      	movgt	r2, #1
 8000c66:	   |        |   f1b8 0f01 	cmp.w	r8, #1
 8000c6a:	   |        +-- d00e      	beq.n	8000c8a <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x18c>
 8000c6c:	   |        |   f993 1001 	ldrsb.w	r1, [r3, #1]
 8000c70:	   |        |   f111 0f41 	cmn.w	r1, #65	; 0x41
 8000c74:	   |        |   bfc8      	it	gt
 8000c76:	   |        |   3201      	addgt	r2, #1
 8000c78:	   |        |   f1b8 0f02 	cmp.w	r8, #2
 8000c7c:	   |        +-- d005      	beq.n	8000c8a <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x18c>
 8000c7e:	   |        |   f993 1002 	ldrsb.w	r1, [r3, #2]
 8000c82:	   |        |   f111 0f41 	cmn.w	r1, #65	; 0x41
 8000c86:	   |        |   bfc8      	it	gt
 8000c88:	   |        |   3201      	addgt	r2, #1
 8000c8a:	   |        \-> 4410      	add	r0, r2
 8000c8c:	   |        /-- e00f      	b.n	8000cae <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x1b0>
 8000c8e:	   |  /-----|-> fa3f f18e 	uxtb16	r1, lr
 8000c92:	   |  |     |   fa3f f39e 	uxtb16	r3, lr, ror #8
 8000c96:	   |  |     |   4419      	add	r1, r3
 8000c98:	   |  |     |   ebaa 0a0b 	sub.w	sl, sl, fp
 8000c9c:	   |  |     |   eb0c 048b 	add.w	r4, ip, fp, lsl #2
 8000ca0:	   |  |     |   f01b 0203 	ands.w	r2, fp, #3
 8000ca4:	   |  |     |   eb01 4101 	add.w	r1, r1, r1, lsl #16
 8000ca8:	   |  |     |   eb00 4011 	add.w	r0, r0, r1, lsr #16
 8000cac:	   |  |  /--|-- d13e      	bne.n	8000d2c <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x22e>
 8000cae:	   |  |  |  \-> f1ba 0f00 	cmp.w	sl, #0
 8000cb2:	   \--|--|----- f43f af82 	beq.w	8000bba <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0xbc>
 8000cb6:	      |  |      f1ba 0fc0 	cmp.w	sl, #192	; 0xc0
 8000cba:	      |  |      46d3      	mov	fp, sl
 8000cbc:	      |  |      bf28      	it	cs
 8000cbe:	      |  |      f04f 0bc0 	movcs.w	fp, #192	; 0xc0
 8000cc2:	      |  |      f01b 01fc 	ands.w	r1, fp, #252	; 0xfc
 8000cc6:	      |  |      46a4      	mov	ip, r4
 8000cc8:	      |  |      f04f 0e00 	mov.w	lr, #0
 8000ccc:	      |  |      eb04 0881 	add.w	r8, r4, r1, lsl #2
 8000cd0:	      +--|----- d0dd      	beq.n	8000c8e <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x190>
 8000cd2:	      |  |      4664      	mov	r4, ip
 8000cd4:	      |  |  /-> 2c00      	cmp	r4, #0
 8000cd6:	      |  |  |   bf1f      	itttt	ne
 8000cd8:	      |  |  |   e9d4 6500 	ldrdne	r6, r5, [r4]
 8000cdc:	      |  |  |   e9d4 1202 	ldrdne	r1, r2, [r4, #8]
 8000ce0:	      |  |  |   43f3      	mvnne	r3, r6
 8000ce2:	      |  |  |   09db      	lsrne	r3, r3, #7
 8000ce4:	      |  |  |   bf1f      	itttt	ne
 8000ce6:	      |  |  |   ea43 1396 	orrne.w	r3, r3, r6, lsr #6
 8000cea:	      |  |  |   f023 33fe 	bicne.w	r3, r3, #4278124286	; 0xfefefefe
 8000cee:	      |  |  |   4473      	addne	r3, lr
 8000cf0:	      |  |  |   43ee      	mvnne	r6, r5
 8000cf2:	      |  |  |   bf1f      	itttt	ne
 8000cf4:	      |  |  |   09f6      	lsrne	r6, r6, #7
 8000cf6:	      |  |  |   ea46 1695 	orrne.w	r6, r6, r5, lsr #6
 8000cfa:	      |  |  |   f026 36fe 	bicne.w	r6, r6, #4278124286	; 0xfefefefe
 8000cfe:	      |  |  |   4433      	addne	r3, r6
 8000d00:	      |  |  |   bf1f      	itttt	ne
 8000d02:	      |  |  |   43ce      	mvnne	r6, r1
 8000d04:	      |  |  |   09f6      	lsrne	r6, r6, #7
 8000d06:	      |  |  |   ea46 1191 	orrne.w	r1, r6, r1, lsr #6
 8000d0a:	      |  |  |   f021 31fe 	bicne.w	r1, r1, #4278124286	; 0xfefefefe
 8000d0e:	      |  |  |   bf1f      	itttt	ne
 8000d10:	      |  |  |   4419      	addne	r1, r3
 8000d12:	      |  |  |   43d3      	mvnne	r3, r2
 8000d14:	      |  |  |   09db      	lsrne	r3, r3, #7
 8000d16:	      |  |  |   ea43 1292 	orrne.w	r2, r3, r2, lsr #6
 8000d1a:	      |  |  |   bf1f      	itttt	ne
 8000d1c:	      |  |  |   f022 32fe 	bicne.w	r2, r2, #4278124286	; 0xfefefefe
 8000d20:	      |  |  |   eb02 0e01 	addne.w	lr, r2, r1
 8000d24:	      |  |  |   3410      	addne	r4, #16
 8000d26:	      |  |  |   4544      	cmpne	r4, r8
 8000d28:	      |  |  \-- d1d4      	bne.n	8000cd4 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x1d6>
 8000d2a:	      \--|----- e7b0      	b.n	8000c8e <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x190>
 8000d2c:	         \----> f1bc 0f00 	cmp.w	ip, #0
 8000d30:	         /----- d00a      	beq.n	8000d48 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x24a>
 8000d32:	         |      1e51      	subs	r1, r2, #1
 8000d34:	         |      f021 4140 	bic.w	r1, r1, #3221225472	; 0xc0000000
 8000d38:	         |      1c4b      	adds	r3, r1, #1
 8000d3a:	         |      2903      	cmp	r1, #3
 8000d3c:	         |      f003 0c03 	and.w	ip, r3, #3
 8000d40:	         |  /-- d205      	bcs.n	8000d4e <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x250>
 8000d42:	         |  |   f04f 0e00 	mov.w	lr, #0
 8000d46:	      /--|--|-- e033      	b.n	8000db0 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x2b2>
 8000d48:	      |  \--|-> f04f 0e00 	mov.w	lr, #0
 8000d4c:	      |  /--|-- e058      	b.n	8000e00 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x302>
 8000d4e:	      |  |  \-> f023 0403 	bic.w	r4, r3, #3
 8000d52:	      |  |      f04f 0e00 	mov.w	lr, #0
 8000d56:	      |  |  /-> e9d8 1600 	ldrd	r1, r6, [r8]
 8000d5a:	      |  |  |   3c04      	subs	r4, #4
 8000d5c:	      |  |  |   e9d8 5202 	ldrd	r5, r2, [r8, #8]
 8000d60:	      |  |  |   f108 0810 	add.w	r8, r8, #16
 8000d64:	      |  |  |   ea6f 0301 	mvn.w	r3, r1
 8000d68:	      |  |  |   ea4f 13d3 	mov.w	r3, r3, lsr #7
 8000d6c:	      |  |  |   ea43 1191 	orr.w	r1, r3, r1, lsr #6
 8000d70:	      |  |  |   ea6f 0306 	mvn.w	r3, r6
 8000d74:	      |  |  |   f021 31fe 	bic.w	r1, r1, #4278124286	; 0xfefefefe
 8000d78:	      |  |  |   ea4f 13d3 	mov.w	r3, r3, lsr #7
 8000d7c:	      |  |  |   4471      	add	r1, lr
 8000d7e:	      |  |  |   ea43 1396 	orr.w	r3, r3, r6, lsr #6
 8000d82:	      |  |  |   f023 33fe 	bic.w	r3, r3, #4278124286	; 0xfefefefe
 8000d86:	      |  |  |   4419      	add	r1, r3
 8000d88:	      |  |  |   ea6f 0305 	mvn.w	r3, r5
 8000d8c:	      |  |  |   ea4f 13d3 	mov.w	r3, r3, lsr #7
 8000d90:	      |  |  |   ea43 1395 	orr.w	r3, r3, r5, lsr #6
 8000d94:	      |  |  |   f023 33fe 	bic.w	r3, r3, #4278124286	; 0xfefefefe
 8000d98:	      |  |  |   4419      	add	r1, r3
 8000d9a:	      |  |  |   ea6f 0302 	mvn.w	r3, r2
 8000d9e:	      |  |  |   ea4f 13d3 	mov.w	r3, r3, lsr #7
 8000da2:	      |  |  |   ea43 1292 	orr.w	r2, r3, r2, lsr #6
 8000da6:	      |  |  |   f022 32fe 	bic.w	r2, r2, #4278124286	; 0xfefefefe
 8000daa:	      |  |  |   eb02 0e01 	add.w	lr, r2, r1
 8000dae:	      |  |  \-- d1d2      	bne.n	8000d56 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x258>
 8000db0:	      \--|----> f1bc 0f00 	cmp.w	ip, #0
 8000db4:	         +----- d024      	beq.n	8000e00 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x302>
 8000db6:	         |      f8d8 1000 	ldr.w	r1, [r8]
 8000dba:	         |      f1bc 0f01 	cmp.w	ip, #1
 8000dbe:	         |      ea6f 0601 	mvn.w	r6, r1
 8000dc2:	         |      ea4f 16d6 	mov.w	r6, r6, lsr #7
 8000dc6:	         |      ea46 1191 	orr.w	r1, r6, r1, lsr #6
 8000dca:	         |      f021 31fe 	bic.w	r1, r1, #4278124286	; 0xfefefefe
 8000dce:	         |      448e      	add	lr, r1
 8000dd0:	         +----- d016      	beq.n	8000e00 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x302>
 8000dd2:	         |      f8d8 1004 	ldr.w	r1, [r8, #4]
 8000dd6:	         |      f1bc 0f02 	cmp.w	ip, #2
 8000dda:	         |      ea6f 0601 	mvn.w	r6, r1
 8000dde:	         |      ea4f 16d6 	mov.w	r6, r6, lsr #7
 8000de2:	         |      ea46 1191 	orr.w	r1, r6, r1, lsr #6
 8000de6:	         |      f021 31fe 	bic.w	r1, r1, #4278124286	; 0xfefefefe
 8000dea:	         |      448e      	add	lr, r1
 8000dec:	         +----- d008      	beq.n	8000e00 <_ZN4core3str5count14do_count_chars17h44204b1f6175f776E+0x302>
 8000dee:	         |      f8d8 1008 	ldr.w	r1, [r8, #8]
 8000df2:	         |      43ca      	mvns	r2, r1
 8000df4:	         |      09d2      	lsrs	r2, r2, #7
 8000df6:	         |      ea42 1191 	orr.w	r1, r2, r1, lsr #6
 8000dfa:	         |      f021 31fe 	bic.w	r1, r1, #4278124286	; 0xfefefefe
 8000dfe:	         |      448e      	add	lr, r1
 8000e00:	         \----> fa3f f18e 	uxtb16	r1, lr
 8000e04:	                fa3f f29e 	uxtb16	r2, lr, ror #8
 8000e08:	                4411      	add	r1, r2
 8000e0a:	                eb01 4101 	add.w	r1, r1, r1, lsl #16
 8000e0e:	                eb00 4011 	add.w	r0, r0, r1, lsr #16
 8000e12:	                e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000e16:	                bdf0      	pop	{r4, r5, r6, r7, pc}

08000e18 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E>:
 8000e18:	                   b5f0      	push	{r4, r5, r6, r7, lr}
 8000e1a:	                   af03      	add	r7, sp, #12
 8000e1c:	                   e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 8000e20:	                   b08e      	sub	sp, #56	; 0x38
 8000e22:	                   6804      	ldr	r4, [r0, #0]
 8000e24:	                   468c      	mov	ip, r1
 8000e26:	                   f242 7310 	movw	r3, #10000	; 0x2710
 8000e2a:	                   2c00      	cmp	r4, #0
 8000e2c:	                   4625      	mov	r5, r4
 8000e2e:	                   bf48      	it	mi
 8000e30:	                   4265      	negmi	r5, r4
 8000e32:	                   f240 21fc 	movw	r1, #764	; 0x2fc
 8000e36:	                   f2c0 0100 	movt	r1, #0
 8000e3a:	                   429d      	cmp	r5, r3
 8000e3c:	                   4479      	add	r1, pc
 8000e3e:	/----------------- d341      	bcc.n	8000ec4 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0xac>
 8000e40:	|                  f24e 0aff 	movw	sl, #57599	; 0xe0ff
 8000e44:	|                  f1a7 083f 	sub.w	r8, r7, #63	; 0x3f
 8000e48:	|                  2000      	movs	r0, #0
 8000e4a:	|                  f241 4b7b 	movw	fp, #5243	; 0x147b
 8000e4e:	|                  f04f 0e64 	mov.w	lr, #100	; 0x64
 8000e52:	|                  f2c0 5af5 	movt	sl, #1525	; 0x5f5
 8000e56:	|                  e9cd 4c02 	strd	r4, ip, [sp, #8]
 8000e5a:	|              /-> f241 7259 	movw	r2, #5977	; 0x1759
 8000e5e:	|              |   eb08 0400 	add.w	r4, r8, r0
 8000e62:	|              |   f2cd 12b7 	movt	r2, #53687	; 0xd1b7
 8000e66:	|              |   3804      	subs	r0, #4
 8000e68:	|              |   fba5 2c02 	umull	r2, ip, r5, r2
 8000e6c:	|              |   4555      	cmp	r5, sl
 8000e6e:	|              |   ea4f 325c 	mov.w	r2, ip, lsr #13
 8000e72:	|              |   fb02 5613 	mls	r6, r2, r3, r5
 8000e76:	|              |   4615      	mov	r5, r2
 8000e78:	|              |   b2b3      	uxth	r3, r6
 8000e7a:	|              |   ea4f 0393 	mov.w	r3, r3, lsr #2
 8000e7e:	|              |   fb03 f30b 	mul.w	r3, r3, fp
 8000e82:	|              |   ea4f 4353 	mov.w	r3, r3, lsr #17
 8000e86:	|              |   fb03 661e 	mls	r6, r3, lr, r6
 8000e8a:	|              |   f831 3013 	ldrh.w	r3, [r1, r3, lsl #1]
 8000e8e:	|              |   f8a4 3023 	strh.w	r3, [r4, #35]	; 0x23
 8000e92:	|              |   f242 7310 	movw	r3, #10000	; 0x2710
 8000e96:	|              |   b2b6      	uxth	r6, r6
 8000e98:	|              |   f831 6016 	ldrh.w	r6, [r1, r6, lsl #1]
 8000e9c:	|              |   f8a4 6025 	strh.w	r6, [r4, #37]	; 0x25
 8000ea0:	|              \-- d8db      	bhi.n	8000e5a <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0x42>
 8000ea2:	|                  e9dd 4c02 	ldrd	r4, ip, [sp, #8]
 8000ea6:	|                  f100 0327 	add.w	r3, r0, #39	; 0x27
 8000eaa:	|                  4615      	mov	r5, r2
 8000eac:	|                  2d63      	cmp	r5, #99	; 0x63
 8000eae:	|           /----- d80c      	bhi.n	8000eca <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0xb2>
 8000eb0:	|           |  /-> 4628      	mov	r0, r5
 8000eb2:	|           |  |   280a      	cmp	r0, #10
 8000eb4:	|  /--------|--|-- d31b      	bcc.n	8000eee <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0xd6>
 8000eb6:	|  |     /--|--|-> f831 0010 	ldrh.w	r0, [r1, r0, lsl #1]
 8000eba:	|  |     |  |  |   3b02      	subs	r3, #2
 8000ebc:	|  |     |  |  |   f1a7 013f 	sub.w	r1, r7, #63	; 0x3f
 8000ec0:	|  |     |  |  |   52c8      	strh	r0, [r1, r3]
 8000ec2:	|  |  /--|--|--|-- e019      	b.n	8000ef8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0xe0>
 8000ec4:	\--|--|--|--|--|-> 2327      	movs	r3, #39	; 0x27
 8000ec6:	   |  |  |  |  |   2d63      	cmp	r5, #99	; 0x63
 8000ec8:	   |  |  |  |  \-- d9f2      	bls.n	8000eb0 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0x98>
 8000eca:	   |  |  |  \----> b2a8      	uxth	r0, r5
 8000ecc:	   |  |  |         f241 427b 	movw	r2, #5243	; 0x147b
 8000ed0:	   |  |  |         0880      	lsrs	r0, r0, #2
 8000ed2:	   |  |  |         3b02      	subs	r3, #2
 8000ed4:	   |  |  |         4350      	muls	r0, r2
 8000ed6:	   |  |  |         2264      	movs	r2, #100	; 0x64
 8000ed8:	   |  |  |         f1a7 063f 	sub.w	r6, r7, #63	; 0x3f
 8000edc:	   |  |  |         0c40      	lsrs	r0, r0, #17
 8000ede:	   |  |  |         fb00 5212 	mls	r2, r0, r2, r5
 8000ee2:	   |  |  |         b292      	uxth	r2, r2
 8000ee4:	   |  |  |         f831 2012 	ldrh.w	r2, [r1, r2, lsl #1]
 8000ee8:	   |  |  |         52f2      	strh	r2, [r6, r3]
 8000eea:	   |  |  |         280a      	cmp	r0, #10
 8000eec:	   |  |  \-------- d2e3      	bcs.n	8000eb6 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hdd7bd15d96f960c3E+0x9e>
 8000eee:	   \--|----------> 3b01      	subs	r3, #1
 8000ef0:	      |            f1a7 013f 	sub.w	r1, r7, #63	; 0x3f
 8000ef4:	      |            3030      	adds	r0, #48	; 0x30
 8000ef6:	      |            54c8      	strb	r0, [r1, r3]
 8000ef8:	      \----------> f240 2228 	movw	r2, #552	; 0x228
 8000efc:	                   f1c3 0027 	rsb	r0, r3, #39	; 0x27
 8000f00:	                   f2c0 0200 	movt	r2, #0
 8000f04:	                   f1a7 013f 	sub.w	r1, r7, #63	; 0x3f
 8000f08:	                   4419      	add	r1, r3
 8000f0a:	                   e9cd 1000 	strd	r1, r0, [sp]
 8000f0e:	                   43e0      	mvns	r0, r4
 8000f10:	                   447a      	add	r2, pc
 8000f12:	                   0fc1      	lsrs	r1, r0, #31
 8000f14:	                   4660      	mov	r0, ip
 8000f16:	                   2300      	movs	r3, #0
 8000f18:	                   f7ff fc65 	bl	80007e6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE>
 8000f1c:	                   b00e      	add	sp, #56	; 0x38
 8000f1e:	                   e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8000f22:	                   bdf0      	pop	{r4, r5, r6, r7, pc}

08000f24 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE>:
 8000f24:	                   b5f0      	push	{r4, r5, r6, r7, lr}
 8000f26:	                   af03      	add	r7, sp, #12
 8000f28:	                   e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 8000f2c:	                   b08e      	sub	sp, #56	; 0x38
 8000f2e:	                   6804      	ldr	r4, [r0, #0]
 8000f30:	                   f240 10fc 	movw	r0, #508	; 0x1fc
 8000f34:	                   f2c0 0000 	movt	r0, #0
 8000f38:	                   f242 7310 	movw	r3, #10000	; 0x2710
 8000f3c:	                   4478      	add	r0, pc
 8000f3e:	                   460d      	mov	r5, r1
 8000f40:	                   429c      	cmp	r4, r3
 8000f42:	/----------------- d341      	bcc.n	8000fc8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0xa4>
 8000f44:	|                  f24e 08ff 	movw	r8, #57599	; 0xe0ff
 8000f48:	|                  f1a7 0e3f 	sub.w	lr, r7, #63	; 0x3f
 8000f4c:	|                  2100      	movs	r1, #0
 8000f4e:	|                  f241 4a7b 	movw	sl, #5243	; 0x147b
 8000f52:	|                  f04f 0b64 	mov.w	fp, #100	; 0x64
 8000f56:	|                  f2c0 58f5 	movt	r8, #1525	; 0x5f5
 8000f5a:	|                  9503      	str	r5, [sp, #12]
 8000f5c:	|              /-> f241 7259 	movw	r2, #5977	; 0x1759
 8000f60:	|              |   4544      	cmp	r4, r8
 8000f62:	|              |   f2cd 12b7 	movt	r2, #53687	; 0xd1b7
 8000f66:	|              |   fba4 2502 	umull	r2, r5, r4, r2
 8000f6a:	|              |   ea4f 3255 	mov.w	r2, r5, lsr #13
 8000f6e:	|              |   fb02 4513 	mls	r5, r2, r3, r4
 8000f72:	|              |   eb0e 0301 	add.w	r3, lr, r1
 8000f76:	|              |   f1a1 0104 	sub.w	r1, r1, #4
 8000f7a:	|              |   4614      	mov	r4, r2
 8000f7c:	|              |   fa1f fc85 	uxth.w	ip, r5
 8000f80:	|              |   ea4f 069c 	mov.w	r6, ip, lsr #2
 8000f84:	|              |   fb06 f60a 	mul.w	r6, r6, sl
 8000f88:	|              |   ea4f 4656 	mov.w	r6, r6, lsr #17
 8000f8c:	|              |   fb06 551b 	mls	r5, r6, fp, r5
 8000f90:	|              |   f830 6016 	ldrh.w	r6, [r0, r6, lsl #1]
 8000f94:	|              |   f8a3 6023 	strh.w	r6, [r3, #35]	; 0x23
 8000f98:	|              |   b2ad      	uxth	r5, r5
 8000f9a:	|              |   f830 5015 	ldrh.w	r5, [r0, r5, lsl #1]
 8000f9e:	|              |   f8a3 5025 	strh.w	r5, [r3, #37]	; 0x25
 8000fa2:	|              |   f242 7310 	movw	r3, #10000	; 0x2710
 8000fa6:	|              \-- d8d9      	bhi.n	8000f5c <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0x38>
 8000fa8:	|                  9d03      	ldr	r5, [sp, #12]
 8000faa:	|                  f101 0327 	add.w	r3, r1, #39	; 0x27
 8000fae:	|                  4614      	mov	r4, r2
 8000fb0:	|                  2c63      	cmp	r4, #99	; 0x63
 8000fb2:	|           /----- d80c      	bhi.n	8000fce <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0xaa>
 8000fb4:	|           |  /-> 4621      	mov	r1, r4
 8000fb6:	|           |  |   290a      	cmp	r1, #10
 8000fb8:	|  /--------|--|-- d31b      	bcc.n	8000ff2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0xce>
 8000fba:	|  |     /--|--|-> f830 0011 	ldrh.w	r0, [r0, r1, lsl #1]
 8000fbe:	|  |     |  |  |   3b02      	subs	r3, #2
 8000fc0:	|  |     |  |  |   f1a7 013f 	sub.w	r1, r7, #63	; 0x3f
 8000fc4:	|  |     |  |  |   52c8      	strh	r0, [r1, r3]
 8000fc6:	|  |  /--|--|--|-- e01a      	b.n	8000ffe <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0xda>
 8000fc8:	\--|--|--|--|--|-> 2327      	movs	r3, #39	; 0x27
 8000fca:	   |  |  |  |  |   2c63      	cmp	r4, #99	; 0x63
 8000fcc:	   |  |  |  |  \-- d9f2      	bls.n	8000fb4 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0x90>
 8000fce:	   |  |  |  \----> b2a1      	uxth	r1, r4
 8000fd0:	   |  |  |         f241 427b 	movw	r2, #5243	; 0x147b
 8000fd4:	   |  |  |         0889      	lsrs	r1, r1, #2
 8000fd6:	   |  |  |         3b02      	subs	r3, #2
 8000fd8:	   |  |  |         4351      	muls	r1, r2
 8000fda:	   |  |  |         2264      	movs	r2, #100	; 0x64
 8000fdc:	   |  |  |         f1a7 063f 	sub.w	r6, r7, #63	; 0x3f
 8000fe0:	   |  |  |         0c49      	lsrs	r1, r1, #17
 8000fe2:	   |  |  |         fb01 4212 	mls	r2, r1, r2, r4
 8000fe6:	   |  |  |         b292      	uxth	r2, r2
 8000fe8:	   |  |  |         f830 2012 	ldrh.w	r2, [r0, r2, lsl #1]
 8000fec:	   |  |  |         52f2      	strh	r2, [r6, r3]
 8000fee:	   |  |  |         290a      	cmp	r1, #10
 8000ff0:	   |  |  \-------- d2e3      	bcs.n	8000fba <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf7dd6a4e8aa131efE+0x96>
 8000ff2:	   \--|----------> 3b01      	subs	r3, #1
 8000ff4:	      |            f101 0030 	add.w	r0, r1, #48	; 0x30
 8000ff8:	      |            f1a7 013f 	sub.w	r1, r7, #63	; 0x3f
 8000ffc:	      |            54c8      	strb	r0, [r1, r3]
 8000ffe:	      \----------> f240 1228 	movw	r2, #296	; 0x128
 8001002:	                   f1a7 013f 	sub.w	r1, r7, #63	; 0x3f
 8001006:	                   f2c0 0200 	movt	r2, #0
 800100a:	                   f1c3 0027 	rsb	r0, r3, #39	; 0x27
 800100e:	                   4419      	add	r1, r3
 8001010:	                   447a      	add	r2, pc
 8001012:	                   e9cd 1000 	strd	r1, r0, [sp]
 8001016:	                   4628      	mov	r0, r5
 8001018:	                   2101      	movs	r1, #1
 800101a:	                   2300      	movs	r3, #0
 800101c:	                   f7ff fbe3 	bl	80007e6 <_ZN4core3fmt9Formatter12pad_integral17h5f473887ffb34b0fE>
 8001020:	                   b00e      	add	sp, #56	; 0x38
 8001022:	                   e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8001026:	                   bdf0      	pop	{r4, r5, r6, r7, pc}

08001028 <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E>:
 8001028:	          b5f0      	push	{r4, r5, r6, r7, lr}
 800102a:	          af03      	add	r7, sp, #12
 800102c:	          e92d 0d00 	stmdb	sp!, {r8, sl, fp}
 8001030:	/-------- b34a      	cbz	r2, 8001086 <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x5e>
 8001032:	|         f1a2 0e01 	sub.w	lr, r2, #1
 8001036:	|         f1c2 0c00 	rsb	ip, r2, #0
 800103a:	|         2300      	movs	r3, #0
 800103c:	|     /-> 5ccc      	ldrb	r4, [r1, r3]
 800103e:	|     |   5cc5      	ldrb	r5, [r0, r3]
 8001040:	|     |   42a5      	cmp	r5, r4
 8001042:	|  /--|-- d124      	bne.n	800108e <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x66>
 8001044:	|  |  |   459e      	cmp	lr, r3
 8001046:	+--|--|-- d01e      	beq.n	8001086 <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x5e>
 8001048:	|  |  |   eb01 0803 	add.w	r8, r1, r3
 800104c:	|  |  |   eb00 0a03 	add.w	sl, r0, r3
 8001050:	|  |  |   f898 4001 	ldrb.w	r4, [r8, #1]
 8001054:	|  |  |   f89a 5001 	ldrb.w	r5, [sl, #1]
 8001058:	|  |  |   42a5      	cmp	r5, r4
 800105a:	|  +--|-- d118      	bne.n	800108e <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x66>
 800105c:	|  |  |   eb0c 0603 	add.w	r6, ip, r3
 8001060:	|  |  |   1cb4      	adds	r4, r6, #2
 8001062:	+--|--|-- d010      	beq.n	8001086 <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x5e>
 8001064:	|  |  |   f898 4002 	ldrb.w	r4, [r8, #2]
 8001068:	|  |  |   f89a 5002 	ldrb.w	r5, [sl, #2]
 800106c:	|  |  |   42a5      	cmp	r5, r4
 800106e:	|  +--|-- d10e      	bne.n	800108e <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x66>
 8001070:	|  |  |   1cf4      	adds	r4, r6, #3
 8001072:	+--|--|-- d008      	beq.n	8001086 <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x5e>
 8001074:	|  |  |   f898 4003 	ldrb.w	r4, [r8, #3]
 8001078:	|  |  |   f89a 5003 	ldrb.w	r5, [sl, #3]
 800107c:	|  |  |   42a5      	cmp	r5, r4
 800107e:	|  +--|-- d106      	bne.n	800108e <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x66>
 8001080:	|  |  |   3304      	adds	r3, #4
 8001082:	|  |  |   429a      	cmp	r2, r3
 8001084:	|  |  \-- d1da      	bne.n	800103c <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E+0x14>
 8001086:	\--|----> 2000      	movs	r0, #0
 8001088:	   |      e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 800108c:	   |      bdf0      	pop	{r4, r5, r6, r7, pc}
 800108e:	   \----> 1b28      	subs	r0, r5, r4
 8001090:	          e8bd 0d00 	ldmia.w	sp!, {r8, sl, fp}
 8001094:	          bdf0      	pop	{r4, r5, r6, r7, pc}

08001096 <_ZN17compiler_builtins3mem4bcmp17h8b803fec55a7607aE>:
 8001096:	f7ff bfc7 	b.w	8001028 <_ZN17compiler_builtins3mem6memcmp17h2c4c3af3abcfeac7E>
 800109a:	dede      	udf	#222	; 0xde
