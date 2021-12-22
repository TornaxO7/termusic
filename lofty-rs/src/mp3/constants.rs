pub const BITRATES: [[[u32; 16]; 3]; 2] = [
	// Order:
	// Layer 1
	// Layer 2
	// Layer 3
	[
		// Version 1
		[
			0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0,
		],
		[
			0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0,
		],
		[
			0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0,
		],
	],
	[
		// Version 2/2.5
		[
			0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0,
		],
		[
			0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0,
		],
		[
			0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0,
		],
	],
];

pub const SAMPLE_RATES: [[u32; 3]; 3] = [
	[44100, 48000, 32000], // Version 1
	[22050, 24000, 16000], // Version 2
	[11025, 12000, 8000],  // Version 2.5
];

pub const SAMPLES: [[u16; 2]; 3] = [
	// Order:
	// [Version 1, Version 2/2.5]
	// Layer 1
	// Layer 2
	// Layer 3
	[384, 384],
	[1152, 1152],
	[1152, 576],
];

pub const SIDE_INFORMATION_SIZES: [[u32; 4]; 3] = [
	[32, 32, 32, 17], // Version 1
	[17, 17, 17, 9],  // Version 2
	[17, 17, 17, 9],  // Version 2.5
];

pub const PADDING_SIZES: [u8; 3] = [4, 1, 1];
