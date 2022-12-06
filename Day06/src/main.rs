use std::collections::HashSet;

// IMPROVEMENTS
// 1. Use &str instead of String
// 2. Dont hardcode len in the tests and in the main function

fn pos_first_marker(input: String, len: usize) -> usize {
    for pos in 0..input.len() - 3{
        let curr_marker = get_marker(input.clone(), pos, len);
        assert_eq!(curr_marker.len(), {len}, "Markers must have size {len}");

        if all_different(curr_marker) == true {

            // The marker position is the one of the last element of the marker
            // Markers have `len` elements
            // And we start counting by one
            return pos + len;
        }
    }

    panic!("Could not find position of first marker!");
}

fn all_different(curr_marker: String) -> bool {
    let mut char_set: HashSet<char> = HashSet::new();

    for char in curr_marker.chars() {
        char_set.insert(char);
    }

    return curr_marker.len() == char_set.len();
}

fn get_marker(input: String, pos: usize, len: usize) -> String {
    return input[pos .. pos + len].to_string();
}

fn main() {
    let input = "bfdbbngnvnsvshhhrvrbrtbrrhqrqgrrmmdfmmqttptltntrntrnrcrdcrrctctdtwtrwwmlltcltcllmpprvvtbbmbvbsvvqwvvscswsqqgzqqppvzppnddjwddlrdlllmwllfccdfccswwrhhndhdhfdftdfdcdcllcjjbsbgssvlvrrhfrfjfpjffvnfvfwfzftztwwrhwrhwhwddpjjmhmgmsssstzszqzfqzzprpzpnndttphtpphvvcpcjppdtdwwqgwwwnhwnnvhvsvqqtrrbsscwssgwglgwlldjdzjjbsjbsbvvpbvppvfvqvbqbzzwtwbwpbbbcffdgggpnpfptpfphfhfthfthffzrrbzbcblbmmzmhhnvvqvlvwvzvtztgzznbbcqbccwhcwcnwnhndhhgcghhlthllplvppgngrgsswfsfnfjjswstsrtrhthhqzqggdtggzvvjljddqdzzrmrjmjcmjmrrwlldttrhhfjjgbglgjlldrllppnwpnwnndrddtdqdhhpccbgcgzzswzztgzzrwwzzmtztvvrtrgttvddwvwvmwvmmdjdrdtthqhgqhggldgdfdnnzjnjffmrrnprnpnssjrrbrjjrrzvrvhvllfvlvnlvvhlhrhzrzrsrswrssjvjdjfjwwjmjvjjthhnjnmnppsccmlcclfclcbllldwldwdlwddbddmnmrmgrmmtsszgssqjsqscswwzmwmbwwvqqbtbtwwpbbwdbdbhhqmqgqddflfmfnmffbtfbbsdsffspsjpsspzszccrsrstrtjrjppcrrpqqvttsbbjtbjjpbbccmzznvvzzvnvmmwmhmvmsstlssqtssvggtbgttgvtggfhggbcgcmcrmcmhhbttdlttfrfprrvttrtggcpplccqggcwwdjjjplprrbsrbbjsjbbfhbhphtttpffdgdgjddzldzlzplpnplljttdqqlmlllhzhjhccpwcwhwwbbqtqffjpfpfpjptplltbtzbbwwvggpcpprdprpbpvvgzgczzfffzcfzccjffrmrhmrhhchvhddsvswscsnnpfppdrdcrdrgdgmdgmgllhggjwggvqvmmvrrzpprhhqfhfvffvlvnlngnffbbtccfbfhfwhhhhwcwzwjjbffvrffqrrnsnwswrsrvrjvrvprpsptspttbztzfzlfzlflcflclqlplglmlqqvlvvhccmmddqvqzvvmlllvnlnhnfnwnlndnntrrhvvgjjnpnggfqgqdqqzvvclvclvlfvlvvlzlssqffmrfmfmjjczcmmmnttnfnmfnmmmhmggjgdjjqmjjvsslszlzrrpdpcpfcpfpnpssvtsvtvllbttzzljzzhrrnsnddjgdjgjljzzvhvddmdvvdtdmmpmqmnnwcwfwbfwwsvvbpvvgvnvcvdvtvrrcppwbppcqpccwqqzmqmmzczppwbbrzrjzzcrcvrrpttrjjcdjcjfcclscstthqhfqhffdcfdfqdqldddsswbblppdmdnnjvvhwvhvjhvjhjhlhcclrrcqcgcjgcjjtbtctbcbsccfwwltlrlplpfpjphplhhgtghhrppwhhrprwpwhhdqdpdwpwwtccvncvvvrpvphpssfpplcczttltgltldlmlqmmbsszdszznpprsprsprsppsbppjddjhhnrrfsrffmlmglgmlglccddpssdpsddhhfmfsfrfsfnnlggfrgghmgmrmprmprrnvrnnbqnqddhhfwfmfwfmfpfdfzdzppsbpsbpprfrvvwvtwtltvlttrhhgvhhjttmssdggnzzvmvcczmzbzwzttvtpvpcphcctmmhshjsjbssglsglgtltslscchhhsjjpljjdtdsdpppptlpprmppwdppglgmggnddztdzddzppswwmhfbpqzffjqgmsntwsnrwqrqwgpwgrbpbjwrhbcdcvqjnwslsnwhglcsjbwjhswjvzssfqgwbbdgbwfrblfmmlmsndhtlbwzfwsspqlncspqbgbnzshbwpvrmjqjzbcbzzdgssbtqdzffjphqjvrspfrjhpspbwcjwbfhqzsdnjwqjzjtjgnbrdbwqhzffphzppvlmsmppqcfjbjbdsnbwtvthwqcfrtfrwchnmqmhnwfcjtbwqwwvlnpmwlrvzwljrljzqstzglqwbzfdftzltlcbvmmfwcjqglvznztwnvzvftpndqmngqswppsnqhdbgthrddfbcfpflpndrhmcqwvnbfztsvnjjdwqgpmvdwvdftgbtvrwbnvvrwsdfzhwbwdhlpzbcqdzhbfqtpjqcmrpvcsrmcwvgghqrclfzpfgnppzmhvdhvdfrcrnjbdcwbftcqjhhfdsnfnwjzjllzzqftzsjrqnsbpjdcswhhmwwdzmvmqcjtqnczjcvzmmqwzjhjpcczgpbmcvbwmpmvnghlrgcmrrdnmjvmvnhtpfpgwgmdfzvlbclzjzwdqqcvfhhgfzdhzpdvfmwjlzzrpdgzmttmvvcplbwfzqftcgcwcgcpgwvnmlqsplpqwfnhwvqtlwcspqdzshsqnlcpqhpcpbwdhdjsmvtbqdwbcrscqfjcrcjhbjbpzbshpbmlcthmbjfwhzfphgbfqfnfztptzvdnwrpslmdtpmzmpbsszqshwdghrbtvhwzhcmpcgfqggpgzwmhhdrlhlvnpzvwwhzqvgvhrzngttcnqgjjhnblncnqnjzlwnmwnrtvwjtnrbhthncwmzwqdbdgtwrncljddnbhmphgjzfrrgmmcwfwjwjlcrhvcdtvsrvsfhmlgmzsgjchhrfmqslmdgtdtrlbhdffddvbsdbdwlwdcmcmmpvzpdtmbthjdzlpwftptpfsggmhjfjwvbwljsfhfwtbfwmczwhbmhvzllqtcqqfbrcdqqsrcpfmnswnfzfqghcmcbqwgvzqpwvvmbpddlhgjgzvgmpljznrhqphwcztqzpnhzqdgpwwclmsgpwnwtvtsjsdmcnvmjbqglttrhbzqdbgwnbsqzmsmztndrtmlpszhzgjbbftbsdwwdrlftrbbnrsqshfhdpdrwmztcqzdjlnthnjhppwntmbqdgzpmfmfnccblsdwljqhjfgtlgvpzpjbsndmwzfwrbmdhpnmbchqlwqtbhhhqqbsfnvscjwrzvjdtvbsqwzvfhwbbjgqpzcwqjdrlfmggzmhbcrhtbqdjntbtqdvmvpqflmccfpnbmnmtqbdflsgczpbsqpfphlzqgvwbjlmsgshrhpcljzdvwvdvlqqwchtjmjgtqjhgwtnddmhphwhvwhtrhfbjjjzfgrcqngnnddctzdzlqjlbdwmjqzccwrvctrzgtzqsswggbqdnplclhtdslcvzhppcjjslnshtwjnbrwdprqhdtfqmqpgfgnqtdnnhrnzfrsqhlftpdslgmmvqhvpjqjwpwgtnmgrbhwntdjftfwtjzjtprctbtsjmqmpcbbtrjvsgqgsfjprqmsmdztbhnbgzldqfzgwqwnnccgcfclctrwqmqpgvfglgsmmpjszqnphnzcnvswpsfsrmnsnlqnpmvfdvdtfgzrdmbftdrrrbfsvzfgmnffvjpcpnndrwhtjjrrvnztlfhcvfqjgfrhtbnhmwnrmwdhzmmtvjmsqmghtbtfjwdnvdcqtqjrfhrwscjftmbgjmcsrbpdpttlmvfmnfjhnptqvggnshzqnlqqdpqqsqssppbwpblhgfrwrblpzwvqphpsgfmbpqtqqpjpgnbblzstgcjhqntgpbfwlzzctqbnbvpgwsdsdldqzhvznqcsrrghpwllshqpdlqnqgzfwrnhwsvhftzplspcbqmclplprlthvwjhdndrjblqdgwvgjlbmblbmcnbzwzdlnpnhhppvrtngvqqwsttgwlvtcqmtrvpbnvcnfqdtqrsrsmhclmtgbdwwdvhwgfcqpmprcpdhqwftcchbwvstcdqrlwtgbcfqfgzprgvpbbzlqfzbqtcrlzscnqpqwtgzbbbdvsvmhggdr".to_string();
    let pos_marker = pos_first_marker(input.clone(), 4);
    println!("Part one: {pos_marker}");

    let pos_marker = pos_first_marker(input.clone(), 14);
    println!("Part two: {pos_marker}");
}

