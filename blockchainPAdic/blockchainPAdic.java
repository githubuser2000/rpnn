import java.util.*;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

class Node {
    String name;
    String hash;
    List<Node> children = new ArrayList<>();

    Node(String name) {
        this.name = name;
    }
}

public class TreeBlockchain {

    static Random rand = new Random();

    static String sha256(String base) {
        try {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] hash = digest.digest(base.getBytes());
            StringBuilder hexString = new StringBuilder();
            for (byte b : hash) {
                String hex = Integer.toHexString(0xff & b);
                if(hex.length() == 1) hexString.append('0');
                hexString.append(hex);
            }
            return hexString.toString();
        } catch(NoSuchAlgorithmException e) {
            throw new RuntimeException(e);
        }
    }

    static String generateName(String level) {
        return level + "_" + (rand.nextInt(1000) + 1);
    }

    static Node generateNode(String level, int depth, String parentHash) {
        Node node = new Node(generateName(level));
        int numChildren = depth > 0 ? rand.nextInt(3) + 1 : 0;

        for(int i=0;i<numChildren;i++) {
            if(depth == 3) node.children.add(generateNode("Bundesland", depth-1, parentHash));
            else if(depth == 2) node.children.add(generateNode("Bezirk", depth-1, parentHash));
            else if(depth == 1) node.children.add(generateNode("Dorf", depth-1, parentHash));
        }

        StringBuilder data = new StringBuilder(node.name + parentHash);
        for(Node c : node.children) data.append(c.hash);
        node.hash = sha256(data.toString());

        return node;
    }

    static void printNode(Node node, int indent) {
        for(int i=0;i<indent;i++) System.out.print("  ");
        System.out.println(node.name + " : " + node.hash);
        for(Node c : node.children) printNode(c, indent+1);
    }

    public static void main(String[] args) {
        Node tree = generateNode("Land", 3, "");
        printNode(tree, 0);
    }
}
