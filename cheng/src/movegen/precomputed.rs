use crate::board::BoardMask;

pub const KNIGHT_MOVES: [BoardMask; 64] = [
    BoardMask::const_from(0x0000000000020400),
    BoardMask::const_from(0x0000000000050800),
    BoardMask::const_from(0x00000000000A1100),
    BoardMask::const_from(0x0000000000142200),
    BoardMask::const_from(0x0000000000284400),
    BoardMask::const_from(0x0000000000508800),
    BoardMask::const_from(0x0000000000A01000),
    BoardMask::const_from(0x0000000000402000),
    BoardMask::const_from(0x0000000002040004),
    BoardMask::const_from(0x0000000005080008),
    BoardMask::const_from(0x000000000A110011),
    BoardMask::const_from(0x0000000014220022),
    BoardMask::const_from(0x0000000028440044),
    BoardMask::const_from(0x0000000050880088),
    BoardMask::const_from(0x00000000A0100010),
    BoardMask::const_from(0x0000000040200020),
    BoardMask::const_from(0x0000000204000402),
    BoardMask::const_from(0x0000000508000805),
    BoardMask::const_from(0x0000000A1100110A),
    BoardMask::const_from(0x0000001422002214),
    BoardMask::const_from(0x0000002844004428),
    BoardMask::const_from(0x0000005088008850),
    BoardMask::const_from(0x000000A0100010A0),
    BoardMask::const_from(0x0000004020002040),
    BoardMask::const_from(0x0000020400040200),
    BoardMask::const_from(0x0000050800080500),
    BoardMask::const_from(0x00000A1100110A00),
    BoardMask::const_from(0x0000142200221400),
    BoardMask::const_from(0x0000284400442800),
    BoardMask::const_from(0x0000508800885000),
    BoardMask::const_from(0x0000A0100010A000),
    BoardMask::const_from(0x0000402000204000),
    BoardMask::const_from(0x0002040004020000),
    BoardMask::const_from(0x0005080008050000),
    BoardMask::const_from(0x000A1100110A0000),
    BoardMask::const_from(0x0014220022140000),
    BoardMask::const_from(0x0028440044280000),
    BoardMask::const_from(0x0050880088500000),
    BoardMask::const_from(0x00A0100010A00000),
    BoardMask::const_from(0x0040200020400000),
    BoardMask::const_from(0x0204000402000000),
    BoardMask::const_from(0x0508000805000000),
    BoardMask::const_from(0x0A1100110A000000),
    BoardMask::const_from(0x1422002214000000),
    BoardMask::const_from(0x2844004428000000),
    BoardMask::const_from(0x5088008850000000),
    BoardMask::const_from(0xA0100010A0000000),
    BoardMask::const_from(0x4020002040000000),
    BoardMask::const_from(0x0400040200000000),
    BoardMask::const_from(0x0800080500000000),
    BoardMask::const_from(0x1100110A00000000),
    BoardMask::const_from(0x2200221400000000),
    BoardMask::const_from(0x4400442800000000),
    BoardMask::const_from(0x8800885000000000),
    BoardMask::const_from(0x100010A000000000),
    BoardMask::const_from(0x2000204000000000),
    BoardMask::const_from(0x0004020000000000),
    BoardMask::const_from(0x0008050000000000),
    BoardMask::const_from(0x00110A0000000000),
    BoardMask::const_from(0x0022140000000000),
    BoardMask::const_from(0x0044280000000000),
    BoardMask::const_from(0x0088500000000000),
    BoardMask::const_from(0x0010A00000000000),
    BoardMask::const_from(0x0020400000000000),
];

