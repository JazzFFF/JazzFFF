import java.lang.instrument.Instrumentation;

public class AgentMain {

	public static void agentmain(String agentArgs, Instrumentation inst){

        System.out.println("通过控制台打印：print all class：");

        Class[] allLoadedClasses = inst.getAllLoadedClasses();
        int i=0;
        for (Class allLoadedClass : allLoadedClasses) {
        	i++;
            System.out.println("i:" + i + "  " + allLoadedClass.getName());

        }

    }

}
