import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Lambdas {
    public static void main(String args[]) {
        String[] companies = {"SoTrusty", "Apple", "Microsoft"};
        
        List<String> result = Arrays.stream(companies)
        .map(company -> String.format("%s is a great company", company))
        .collect(Collectors.toList());

        for (String company : result) {
            System.out.println(company);
        }
    }
}