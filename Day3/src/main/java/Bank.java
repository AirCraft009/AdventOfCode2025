public class Bank {
    private String layout;
    private static int idGiver;
    private int id;
    public int maxJolt;
    public long maxFriction;

    public Bank(String layout){
        id = getNewId();
        if(layout == null){
            throw new NullPointerException("The provided Bank layout cannot be null\nID: " + id);
        }
        this.layout = layout;
    }

    public int getNewId(){
        return idGiver += 1;
    }

    public void calcMaxFriction(){

        // set Maxval at the tenner position
        long finVal = 0;
        int maxInd = largestNumFromArrSlice(0, 11);
        int extra = Integer.parseInt(String.valueOf(layout.charAt(maxInd)));
        finVal += extra;
        for (int i = 0; i < 11; i++) {
            finVal *= 10;
            maxInd = largestNumFromArrSlice(maxInd + 1, 10 - i);
            // not surrounding with a try catch as It's done in the largesNum... func
            extra = Integer.parseInt(String.valueOf(layout.charAt(maxInd)));
            finVal += extra;
        }
        maxFriction = finVal;
    }

    public int largestNumFromArrSlice(int start, int endOffset){
        int highVal;
        int maxVal = 0;
        int maxInd = 0;
        for (int i = start; i < layout.length()-endOffset; i++) {
            char cNum = layout.charAt(i);
            try {
                highVal = Integer.parseInt(String.valueOf(cNum));
            } catch (Exception e) {
                throw new IllegalArgumentException("illegal State of Bank layout\nID: " + id);
            }
            if (highVal >maxVal){
                maxVal = highVal;
                maxInd = i;
                if(maxVal == 9){
                    break;
                }
            }
        }
        return maxInd;
    }


    public void calcMaxJolt(){
        int maxInd = largestNumFromArrSlice(0, 1);
        // set Maxval at the tenner position
        int finVal = Integer.parseInt(String.valueOf(layout.charAt(maxInd))) * 10;
        maxInd = largestNumFromArrSlice(maxInd + 1, 0);
        finVal += Integer.parseInt(String.valueOf(layout.charAt(maxInd)));
        //System.out.println(finVal);
        maxJolt = finVal;
    }
}