pub const KING_MOVES: [BoardMask; 64] = [
    BoardMask::const_from(0x0000000000000302),
    BoardMask::const_from(0x0000000000000705),
    BoardMask::const_from(0x0000000000000E0A),
    BoardMask::const_from(0x0000000000001C14),
    BoardMask::const_from(0x0000000000003828),
    BoardMask::const_from(0x0000000000007050),
    BoardMask::const_from(0x000000000000E0A0),
    BoardMask::const_from(0x000000000000C040),
    BoardMask::const_from(0x0000000000030203),
    BoardMask::const_from(0x0000000000070507),
    BoardMask::const_from(0x00000000000E0A0E),
    BoardMask::const_from(0x00000000001C141C),
    BoardMask::const_from(0x0000000000382838),
    BoardMask::const_from(0x0000000000705070),
    BoardMask::const_from(0x0000000000E0A0E0),
    BoardMask::const_from(0x0000000000C040C0),
    BoardMask::const_from(0x0000000003020300),
    BoardMask::const_from(0x0000000007050700),
    BoardMask::const_from(0x000000000E0A0E00),
    BoardMask::const_from(0x000000001C141C00),
    BoardMask::const_from(0x0000000038283800),
    BoardMask::const_from(0x0000000070507000),
    BoardMask::const_from(0x00000000E0A0E000),
    BoardMask::const_from(0x00000000C040C000),
    BoardMask::const_from(0x0000000302030000),
    BoardMask::const_from(0x0000000705070000),
    BoardMask::const_from(0x0000000E0A0E0000),
    BoardMask::const_from(0x0000001C141C0000),
    BoardMask::const_from(0x0000003828380000),
    BoardMask::const_from(0x0000007050700000),
    BoardMask::const_from(0x000000E0A0E00000),
    BoardMask::const_from(0x000000C040C00000),
    BoardMask::const_from(0x0000030203000000),
    BoardMask::const_from(0x0000070507000000),
    BoardMask::const_from(0x00000E0A0E000000),
    BoardMask::const_from(0x00001C141C000000),
    BoardMask::const_from(0x0000382838000000),
    BoardMask::const_from(0x0000705070000000),
    BoardMask::const_from(0x0000E0A0E0000000),
    BoardMask::const_from(0x0000C040C0000000),
    BoardMask::const_from(0x0003020300000000),
    BoardMask::const_from(0x0007050700000000),
    BoardMask::const_from(0x000E0A0E00000000),
    BoardMask::const_from(0x001C141C00000000),
    BoardMask::const_from(0x0038283800000000),
    BoardMask::const_from(0x0070507000000000),
    BoardMask::const_from(0x00E0A0E000000000),
    BoardMask::const_from(0x00C040C000000000),
    BoardMask::const_from(0x0302030000000000),
    BoardMask::const_from(0x0705070000000000),
    BoardMask::const_from(0x0E0A0E0000000000),
    BoardMask::const_from(0x1C141C0000000000),
    BoardMask::const_from(0x3828380000000000),
    BoardMask::const_from(0x7050700000000000),
    BoardMask::const_from(0xE0A0E00000000000),
    BoardMask::const_from(0xC040C00000000000),
    BoardMask::const_from(0x0203000000000000),
    BoardMask::const_from(0x0507000000000000),
    BoardMask::const_from(0x0A0E000000000000),
    BoardMask::const_from(0x141C000000000000),
    BoardMask::const_from(0x2838000000000000),
    BoardMask::const_from(0x5070000000000000),
    BoardMask::const_from(0xA0E0000000000000),
    BoardMask::const_from(0x40C0000000000000),
];

