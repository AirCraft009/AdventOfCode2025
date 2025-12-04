import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class BatterieBanks {
    List<String> TextBank;
    Bank[] allBanks;
    int JoltageSum;
    long fricSum;

    public BatterieBanks() throws IOException {
        TextBank = Files.readAllLines(Paths.get("input.txt"));
        allBanks = new Bank[TextBank.size()];
    }

    public int evalJoltage(){
        int sum  = 0;
        for (int i = 0; i < TextBank.size(); i++ ){
            allBanks[i] = new Bank(TextBank.get(i));
            allBanks[i].calcMaxJolt();
            sum += allBanks[i].maxJolt;
        }
        return sum;
    }

    public long evalFriction(){
        long sum  = 0;
        for (int i = 0; i < TextBank.size(); i++ ){
            allBanks[i] = new Bank(TextBank.get(i));
            allBanks[i].calcMaxFriction();
            sum += allBanks[i].maxFriction;
        }
        return sum;
    }

    public int getJoltageSum() {
        if(JoltageSum == 0){
            JoltageSum = evalJoltage();
        }
        return JoltageSum;
    }

    public long getFricSum() {
        if(fricSum == 0){
            fricSum = evalFriction();
        }
        return fricSum;
    }
}
