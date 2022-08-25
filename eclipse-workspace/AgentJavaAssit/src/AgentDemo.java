//AgentDemo.java
import java.io.IOException;
import java.lang.instrument.Instrumentation;
import java.lang.instrument.UnmodifiableClassException;
public class AgentDemo {    
	public static void agentmain(String agentArgs, Instrumentation inst) throws IOException, UnmodifiableClassException {        
		Class[] classes = inst.getAllLoadedClasses();        // 判断类是否已经加载        
		for (Class aClass : classes) {            
			if (aClass.getName().equals(TransformerDemo.editClassName)) {                
				// 添加 Transformer                
				inst.addTransformer(new TransformerDemo(), true);                
				// 触发 Transformer                
				inst.retransformClasses(aClass);            
			}        
		}    
	}
}			