pub const BISHOP_OCCUPANCY: [BoardMask; 64] = [
    BoardMask::const_from(0x0040201008040200),
    BoardMask::const_from(0x0000402010080400),
    BoardMask::const_from(0x0000004020100A00),
    BoardMask::const_from(0x0000000040221400),
    BoardMask::const_from(0x0000000002442800),
    BoardMask::const_from(0x0000000204085000),
    BoardMask::const_from(0x0000020408102000),
    BoardMask::const_from(0x0002040810204000),
    BoardMask::const_from(0x0020100804020000),
    BoardMask::const_from(0x0040201008040000),
    BoardMask::const_from(0x00004020100A0000),
    BoardMask::const_from(0x0000004022140000),
    BoardMask::const_from(0x0000000244280000),
    BoardMask::const_from(0x0000020408500000),
    BoardMask::const_from(0x0002040810200000),
    BoardMask::const_from(0x0004081020400000),
    BoardMask::const_from(0x0010080402000200),
    BoardMask::const_from(0x0020100804000400),
    BoardMask::const_from(0x004020100A000A00),
    BoardMask::const_from(0x0000402214001400),
    BoardMask::const_from(0x0000024428002800),
    BoardMask::const_from(0x0002040850005000),
    BoardMask::const_from(0x0004081020002000),
    BoardMask::const_from(0x0008102040004000),
    BoardMask::const_from(0x0008040200020400),
    BoardMask::const_from(0x0010080400040800),
    BoardMask::const_from(0x0020100A000A1000),
    BoardMask::const_from(0x0040221400142200),
    BoardMask::const_from(0x0002442800284400),
    BoardMask::const_from(0x0004085000500800),
    BoardMask::const_from(0x0008102000201000),
    BoardMask::const_from(0x0010204000402000),
    BoardMask::const_from(0x0004020002040800),
    BoardMask::const_from(0x0008040004081000),
    BoardMask::const_from(0x00100A000A102000),
    BoardMask::const_from(0x0022140014224000),
    BoardMask::const_from(0x0044280028440200),
    BoardMask::const_from(0x0008500050080400),
    BoardMask::const_from(0x0010200020100800),
    BoardMask::const_from(0x0020400040201000),
    BoardMask::const_from(0x0002000204081000),
    BoardMask::const_from(0x0004000408102000),
    BoardMask::const_from(0x000A000A10204000),
    BoardMask::const_from(0x0014001422400000),
    BoardMask::const_from(0x0028002844020000),
    BoardMask::const_from(0x0050005008040200),
    BoardMask::const_from(0x0020002010080400),
    BoardMask::const_from(0x0040004020100800),
    BoardMask::const_from(0x0000020408102000),
    BoardMask::const_from(0x0000040810204000),
    BoardMask::const_from(0x00000A1020400000),
    BoardMask::const_from(0x0000142240000000),
    BoardMask::const_from(0x0000284402000000),
    BoardMask::const_from(0x0000500804020000),
    BoardMask::const_from(0x0000201008040200),
    BoardMask::const_from(0x0000402010080400),
    BoardMask::const_from(0x0002040810204000),
    BoardMask::const_from(0x0004081020400000),
    BoardMask::const_from(0x000A102040000000),
    BoardMask::const_from(0x0014224000000000),
    BoardMask::const_from(0x0028440200000000),
    BoardMask::const_from(0x0050080402000000),
    BoardMask::const_from(0x0020100804020000),
    BoardMask::const_from(0x0040201008040200),
];

