public class ReferenceCopy {
    public static void main(String[] args) {
        StringBuilder s = new StringBuilder("hello");
        StringBuilder t = s;
        System.out.println(s);
        System.out.println(t);
    }
}
