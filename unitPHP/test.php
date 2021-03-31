class IsNumberZeroOrNegativeTest extends TestCase {
  public function testZeroOrNegative() {
    $this->assertFalse(lessThanOrEqualToZero(5), "5 is more than zero");
    $this->assertTrue(lessThanOrEqualToZero(0), "0 is equal zero");
    $this->assertTrue(lessThanOrEqualToZero(-5), "-5 is less than zero");
    $this->assertFalse(lessThanOrEqualToZero(1), "1 is more than zero");
    $this->assertFalse(lessThanOrEqualToZero(2), "2 is more than zero");
    $this->assertFalse(lessThanOrEqualToZero(10000), "10000 is more than zero");
    $this->assertFalse(lessThanOrEqualToZero(0.5), "0.5 is more than zero");
  }
}