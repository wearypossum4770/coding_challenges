Test.assertEquals(profitableGamble(0.2, 50, 9), true);
Test.assertEquals(profitableGamble(0.9, 1, 2), false);
Test.assertEquals(profitableGamble(0.9, 3, 2), true);
Test.assertEquals(profitableGamble(0.33, 10, 3.3), true);
Test.assertEquals(profitableGamble(0, 1000, 0.01), false);
Test.assertEquals(profitableGamble(0.1, 1000, 7), true);
Test.assertEquals(profitableGamble(0, 0, 0), false);
