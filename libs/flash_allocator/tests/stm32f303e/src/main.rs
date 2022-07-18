mod fake_flash;

#[cfg(test)]
mod tests {
    use std::fmt;
    use flash_allocator::flash::{FlashAllocator, FlashAllocatorImpl, FlashMethods, FlashPage};
    use crate::fake_flash::Flash;

    /*
        Used to get a formatter instance
    */
    pub struct Fmt<F>(pub F)
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result;
    impl<F> fmt::Debug for Fmt<F>
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            (self.0)(f)
        }
    }

    // Flash: 0x0800 0000 - 0x0807 FFFF
    // Size: 512Kb
    const START_ADDR: u32 = 0x0800_0000;
    const END_ADDR: u32 = 0x0807_FFFF;
    const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x80000 -> 2^19 -> 524288
    const BLOCK_SIZE: usize = 4096;
    const FLAG_SIZE: usize = 2;

    static FLASH_PAGES: [FlashPage; 256] = [
        FlashPage::new(0, 0x08000000, 2048),   // 0
        FlashPage::new(1, 0x08000800, 2048),   // 1
        FlashPage::new(2, 0x08001000, 2048),   // 2
        FlashPage::new(3, 0x08001800, 2048),   // 3
        FlashPage::new(4, 0x08002000, 2048),   // 4
        FlashPage::new(5, 0x08002800, 2048),   // 5
        FlashPage::new(6, 0x08003000, 2048),   // 6
        FlashPage::new(7, 0x08003800, 2048),   // 7
        FlashPage::new(8, 0x08004000, 2048),   // 8
        FlashPage::new(9, 0x08004800, 2048),   // 9
        FlashPage::new(10, 0x08005000, 2048),  // 10
        FlashPage::new(11, 0x08005800, 2048),  // 11
        FlashPage::new(12, 0x08006000, 2048),  // 12
        FlashPage::new(13, 0x08006800, 2048),  // 13
        FlashPage::new(14, 0x08007000, 2048),  // 14
        FlashPage::new(15, 0x08007800, 2048),  // 15
        FlashPage::new(16, 0x08008000, 2048),  // 16
        FlashPage::new(17, 0x08008800, 2048),  // 17
        FlashPage::new(18, 0x08009000, 2048),  // 18
        FlashPage::new(19, 0x08009800, 2048),  // 19
        FlashPage::new(20, 0x0800A000, 2048),  // 20
        FlashPage::new(21, 0x0800A800, 2048),  // 21
        FlashPage::new(22, 0x0800B000, 2048),  // 22
        FlashPage::new(23, 0x0800B800, 2048),  // 23
        FlashPage::new(24, 0x0800C000, 2048),  // 24
        FlashPage::new(25, 0x0800C800, 2048),  // 25
        FlashPage::new(26, 0x0800D000, 2048),  // 26
        FlashPage::new(27, 0x0800D800, 2048),  // 27
        FlashPage::new(28, 0x0800E000, 2048),  // 28
        FlashPage::new(29, 0x0800E800, 2048),  // 29
        FlashPage::new(30, 0x0800F000, 2048),  // 30
        FlashPage::new(31, 0x0800F800, 2048),  // 31
        FlashPage::new(32, 0x08010000, 2048),  // 32
        FlashPage::new(33, 0x08010800, 2048),  // 33
        FlashPage::new(34, 0x08011000, 2048),  // 34
        FlashPage::new(35, 0x08011800, 2048),  // 35
        FlashPage::new(36, 0x08012000, 2048),  // 36
        FlashPage::new(37, 0x08012800, 2048),  // 37
        FlashPage::new(38, 0x08013000, 2048),  // 38
        FlashPage::new(39, 0x08013800, 2048),  // 39
        FlashPage::new(40, 0x08014000, 2048),  // 40
        FlashPage::new(41, 0x08014800, 2048),  // 41
        FlashPage::new(42, 0x08015000, 2048),  // 42
        FlashPage::new(43, 0x08015800, 2048),  // 43
        FlashPage::new(44, 0x08016000, 2048),  // 44
        FlashPage::new(45, 0x08016800, 2048),  // 45
        FlashPage::new(46, 0x08017000, 2048),  // 46
        FlashPage::new(47, 0x08017800, 2048),  // 47
        FlashPage::new(48, 0x08018000, 2048),  // 48
        FlashPage::new(49, 0x08018800, 2048),  // 49
        FlashPage::new(50, 0x08019000, 2048),  // 50
        FlashPage::new(51, 0x08019800, 2048),  // 51
        FlashPage::new(52, 0x0801A000, 2048),  // 52
        FlashPage::new(53, 0x0801A800, 2048),  // 53
        FlashPage::new(54, 0x0801B000, 2048),  // 54
        FlashPage::new(55, 0x0801B800, 2048),  // 55
        FlashPage::new(56, 0x0801C000, 2048),  // 56
        FlashPage::new(57, 0x0801C800, 2048),  // 57
        FlashPage::new(58, 0x0801D000, 2048),  // 58
        FlashPage::new(59, 0x0801D800, 2048),  // 59
        FlashPage::new(60, 0x0801E000, 2048),  // 60
        FlashPage::new(61, 0x0801E800, 2048),  // 61
        FlashPage::new(62, 0x0801F000, 2048),  // 62
        FlashPage::new(63, 0x0801F800, 2048),  // 63
        FlashPage::new(64, 0x08020000, 2048),  // 64
        FlashPage::new(65, 0x08020800, 2048),  // 65
        FlashPage::new(66, 0x08021000, 2048),  // 66
        FlashPage::new(67, 0x08021800, 2048),  // 67
        FlashPage::new(68, 0x08022000, 2048),  // 68
        FlashPage::new(69, 0x08022800, 2048),  // 69
        FlashPage::new(70, 0x08023000, 2048),  // 70
        FlashPage::new(71, 0x08023800, 2048),  // 71
        FlashPage::new(72, 0x08024000, 2048),  // 72
        FlashPage::new(73, 0x08024800, 2048),  // 73
        FlashPage::new(74, 0x08025000, 2048),  // 74
        FlashPage::new(75, 0x08025800, 2048),  // 75
        FlashPage::new(76, 0x08026000, 2048),  // 76
        FlashPage::new(77, 0x08026800, 2048),  // 77
        FlashPage::new(78, 0x08027000, 2048),  // 78
        FlashPage::new(79, 0x08027800, 2048),  // 79
        FlashPage::new(80, 0x08028000, 2048),  // 80
        FlashPage::new(81, 0x08028800, 2048),  // 81
        FlashPage::new(82, 0x08029000, 2048),  // 82
        FlashPage::new(83, 0x08029800, 2048),  // 83
        FlashPage::new(84, 0x0802A000, 2048),  // 84
        FlashPage::new(85, 0x0802A800, 2048),  // 85
        FlashPage::new(86, 0x0802B000, 2048),  // 86
        FlashPage::new(87, 0x0802B800, 2048),  // 87
        FlashPage::new(88, 0x0802C000, 2048),  // 88
        FlashPage::new(89, 0x0802C800, 2048),  // 89
        FlashPage::new(90, 0x0802D000, 2048),  // 90
        FlashPage::new(91, 0x0802D800, 2048),  // 91
        FlashPage::new(92, 0x0802E000, 2048),  // 92
        FlashPage::new(93, 0x0802E800, 2048),  // 93
        FlashPage::new(94, 0x0802F000, 2048),  // 94
        FlashPage::new(95, 0x0802F800, 2048),  // 95
        FlashPage::new(96, 0x08030000, 2048),  // 96
        FlashPage::new(97, 0x08030800, 2048),  // 97
        FlashPage::new(98, 0x08031000, 2048),  // 98
        FlashPage::new(99, 0x08031800, 2048),  // 99
        FlashPage::new(100, 0x08032000, 2048), // 100
        FlashPage::new(101, 0x08032800, 2048), // 101
        FlashPage::new(102, 0x08033000, 2048), // 102
        FlashPage::new(103, 0x08033800, 2048), // 103
        FlashPage::new(104, 0x08034000, 2048), // 104
        FlashPage::new(105, 0x08034800, 2048), // 105
        FlashPage::new(106, 0x08035000, 2048), // 106
        FlashPage::new(107, 0x08035800, 2048), // 107
        FlashPage::new(108, 0x08036000, 2048), // 108
        FlashPage::new(109, 0x08036800, 2048), // 109
        FlashPage::new(110, 0x08037000, 2048), // 110
        FlashPage::new(111, 0x08037800, 2048), // 111
        FlashPage::new(112, 0x08038000, 2048), // 112
        FlashPage::new(113, 0x08038800, 2048), // 113
        FlashPage::new(114, 0x08039000, 2048), // 114
        FlashPage::new(115, 0x08039800, 2048), // 115
        FlashPage::new(116, 0x0803A000, 2048), // 116
        FlashPage::new(117, 0x0803A800, 2048), // 117
        FlashPage::new(118, 0x0803B000, 2048), // 118
        FlashPage::new(119, 0x0803B800, 2048), // 119
        FlashPage::new(120, 0x0803C000, 2048), // 120
        FlashPage::new(121, 0x0803C800, 2048), // 121
        FlashPage::new(122, 0x0803D000, 2048), // 122
        FlashPage::new(123, 0x0803D800, 2048), // 123
        FlashPage::new(124, 0x0803E000, 2048), // 124
        FlashPage::new(125, 0x0803E800, 2048), // 125
        FlashPage::new(126, 0x0803F000, 2048), // 126
        FlashPage::new(127, 0x0803F800, 2048), // 127
        FlashPage::new(128, 0x08040000, 2048), // 128
        FlashPage::new(129, 0x08040800, 2048), // 129
        FlashPage::new(130, 0x08041000, 2048), // 130
        FlashPage::new(131, 0x08041800, 2048), // 131
        FlashPage::new(132, 0x08042000, 2048), // 132
        FlashPage::new(133, 0x08042800, 2048), // 133
        FlashPage::new(134, 0x08043000, 2048), // 134
        FlashPage::new(135, 0x08043800, 2048), // 135
        FlashPage::new(136, 0x08044000, 2048), // 136
        FlashPage::new(137, 0x08044800, 2048), // 137
        FlashPage::new(138, 0x08045000, 2048), // 138
        FlashPage::new(139, 0x08045800, 2048), // 139
        FlashPage::new(140, 0x08046000, 2048), // 140
        FlashPage::new(141, 0x08046800, 2048), // 141
        FlashPage::new(142, 0x08047000, 2048), // 142
        FlashPage::new(143, 0x08047800, 2048), // 143
        FlashPage::new(144, 0x08048000, 2048), // 144
        FlashPage::new(145, 0x08048800, 2048), // 145
        FlashPage::new(146, 0x08049000, 2048), // 146
        FlashPage::new(147, 0x08049800, 2048), // 147
        FlashPage::new(148, 0x0804A000, 2048), // 148
        FlashPage::new(149, 0x0804A800, 2048), // 149
        FlashPage::new(150, 0x0804B000, 2048), // 150
        FlashPage::new(151, 0x0804B800, 2048), // 151
        FlashPage::new(152, 0x0804C000, 2048), // 152
        FlashPage::new(153, 0x0804C800, 2048), // 153
        FlashPage::new(154, 0x0804D000, 2048), // 154
        FlashPage::new(155, 0x0804D800, 2048), // 155
        FlashPage::new(156, 0x0804E000, 2048), // 156
        FlashPage::new(157, 0x0804E800, 2048), // 157
        FlashPage::new(158, 0x0804F000, 2048), // 158
        FlashPage::new(159, 0x0804F800, 2048), // 159
        FlashPage::new(160, 0x08050000, 2048), // 160
        FlashPage::new(161, 0x08050800, 2048), // 161
        FlashPage::new(162, 0x08051000, 2048), // 162
        FlashPage::new(163, 0x08051800, 2048), // 163
        FlashPage::new(164, 0x08052000, 2048), // 164
        FlashPage::new(165, 0x08052800, 2048), // 165
        FlashPage::new(166, 0x08053000, 2048), // 166
        FlashPage::new(167, 0x08053800, 2048), // 167
        FlashPage::new(168, 0x08054000, 2048), // 168
        FlashPage::new(169, 0x08054800, 2048), // 169
        FlashPage::new(170, 0x08055000, 2048), // 170
        FlashPage::new(171, 0x08055800, 2048), // 171
        FlashPage::new(172, 0x08056000, 2048), // 172
        FlashPage::new(173, 0x08056800, 2048), // 173
        FlashPage::new(174, 0x08057000, 2048), // 174
        FlashPage::new(175, 0x08057800, 2048), // 175
        FlashPage::new(176, 0x08058000, 2048), // 176
        FlashPage::new(177, 0x08058800, 2048), // 177
        FlashPage::new(178, 0x08059000, 2048), // 178
        FlashPage::new(179, 0x08059800, 2048), // 179
        FlashPage::new(180, 0x0805A000, 2048), // 180
        FlashPage::new(181, 0x0805A800, 2048), // 181
        FlashPage::new(182, 0x0805B000, 2048), // 182
        FlashPage::new(183, 0x0805B800, 2048), // 183
        FlashPage::new(184, 0x0805C000, 2048), // 184
        FlashPage::new(185, 0x0805C800, 2048), // 185
        FlashPage::new(186, 0x0805D000, 2048), // 186
        FlashPage::new(187, 0x0805D800, 2048), // 187
        FlashPage::new(188, 0x0805E000, 2048), // 188
        FlashPage::new(189, 0x0805E800, 2048), // 189
        FlashPage::new(190, 0x0805F000, 2048), // 190
        FlashPage::new(191, 0x0805F800, 2048), // 191
        FlashPage::new(192, 0x08060000, 2048), // 192
        FlashPage::new(193, 0x08060800, 2048), // 193
        FlashPage::new(194, 0x08061000, 2048), // 194
        FlashPage::new(195, 0x08061800, 2048), // 195
        FlashPage::new(196, 0x08062000, 2048), // 196
        FlashPage::new(197, 0x08062800, 2048), // 197
        FlashPage::new(198, 0x08063000, 2048), // 198
        FlashPage::new(199, 0x08063800, 2048), // 199
        FlashPage::new(200, 0x08064000, 2048), // 200
        FlashPage::new(201, 0x08064800, 2048), // 201
        FlashPage::new(202, 0x08065000, 2048), // 202
        FlashPage::new(203, 0x08065800, 2048), // 203
        FlashPage::new(204, 0x08066000, 2048), // 204
        FlashPage::new(205, 0x08066800, 2048), // 205
        FlashPage::new(206, 0x08067000, 2048), // 206
        FlashPage::new(207, 0x08067800, 2048), // 207
        FlashPage::new(208, 0x08068000, 2048), // 208
        FlashPage::new(209, 0x08068800, 2048), // 209
        FlashPage::new(210, 0x08069000, 2048), // 210
        FlashPage::new(211, 0x08069800, 2048), // 211
        FlashPage::new(212, 0x0806A000, 2048), // 212
        FlashPage::new(213, 0x0806A800, 2048), // 213
        FlashPage::new(214, 0x0806B000, 2048), // 214
        FlashPage::new(215, 0x0806B800, 2048), // 215
        FlashPage::new(216, 0x0806C000, 2048), // 216
        FlashPage::new(217, 0x0806C800, 2048), // 217
        FlashPage::new(218, 0x0806D000, 2048), // 218
        FlashPage::new(219, 0x0806D800, 2048), // 219
        FlashPage::new(220, 0x0806E000, 2048), // 220
        FlashPage::new(221, 0x0806E800, 2048), // 221
        FlashPage::new(222, 0x0806F000, 2048), // 222
        FlashPage::new(223, 0x0806F800, 2048), // 223
        FlashPage::new(224, 0x08070000, 2048), // 224
        FlashPage::new(225, 0x08070800, 2048), // 225
        FlashPage::new(226, 0x08071000, 2048), // 226
        FlashPage::new(227, 0x08071800, 2048), // 227
        FlashPage::new(228, 0x08072000, 2048), // 228
        FlashPage::new(229, 0x08072800, 2048), // 229
        FlashPage::new(230, 0x08073000, 2048), // 230
        FlashPage::new(231, 0x08073800, 2048), // 231
        FlashPage::new(232, 0x08074000, 2048), // 232
        FlashPage::new(233, 0x08074800, 2048), // 233
        FlashPage::new(234, 0x08075000, 2048), // 234
        FlashPage::new(235, 0x08075800, 2048), // 235
        FlashPage::new(236, 0x08076000, 2048), // 236
        FlashPage::new(237, 0x08076800, 2048), // 237
        FlashPage::new(238, 0x08077000, 2048), // 238
        FlashPage::new(239, 0x08077800, 2048), // 239
        FlashPage::new(240, 0x08078000, 2048), // 240
        FlashPage::new(241, 0x08078800, 2048), // 241
        FlashPage::new(242, 0x08079000, 2048), // 242
        FlashPage::new(243, 0x08079800, 2048), // 243
        FlashPage::new(244, 0x0807A000, 2048), // 244
        FlashPage::new(245, 0x0807A800, 2048), // 245
        FlashPage::new(246, 0x0807B000, 2048), // 246
        FlashPage::new(247, 0x0807B800, 2048), // 247
        FlashPage::new(248, 0x0807C000, 2048), // 248
        FlashPage::new(249, 0x0807C800, 2048), // 249
        FlashPage::new(250, 0x0807D000, 2048), // 250
        FlashPage::new(251, 0x0807D800, 2048), // 251
        FlashPage::new(252, 0x0807E000, 2048), // 252
        FlashPage::new(253, 0x0807E800, 2048), // 253
        FlashPage::new(254, 0x0807F000, 2048), // 254
        FlashPage::new(255, 0x0807F800, 2048), // 255
    ];

    fn init_stm32f303e<'a>(
        flash: &'a mut dyn FlashMethods<'a>,
        from_flash: bool,
    ) -> impl FlashAllocator<'a, FLAG_SIZE> {
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 128
        const NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1
        let bd;
        if !from_flash {
            bd = FlashAllocatorImpl::<
                START_ADDR,
                END_ADDR,
                BLOCK_SIZE,
                NUM_BLOCKS,
                NUM_SLOTS,
                FLAG_SIZE,
            >::new(flash);
        } else {
            bd = FlashAllocatorImpl::<
                START_ADDR,
                END_ADDR,
                BLOCK_SIZE,
                NUM_BLOCKS,
                NUM_SLOTS,
                FLAG_SIZE,
            >::from_flash(flash);
        }
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                FlashAllocatorImpl::<
                    START_ADDR,
                    END_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    NUM_SLOTS,
                    FLAG_SIZE,
                >,
            >()
        );
        return bd;
    }

    #[test]
    fn test() {
        let mut flash = Flash::new(SIZE, START_ADDR, &FLASH_PAGES);
        let mut flash_allocator = init_stm32f303e(&mut flash, false);
        // Allocation 1
        let alloc1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc1.get_base_address());
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Destroy allocator
        drop(flash_allocator);
        // Recreate from flash
        let mut flash_allocator_rec = init_stm32f303e(&mut flash, true);
        // Deallocate 1
        flash_allocator_rec.deallocate(alloc1.get_base_address()).unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Allocate 2
        let alloc2 = flash_allocator_rec.allocate(3 * BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc2.get_base_address());
        // Allocate 3
        let alloc3 = flash_allocator_rec.allocate(4 * BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc3.get_base_address());
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 2
        flash_allocator_rec.deallocate(alloc2.get_base_address()).unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 3
        flash_allocator_rec.deallocate(alloc3.get_base_address()).unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
    }
}

fn main() {}
