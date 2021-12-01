@IsTest
public class DummyTest {
    public DummyTest() {
    }
    
    @IsTest
    static void testDummy() {
        try {
            String str = 'Not real';
            System.assertEquals(str, 'Real', 'Should have thrown an exception');
        } catch (Exception e) {
            // Good
        }
    }
}