pub const BISHOP_MAGICS: [u64; 64] = [
    0x2082409014100446, // #0 collisions
    0x10101101A1040054, // #0 collisions
    0x2244024028212000, // #0 collisions
    0x0000450081B00001, // #0 collisions
    0x0000820100200049, // #0 collisions
    0x30041004440B0012, // #0 collisions
    0x0582208004084120, // #0 collisions
    0x018D090012004040, // #0 collisions
    0x2060404420010420, // #0 collisions
    0x01288200C0888018, // #0 collisions
    0x00490002404040A8, // #0 collisions
    0x008000022104000A, // #0 collisions
    0x0100010483001080, // #0 collisions
    0x0002402120084000, // #0 collisions
    0x0000280850012001, // #0 collisions
    0x10040220210C3312, // #0 collisions
    0xA2100C4014044420, // #0 collisions
    0x090A8008044200C2, // #0 collisions
    0x0080183100891004, // #0 collisions
    0x004400001021242A, // #0 collisions
    0x002010201A082A04, // #0 collisions
    0x0044005140042062, // #0 collisions
    0x0002224001810014, // #0 collisions
    0x0021888494040200, // #0 collisions
    0x0000016505101000, // #0 collisions
    0x0001054200200411, // #0 collisions
    0x00510042000C0020, // #0 collisions
    0x00002000C2588018, // #0 collisions
    0x400840080A018180, // #0 collisions
    0x4086400240140102, // #0 collisions
    0x8810804000084250, // #0 collisions
    0x0105000130026888, // #0 collisions
    0x0409000101501140, // #0 collisions
    0x0004120080100080, // #0 collisions
    0x0842004008001818, // #0 collisions
    0x0001150592002810, // #0 collisions
    0x085001004100AA42, // #0 collisions
    0x0010040080000080, // #0 collisions
    0x00180B1100824001, // #0 collisions
    0x108020A0020A4020, // #0 collisions
    0x06A008000A004141, // #0 collisions
    0x000000900C020042, // #0 collisions
    0x2400090000418800, // #0 collisions
    0x460800115C020A2B, // #0 collisions
    0x2040000418002201, // #0 collisions
    0x0000120800838300, // #0 collisions
    0x0048004016000812, // #0 collisions
    0x0000590204004410, // #0 collisions
    0x0240008208002021, // #0 collisions
    0x320100A01C00080C, // #0 collisions
    0x02080120008200D0, // #0 collisions
    0x0006000000210010, // #0 collisions
    0x0001440200040002, // #0 collisions
    0x0040000310008220, // #0 collisions
    0x8821020010062001, // #0 collisions
    0x8228000C00210408, // #0 collisions
    0x0000201002A06082, // #0 collisions
    0x0212030810041900, // #0 collisions
    0x1118420484100040, // #0 collisions
    0x1238200020800488, // #0 collisions
    0x08000023D4900040, // #0 collisions
    0x021040000A618411, // #0 collisions
    0x0928222400401004, // #0 collisions
    0x4500160800108080, // #0 collisions
];

pub const ROOK_OCCUPANCY: [BoardMask; 64] = [
    BoardMask::const_from(0x000101010101017E),
    BoardMask::const_from(0x000202020202027E),
    BoardMask::const_from(0x000404040404047E),
    BoardMask::const_from(0x000808080808087E),
    BoardMask::const_from(0x001010101010107E),
    BoardMask::const_from(0x002020202020207E),
    BoardMask::const_from(0x004040404040407E),
    BoardMask::const_from(0x008080808080807E),
    BoardMask::const_from(0x0001010101017F00),
    BoardMask::const_from(0x0002020202027C00),
    BoardMask::const_from(0x0004040404047A00),
    BoardMask::const_from(0x0008080808087600),
    BoardMask::const_from(0x0010101010106E00),
    BoardMask::const_from(0x0020202020205E00),
    BoardMask::const_from(0x0040404040403E00),
    BoardMask::const_from(0x008080808080FE00),
    BoardMask::const_from(0x00010101017F0100),
    BoardMask::const_from(0x00020202027C0200),
    BoardMask::const_from(0x00040404047A0400),
    BoardMask::const_from(0x0008080808760800),
    BoardMask::const_from(0x00101010106E1000),
    BoardMask::const_from(0x00202020205E2000),
    BoardMask::const_from(0x00404040403E4000),
    BoardMask::const_from(0x0080808080FE8000),
    BoardMask::const_from(0x000101017F010100),
    BoardMask::const_from(0x000202027C020200),
    BoardMask::const_from(0x000404047A040400),
    BoardMask::const_from(0x0008080876080800),
    BoardMask::const_from(0x001010106E101000),
    BoardMask::const_from(0x002020205E202000),
    BoardMask::const_from(0x004040403E404000),
    BoardMask::const_from(0x00808080FE808000),
    BoardMask::const_from(0x0001017F01010100),
    BoardMask::const_from(0x0002027C02020200),
    BoardMask::const_from(0x0004047A04040400),
    BoardMask::const_from(0x0008087608080800),
    BoardMask::const_from(0x0010106E10101000),
    BoardMask::const_from(0x0020205E20202000),
    BoardMask::const_from(0x0040403E40404000),
    BoardMask::const_from(0x008080FE80808000),
    BoardMask::const_from(0x00017F0101010100),
    BoardMask::const_from(0x00027C0202020200),
    BoardMask::const_from(0x00047A0404040400),
    BoardMask::const_from(0x0008760808080800),
    BoardMask::const_from(0x00106E1010101000),
    BoardMask::const_from(0x00205E2020202000),
    BoardMask::const_from(0x00403E4040404000),
    BoardMask::const_from(0x0080FE8080808000),
    BoardMask::const_from(0x007F010101010100),
    BoardMask::const_from(0x007C020202020200),
    BoardMask::const_from(0x007A040404040400),
    BoardMask::const_from(0x0076080808080800),
    BoardMask::const_from(0x006E101010101000),
    BoardMask::const_from(0x005E202020202000),
    BoardMask::const_from(0x003E404040404000),
    BoardMask::const_from(0x00FE808080808000),
    BoardMask::const_from(0x7E01010101010100),
    BoardMask::const_from(0x7E02020202020200),
    BoardMask::const_from(0x7E04040404040400),
    BoardMask::const_from(0x7E08080808080800),
    BoardMask::const_from(0x7E10101010101000),
    BoardMask::const_from(0x7E20202020202000),
    BoardMask::const_from(0x7E40404040404000),
    BoardMask::const_from(0x7E80808080808000),
];

