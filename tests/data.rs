pub const UNTRANSPONSED: [u32; 1024] = [
    1, 2, 2, 5, 8, 8, 8, 9, 9, 10, 11, 13, 13, 13, 15, 17, 18, 18, 19, 23, 25, 25, 27, 27, 27, 28,
    30, 30, 30, 31, 35, 36, 37, 39, 39, 40, 40, 41, 42, 43, 44, 45, 45, 48, 48, 49, 49, 50, 51, 51,
    51, 52, 53, 53, 54, 55, 55, 55, 55, 57, 58, 59, 60, 60, 61, 61, 61, 63, 64, 65, 65, 66, 67, 67,
    69, 71, 72, 73, 73, 74, 75, 78, 82, 82, 83, 84, 85, 87, 88, 89, 90, 90, 94, 97, 98, 100, 100,
    101, 101, 102, 104, 104, 105, 106, 106, 107, 107, 107, 112, 113, 114, 115, 116, 116, 118, 118,
    119, 120, 121, 121, 122, 124, 126, 128, 131, 133, 133, 134, 134, 134, 134, 134, 137, 139, 140,
    140, 140, 142, 144, 146, 146, 147, 147, 148, 149, 149, 151, 153, 154, 154, 154, 155, 156, 156,
    156, 159, 159, 162, 163, 163, 164, 165, 165, 167, 168, 169, 170, 170, 171, 172, 172, 174, 175,
    179, 180, 180, 183, 183, 186, 187, 188, 188, 189, 190, 190, 191, 191, 193, 194, 194, 196, 196,
    198, 198, 198, 199, 200, 202, 202, 203, 204, 206, 207, 207, 208, 209, 209, 211, 211, 211, 216,
    218, 219, 220, 220, 222, 223, 223, 223, 225, 226, 227, 228, 229, 230, 234, 237, 239, 240, 240,
    242, 244, 245, 246, 246, 246, 249, 249, 250, 250, 250, 252, 252, 254, 257, 258, 259, 259, 260,
    260, 261, 262, 262, 264, 265, 270, 271, 272, 273, 274, 275, 276, 276, 278, 279, 280, 281, 282,
    283, 284, 287, 288, 288, 289, 290, 292, 292, 294, 294, 295, 295, 296, 297, 297, 299, 300, 301,
    301, 301, 302, 302, 303, 304, 304, 304, 305, 306, 307, 308, 308, 308, 309, 310, 311, 311, 311,
    315, 315, 317, 319, 319, 320, 320, 322, 322, 323, 325, 327, 328, 329, 332, 333, 335, 335, 336,
    338, 340, 340, 341, 342, 343, 343, 344, 347, 348, 349, 350, 351, 352, 356, 358, 358, 358, 359,
    359, 359, 359, 360, 360, 360, 361, 362, 364, 365, 366, 366, 371, 375, 376, 376, 377, 378, 380,
    380, 382, 382, 384, 386, 388, 388, 389, 390, 390, 390, 390, 390, 391, 392, 397, 398, 398, 399,
    401, 401, 402, 403, 407, 409, 410, 410, 413, 414, 414, 417, 417, 417, 418, 422, 423, 424, 424,
    425, 427, 429, 430, 430, 431, 432, 432, 432, 433, 435, 435, 436, 436, 439, 439, 439, 441, 441,
    441, 442, 443, 444, 445, 446, 447, 447, 447, 448, 448, 453, 454, 456, 456, 457, 459, 460, 460,
    461, 463, 464, 464, 466, 466, 466, 468, 470, 470, 470, 471, 471, 473, 475, 475, 477, 478, 480,
    480, 481, 481, 481, 482, 486, 486, 487, 487, 488, 489, 489, 489, 489, 489, 491, 492, 492, 492,
    494, 495, 496, 497, 497, 498, 499, 501, 501, 502, 503, 504, 505, 506, 509, 511, 511, 511, 513,
    514, 514, 516, 517, 518, 519, 521, 521, 521, 522, 523, 524, 525, 526, 526, 526, 526, 527, 530,
    532, 533, 533, 534, 535, 536, 537, 537, 538, 538, 539, 539, 540, 540, 541, 541, 543, 544, 545,
    545, 545, 547, 548, 550, 553, 554, 555, 556, 557, 557, 557, 557, 558, 559, 560, 562, 562, 563,
    563, 565, 566, 566, 570, 571, 572, 573, 574, 574, 575, 576, 577, 577, 578, 580, 581, 581, 582,
    583, 583, 585, 586, 586, 586, 587, 587, 589, 589, 590, 591, 593, 593, 594, 594, 596, 597, 597,
    599, 599, 600, 600, 601, 601, 602, 602, 604, 605, 605, 606, 606, 607, 607, 611, 613, 613, 615,
    616, 616, 617, 617, 618, 619, 619, 619, 619, 620, 620, 624, 624, 625, 627, 628, 631, 631, 632,
    632, 634, 636, 636, 638, 638, 638, 639, 639, 639, 641, 642, 648, 648, 649, 650, 650, 651, 652,
    654, 655, 655, 655, 656, 656, 656, 657, 657, 658, 659, 660, 660, 662, 663, 663, 663, 664, 665,
    665, 666, 666, 667, 669, 674, 674, 675, 676, 678, 678, 680, 680, 681, 681, 681, 681, 681, 682,
    684, 685, 685, 688, 688, 689, 690, 691, 693, 693, 693, 694, 696, 697, 697, 698, 699, 699, 700,
    700, 701, 701, 701, 701, 702, 704, 706, 709, 709, 711, 712, 712, 713, 715, 716, 717, 717, 718,
    720, 722, 724, 724, 725, 727, 728, 729, 729, 729, 731, 733, 736, 737, 737, 737, 738, 738, 740,
    741, 741, 742, 743, 744, 746, 747, 748, 748, 749, 751, 751, 751, 753, 753, 754, 754, 754, 754,
    756, 757, 758, 762, 764, 764, 765, 767, 767, 768, 769, 769, 769, 769, 770, 771, 771, 771, 771,
    771, 774, 776, 776, 778, 778, 779, 779, 779, 782, 783, 785, 785, 785, 785, 785, 786, 786, 787,
    791, 793, 793, 794, 795, 796, 796, 797, 797, 797, 797, 799, 800, 801, 802, 802, 802, 805, 805,
    805, 806, 807, 807, 809, 809, 811, 811, 811, 812, 812, 813, 813, 814, 817, 819, 819, 819, 820,
    820, 821, 822, 822, 824, 825, 826, 827, 828, 828, 829, 829, 830, 831, 833, 834, 835, 838, 838,
    839, 839, 842, 846, 846, 847, 847, 847, 849, 849, 850, 851, 851, 851, 854, 855, 855, 856, 856,
    856, 857, 857, 859, 860, 861, 862, 863, 864, 865, 868, 869, 871, 872, 872, 873, 874, 875, 877,
    877, 879, 881, 883, 884, 892, 893, 893, 894, 894, 895, 895, 896, 896, 896, 897, 898, 900, 900,
    900, 900, 901, 902, 903, 903, 903, 903, 904, 904, 906, 906, 907, 907, 907, 907, 908, 908, 911,
    911, 911, 912, 913, 913, 914, 914, 914, 914, 914, 915, 916, 917, 917, 918, 919, 919, 920, 920,
    921, 921, 925, 928, 929, 931, 931, 931, 931, 932, 933, 934, 937, 938, 938, 938, 939, 941, 945,
    945, 949, 952, 952, 952, 953, 954, 954, 956, 956, 956, 957, 957, 959, 959, 961, 963, 963, 964,
    964, 965, 965, 966, 970, 972, 973, 976, 977, 978, 979, 980, 980, 982, 983, 984, 985, 985, 985,
    986, 986, 988, 989, 990, 990, 990, 991, 991, 994, 996, 997, 997, 997, 999,
];
pub const TRANSPOSED: [u32; 1024] = [
    1, 61, 134, 198, 271, 332, 402, 470, 526, 586, 642, 700, 765, 814, 879, 931, 37, 100, 164, 230,
    301, 364, 439, 497, 557, 613, 674, 736, 791, 847, 907, 965, 18, 75, 149, 211, 288, 350, 424,
    486, 540, 599, 657, 716, 776, 829, 900, 953, 51, 116, 183, 250, 311, 388, 454, 514, 573, 627,
    685, 749, 802, 860, 916, 985, 9, 67, 140, 204, 279, 341, 414, 478, 536, 593, 654, 706, 770,
    822, 894, 939, 44, 106, 171, 245, 306, 377, 445, 504, 563, 619, 681, 741, 797, 855, 913, 979,
    27, 88, 156, 223, 295, 359, 432, 489, 547, 605, 663, 725, 785, 838, 903, 959, 55, 122, 190,
    260, 320, 391, 463, 521, 580, 636, 693, 754, 809, 871, 921, 991, 2, 61, 134, 198, 272, 333,
    403, 470, 527, 586, 648, 700, 767, 817, 881, 932, 39, 101, 165, 234, 302, 365, 439, 497, 557,
    615, 674, 737, 793, 849, 907, 966, 18, 78, 149, 211, 289, 351, 425, 487, 541, 600, 658, 717,
    778, 829, 900, 954, 51, 116, 183, 252, 311, 388, 456, 514, 574, 628, 688, 751, 805, 861, 917,
    986, 10, 67, 142, 206, 280, 342, 417, 480, 537, 593, 655, 709, 771, 822, 895, 941, 45, 107,
    172, 246, 307, 378, 446, 505, 563, 619, 681, 742, 797, 855, 913, 980, 28, 89, 156, 223, 296,
    359, 432, 491, 548, 605, 664, 727, 785, 839, 903, 959, 55, 124, 191, 260, 322, 392, 464, 522,
    581, 638, 694, 754, 811, 872, 921, 991, 2, 61, 134, 198, 273, 335, 407, 471, 530, 587, 648,
    701, 767, 819, 883, 933, 39, 101, 165, 237, 302, 366, 441, 498, 557, 616, 675, 737, 793, 849,
    908, 970, 19, 82, 151, 216, 290, 352, 427, 487, 541, 600, 659, 717, 778, 830, 900, 954, 51,
    118, 186, 252, 315, 389, 456, 516, 574, 631, 688, 751, 805, 862, 917, 986, 11, 69, 144, 207,
    281, 343, 417, 480, 537, 594, 655, 709, 771, 824, 895, 945, 45, 107, 172, 246, 308, 380, 447,
    506, 565, 619, 681, 743, 797, 856, 914, 980, 30, 90, 156, 223, 297, 359, 433, 492, 550, 606,
    665, 728, 785, 839, 904, 961, 55, 126, 191, 261, 322, 397, 464, 523, 581, 638, 696, 756, 811,
    872, 925, 994, 5, 63, 134, 199, 274, 335, 409, 471, 532, 587, 649, 701, 768, 819, 884, 934, 40,
    102, 167, 239, 303, 366, 441, 499, 558, 616, 676, 737, 794, 850, 908, 972, 23, 82, 153, 218,
    292, 356, 429, 488, 543, 601, 660, 718, 779, 831, 900, 956, 52, 118, 187, 254, 315, 390, 457,
    517, 575, 631, 689, 751, 805, 863, 918, 988, 13, 71, 146, 207, 282, 343, 417, 481, 538, 594,
    655, 711, 771, 825, 896, 945, 48, 107, 174, 246, 308, 380, 447, 509, 566, 620, 681, 744, 799,
    856, 914, 982, 30, 90, 159, 225, 297, 360, 435, 492, 553, 606, 665, 729, 785, 842, 904, 963,
    57, 128, 193, 262, 323, 398, 466, 524, 582, 638, 697, 757, 811, 873, 928, 996, 8, 64, 137, 200,
    275, 336, 410, 473, 533, 589, 650, 701, 769, 819, 892, 937, 40, 104, 168, 240, 304, 371, 441,
    501, 559, 617, 678, 738, 795, 851, 911, 973, 25, 83, 154, 219, 292, 358, 430, 489, 544, 601,
    660, 720, 779, 833, 901, 956, 53, 119, 188, 257, 317, 390, 459, 518, 576, 632, 690, 753, 806,
    864, 919, 989, 13, 72, 146, 208, 283, 344, 418, 481, 538, 596, 656, 712, 771, 826, 896, 949,
    48, 112, 175, 249, 308, 382, 447, 511, 566, 620, 681, 746, 800, 856, 914, 983, 30, 94, 159,
    226, 299, 360, 435, 492, 554, 607, 666, 729, 785, 846, 906, 963, 58, 131, 194, 262, 325, 398,
    466, 525, 583, 639, 697, 758, 812, 874, 929, 997, 8, 65, 139, 202, 276, 338, 410, 475, 533,
    589, 650, 701, 769, 820, 893, 938, 41, 104, 169, 240, 304, 375, 442, 501, 560, 617, 678, 738,
    796, 851, 911, 976, 25, 84, 154, 220, 294, 358, 430, 489, 545, 602, 662, 722, 779, 834, 902,
    956, 53, 120, 188, 258, 319, 390, 460, 519, 577, 632, 691, 753, 807, 865, 919, 990, 13, 73,
    147, 209, 284, 347, 422, 481, 539, 597, 656, 712, 771, 827, 896, 952, 49, 113, 179, 249, 309,
    382, 448, 511, 570, 624, 682, 747, 801, 857, 914, 984, 31, 97, 162, 227, 300, 360, 436, 494,
    555, 607, 666, 729, 786, 846, 906, 964, 59, 133, 194, 264, 327, 399, 466, 526, 583, 639, 698,
    762, 812, 875, 931, 997, 8, 65, 140, 202, 276, 340, 413, 475, 534, 590, 651, 702, 769, 820,
    893, 938, 42, 105, 170, 242, 304, 376, 443, 502, 562, 618, 680, 740, 796, 851, 911, 977, 27,
    85, 154, 220, 294, 358, 431, 489, 545, 602, 663, 724, 782, 835, 903, 957, 54, 121, 189, 259,
    319, 390, 460, 521, 577, 634, 693, 754, 807, 868, 920, 990, 15, 73, 147, 209, 287, 348, 423,
    482, 539, 597, 656, 713, 774, 828, 897, 952, 49, 114, 180, 250, 310, 384, 448, 511, 571, 624,
    684, 748, 802, 857, 914, 985, 35, 98, 163, 228, 301, 361, 436, 495, 556, 611, 667, 731, 786,
    847, 907, 964, 60, 133, 196, 265, 328, 401, 468, 526, 585, 639, 699, 764, 813, 877, 931, 997,
    9, 66, 140, 203, 278, 340, 414, 477, 535, 591, 652, 704, 769, 821, 894, 938, 43, 106, 170, 244,
    305, 376, 444, 503, 562, 619, 680, 741, 797, 854, 912, 978, 27, 87, 155, 222, 295, 359, 432,
    489, 545, 604, 663, 724, 783, 838, 903, 957, 55, 121, 190, 259, 320, 390, 461, 521, 578, 636,
    693, 754, 809, 869, 920, 990, 17, 74, 148, 211, 288, 349, 424, 486, 540, 599, 657, 715, 776,
    828, 898, 952, 50, 115, 180, 250, 311, 386, 453, 513, 572, 625, 685, 748, 802, 859, 915, 985,
    36, 100, 163, 229, 301, 362, 439, 496, 557, 613, 669, 733, 787, 847, 907, 965, 60, 134, 196,
    270, 329, 401, 470, 526, 586, 641, 699, 764, 813, 877, 931, 999,
];
pub const DELTA_ARR: [u32; 1024] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 2, 1, 1, 1, 3, 0, 0, 2, 1, 1, 1, 2, 0, 1, 0, 1, 1, 0,
    0, 1, 0, 1, 1, 1, 0, 1, 1, 2, 2, 2, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1,
    0, 1, 1, 1, 0, 0, 0, 0, 2, 1, 0, 1, 2, 0, 0, 2, 0, 1, 0, 1, 0, 1, 2, 0, 2, 0, 0, 0, 0, 2, 1, 1,
    1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 6, 0, 2, 3, 2, 1, 2, 1, 1, 4, 1, 1, 0, 0, 0, 2, 0, 1, 2, 2, 0, 1,
    0, 3, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 0, 1, 0, 0, 0, 2, 0, 0, 2, 0, 1, 1, 3, 2, 3, 1, 1, 1,
    1, 0, 2, 2, 1, 1, 3, 2, 1, 0, 1, 3, 1, 0, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    1, 1, 0, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0, 1, 0, 0, 0, 2, 1, 0, 2, 1, 1, 1, 1, 2, 1, 0, 2, 1, 0, 0,
    0, 0, 0, 0, 1, 2, 4, 1, 3, 1, 0, 1, 0, 2, 2, 1, 0, 0, 0, 3, 0, 1, 2, 1, 0, 1, 1, 0, 0, 0, 1, 4,
    1, 4, 2, 5, 1, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 2, 3, 0, 4, 1, 0, 2, 0, 3, 0, 0, 0, 1, 0, 0,
    1, 2, 2, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 2, 0, 4, 0, 0, 0, 0, 1, 2, 1, 1, 2, 0, 0, 1, 0, 1, 1, 0,
    2, 1, 0, 0, 1, 0, 1, 1, 2, 1, 1, 1, 0, 0, 1, 2, 0, 2, 0, 1, 0, 5, 0, 1, 0, 0, 2, 2, 0, 0, 4, 3,
    3, 2, 0, 1, 1, 0, 2, 0, 2, 0, 1, 0, 1, 0, 1, 1, 1, 1, 2, 2, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 2,
    4, 0, 2, 2, 2, 4, 2, 1, 2, 1, 1, 1, 1, 1, 0, 2, 1, 0, 1, 2, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 2,
    2, 2, 2, 0, 1, 0, 0, 1, 1, 0, 0, 2, 0, 1, 1, 0, 3, 0, 2, 0, 0, 0, 0, 3, 1, 1, 0, 1, 2, 0, 0, 2,
    0, 0, 3, 2, 0, 1, 2, 0, 3, 0, 0, 1, 0, 3, 0, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 0, 1, 1, 0, 1, 3, 2,
    3, 1, 3, 1, 1, 1, 1, 2, 1, 2, 1, 0, 1, 0, 8, 3, 0, 2, 1, 1, 1, 5, 0, 2, 1, 1, 2, 1, 1, 1, 3, 1,
    2, 1, 1, 1, 0, 2, 1, 1, 1, 0, 0, 2, 0, 2, 1, 0, 1, 1, 1, 3, 2, 0, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1,
    0, 1, 0, 1, 1, 1, 1, 0, 0, 2, 1, 1, 0, 1, 0, 4, 0, 5, 1, 3, 0, 2, 0, 2, 0, 0, 0, 2, 1, 0, 0, 1,
    0, 4, 0, 1, 2, 0, 0, 0, 1, 1, 1, 0, 0, 4, 2, 0, 1, 3, 1, 0, 2, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1,
    0, 1, 2, 2, 1, 2, 0, 2, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 4, 1, 0, 1, 0, 0, 0, 1, 0, 0, 3,
    0, 1, 0, 1, 2, 0, 0, 0, 1, 1, 2, 2, 0, 1, 1, 0, 0, 1, 0, 1, 2, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1,
    0, 1, 1, 1, 1, 3, 4, 0, 1, 1, 0, 0, 0, 1, 0, 3, 1, 1, 4, 0, 1, 0, 1, 0, 4, 4, 1, 1, 1, 1, 0, 1,
    1, 3, 3, 1, 1, 0, 1, 2, 1, 0, 0, 0, 1, 0, 0, 1, 1, 2, 0, 2, 2, 1, 0, 1, 0, 0, 1, 4, 0, 1, 2, 0,
    0, 0, 1, 0, 0, 2, 3, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 2, 0, 1, 1, 1, 2, 1, 2, 2, 0, 0, 0, 1,
    2, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 2, 3, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 2, 0, 2, 2, 1, 0, 3, 1, 0,
    2, 0, 0, 0, 3, 1, 1, 1, 0, 0, 0, 1, 3, 1, 1, 0, 0, 1, 1, 1, 1, 2, 0, 0, 1, 0, 2, 1, 1, 0, 0, 1,
    4, 1, 1, 1, 1, 1, 0, 1, 1, 4, 1, 2, 0, 1, 1, 0, 1, 0, 2, 1, 1, 2, 2, 0, 2, 0, 1, 2, 1, 2, 0, 0,
    1, 1, 0, 1, 2, 0, 1, 2, 1, 1, 1, 2, 0, 1, 1, 0, 1, 1, 0, 2, 1, 0, 1, 1, 0, 1, 0, 1, 1, 3, 1, 1,
    0, 2, 1, 2, 1, 1, 1, 0, 0, 2, 0, 0, 1, 3, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 0, 0, 2, 1, 0, 0,
    2, 1, 1, 2, 1, 1, 1, 4, 1, 2, 1, 2, 2, 0, 1, 0, 1, 1, 0, 0, 1, 2, 5, 2, 1, 1, 1, 0, 0, 2, 1, 0,
    1, 2, 0, 1, 0, 1, 3, 1, 1, 2, 2, 2, 1, 0, 0, 1, 0, 1, 0, 5, 1, 0, 2, 0, 1, 2, 0, 0, 0, 0, 0, 2,
];
pub const BASE_ARR: [u32; 32] = [
    1, 61, 134, 198, 271, 332, 402, 470, 526, 586, 642, 700, 765, 814, 879, 931, 37, 100, 164, 230,
    301, 364, 439, 497, 557, 613, 674, 736, 791, 847, 907, 965,
];
