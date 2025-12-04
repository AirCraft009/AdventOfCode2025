import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException {
        BatterieBanks batterieBanks = new BatterieBanks();
        System.out.println(batterieBanks.getJoltageSum());
        System.out.println(batterieBanks.getFricSum());
    }
}