pub const ROOK_MAGICS: [u64; 64] = [
    0x1020044148001020, // #0 collisions
    0x444800041200A140, // #0 collisions
    0x0144000800200140, // #0 collisions
    0x0002882204011000, // #0 collisions
    0x8050944008100040, // #0 collisions
    0x0214002002000401, // #0 collisions
    0x0102000019040024, // #0 collisions
    0x5001027008224480, // #0 collisions
    0x8400050280401004, // #0 collisions
    0x715240800E804004, // #0 collisions
    0x0100204090000410, // #0 collisions
    0x0812002492044100, // #0 collisions
    0x00018C0220104081, // #0 collisions
    0x1204402040A04002, // #0 collisions
    0x0000120500084008, // #0 collisions
    0x2001420088004405, // #0 collisions
    0x0800121001080024, // #0 collisions
    0x0282080040340A10, // #0 collisions
    0x0240222000A00922, // #0 collisions
    0x0028020518520400, // #0 collisions
    0x0200208100080110, // #0 collisions
    0x490000A001012004, // #0 collisions
    0x00000380100A0046, // #0 collisions
    0x040210004320014A, // #0 collisions
    0x4008082528013000, // #0 collisions
    0x0014202003022800, // #0 collisions
    0x8082020082802808, // #0 collisions
    0x01101800402442A0, // #0 collisions
    0x1002010048000221, // #0 collisions
    0x84000421100086C0, // #0 collisions
    0x040020200A000010, // #0 collisions
    0x0000010801540080, // #0 collisions
    0x3002008081211008, // #0 collisions
    0x1402820095020300, // #0 collisions
    0x2000110052029900, // #0 collisions
    0x0835120224111100, // #0 collisions
    0xD084024000400400, // #0 collisions
    0x0010020008400020, // #0 collisions
    0x003000A180200080, // #0 collisions
    0x2821010804820100, // #0 collisions
    0x0800048040001526, // #0 collisions
    0x1104010543090230, // #0 collisions
    0x0022000801004800, // #0 collisions
    0x0E08003021000200, // #0 collisions
    0x9900603420182040, // #0 collisions
    0x8000C14820220010, // #0 collisions
    0x0060101004860100, // #64 collisions
    0x0000082001005100, // #0 collisions
    0x0700100184000402, // #0 collisions
    0x400B040058068180, // #0 collisions
    0x0000C0A002048003, // #0 collisions
    0x0204003030080202, // #0 collisions
    0xC00500C003104040, // #0 collisions
    0x021489802000205A, // #0 collisions
    0x000018219050A400, // #0 collisions
    0x0020000900020044, // #0 collisions
    0x0A001C4A00114006, // #0 collisions
    0x82C490040802114A, // #0 collisions
    0x0800294124800602, // #0 collisions
    0x14219480A0100001, // #0 collisions
    0x0800054800140089, // #0 collisions
    0x0018842082002801, // #0 collisions
    0x0003020040841423, // #0 collisions
    0x401020541040200A, // #0 collisions
];

