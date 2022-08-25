import java.util.Scanner;

//HelloWorld.java
public class HelloWorld {    
	public static void main(String[] args) {        
		System.out.println("start...current pid:" + ProcessHandle.current().pid() );        
		hello h1 = new hello();        
		h1.hello();
		// 产生中断，等待注入        
		Scanner sc = new Scanner(System.in);        
		sc.nextInt();        
		hello h2 = new hello();        
		h2.hello();        
		System.out.println("ends...");    
	}
}
