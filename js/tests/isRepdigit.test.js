Test.assertEquals(isRepdigit(6), true);
Test.assertEquals(isRepdigit(66), true);
Test.assertEquals(isRepdigit(666), true);
Test.assertEquals(isRepdigit(6666), true);
Test.assertEquals(isRepdigit(1001), false);
Test.assertEquals(isRepdigit(-11), false, "The number must be >= 0");
