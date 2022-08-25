//TransformerDemo.java
import javassist.ClassClassPath;
import javassist.ClassPool;
import javassist.CtClass;
import javassist.CtMethod;
import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.security.ProtectionDomain;

public class TransformerDemo implements ClassFileTransformer {    
	// 只需要修改这里就能修改别的函数    
	public static final String editClassName = "com.test.hello";    
	public static final String editClassName2 = editClassName.replace('.', '/');    
	public static final String editMethodName = "hello";    
	@Override    
	public byte[] transform(ClassLoader loader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classfileBuffer) throws IllegalClassFormatException {        
		try {            
			ClassPool cp = ClassPool.getDefault();            
			if (classBeingRedefined != null) {                
				ClassClassPath ccp = new ClassClassPath(classBeingRedefined);                
				cp.insertClassPath(ccp);            
				}            
			CtClass ctc = cp.get(editClassName);            
			CtMethod method = ctc.getDeclaredMethod(editMethodName);            
			String source = "{System.out.println(\"hello transformer\");}";           
			method.insertBefore(source);            
			byte[] bytes = ctc.toBytecode();            
			ctc.detach();            
			return bytes;        
			} catch (Exception e){            
				e.printStackTrace();        
			}        
		return null;    
	}
}

