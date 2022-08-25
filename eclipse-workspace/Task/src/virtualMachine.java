import com.sun.tools.attach.AgentInitializationException;
import com.sun.tools.attach.AgentLoadException;
import com.sun.tools.attach.AttachNotSupportedException;
import com.sun.tools.attach.VirtualMachine;
import java.io.IOException;
public class virtualMachine {    
	public static void main( String[] args ) {        
		try {            
				//VirtualMachine 来自tools.jar            
				// VirtualMachine.attach("9444") 9444为线程PID，使用jps查看   
				String idString = "8944";
				VirtualMachine vm = VirtualMachine.attach(idString);           
				System.out.println("attach id:"+ idString);
				//指定要使用的Agent路径           
//				vm.loadAgent("C:\\Users\\zhangzefeng\\Desktop\\JavaProject\\AgentMain.jar");  
				String jarPathString = "C:\\Users\\zhangzefeng\\Desktop\\JavaProject\\AgentAssitMain.jar";
				vm.loadAgent(jarPathString);
				System.out.println("loadAgent:"+ jarPathString);
				
			} catch (AttachNotSupportedException e) {            
				e.printStackTrace();        
			} catch (IOException e) {            
				e.printStackTrace();        
			} catch (AgentLoadException e) {            
				e.printStackTrace();        
			} catch (AgentInitializationException e) {           
				e.printStackTrace();        
			}
	}    
}

