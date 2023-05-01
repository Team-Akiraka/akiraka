/**
 * @author Arrokoth
 * @project Default (Template) Project
 * @copyright Copyright © 2023 Arrokoth All Rights Reserved.
 */
public class VersionChecker {
    public static void main(String[] args) {
        System.out.println("{\"java_home\":\"" + System.getProperty("java.home") + "\",\"java_class_version\":\"" + (Double.parseDouble(System.getProperty("java.class.version")) - 44) + "\"}");
    }
}