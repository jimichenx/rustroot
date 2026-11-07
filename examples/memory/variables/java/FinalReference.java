// Uncomment the marked line to trigger a final-variable assignment error.
public class FinalReference {
    public static void main(String[] args) {
        final StringBuilder s = new StringBuilder("hello");
        s.append("!");
        // s = new StringBuilder("bye");
        System.out.println(s);
    }
}