#[cfg(test)]
mod tests{
    use crate::{pos_first_marker, all_different, get_marker};


    #[test]
    fn test_given_examples() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let computed = pos_first_marker(input, 4);
        let expected = 5;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let computed = pos_first_marker(input, 4);
        let expected = 6;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let computed = pos_first_marker(input, 4);
        let expected = 10;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let computed = pos_first_marker(input, 4);
        let expected = 11;
        assert_eq!(computed, expected, "Code does not work for the given examples");
    }

    #[test]
    fn test_given_examples_part_two() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let computed = pos_first_marker(input, 14);
        let expected = 19;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let computed = pos_first_marker(input, 14);
        let expected = 23;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let computed = pos_first_marker(input, 14);
        let expected = 23;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let computed = pos_first_marker(input, 14);
        let expected = 29;
        assert_eq!(computed, expected, "Code does not work for the given examples");

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let computed = pos_first_marker(input, 14);
        let expected = 26;
        assert_eq!(computed, expected, "Code does not work for the given examples");
    }

    #[test]
    fn test_all_different() {
        let input = "abcd".to_string();
        let computed = all_different(input);
        let expected = true;
        assert_eq!(computed, expected, "`all_different` function is not correct!");

        let input = "abcda".to_string();
        let computed = all_different(input);
        let expected = false;
        assert_eq!(computed, expected, "`all_different` function is not correct!");

        let input = "abcdab".to_string();
        let computed = all_different(input);
        let expected = false;
        assert_eq!(computed, expected, "`all_different` function is not correct!");
    }

    #[test]
    fn test_get_marker(){
        let (input, pos) = ("abcd".to_string(), 0);
        let computed = get_marker(input, pos, 4);
        let expected = "abcd".to_string();
        assert_eq!(computed, expected, "`all_different` function is not correct!");

        let (input, pos) = ("abcdefghijk".to_string(), 0);
        let computed = get_marker(input, pos, 4);
        let expected = "abcd".to_string();
        assert_eq!(computed, expected, "`all_different` function is not correct!");

        let (input, pos) = ("abcdefghijk".to_string(), 2);
        let computed = get_marker(input, pos, 4);
        let expected = "cdef".to_string();
        assert_eq!(computed, expected, "`all_different` function is not correct!");

    }

}
