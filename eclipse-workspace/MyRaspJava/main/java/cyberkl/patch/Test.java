package cyberkl.patch;

import com.cyberkl.CyProbe;
import com.cyberkl.Logger;
import com.cyberkl.type.HookHandler;
import org.apache.commons.compress.archivers.tar.TarArchiveEntry;
import org.apache.commons.compress.archivers.tar.TarArchiveInputStream;

import java.io.InputStream;

public class Test {
    public static void main(String[] args) {
        System.out.println("ffff");
    }

    static void init() {
        System.out.println("test class loading: " + patchId);
        CyProbe probe = CyProbe.getInstance();
        try {
            probe.addHook(
                    "com.vmware.vropspluginui.mvc.ServicesController",
                    "uploadOvaFile",
                    "(Lorg/springframework/web/multipart/commons/CommonsMultipartFile;Ljavax/servlet/http/HttpServletResponse;)V",
                    new HookHandler(Test::checkUploadOvaFile)
                    );
            probe.addHook(
                    "java.io.File",
                    "<init>",
                    "(Ljava/lang/String;)V",
                    new HookHandler(args -> {
                        Logger.info("[File] " + args[0]);
                        return null;
                    })
            );
            probe.apply();
        } catch (Throwable e) {
            e.printStackTrace();
        }
        System.out.println("test class hook finished");
    }

    // 判断参数是否包含路径穿越
    public static boolean has_traversal(String param) {
        // 左右斜杠，一视同仁
        String path = "/" + param.replaceAll("\\\\", "/") + "/";

        if (path.indexOf("/../") != -1) {
            return true;
        }
        return false;
    }

    public static Object checkUploadOvaFile(Object[] args) {
        Object uploadFile = args[0];
        Object response = args[1];
        boolean isEmpty = (Boolean) Reflection.invokeMethod(uploadFile, "isEmpty", new Class[0], new Object[0]);
        if (!isEmpty) {
            InputStream inputStream = (InputStream) Reflection.invokeMethod(uploadFile, "getInputStream", new Class[0],
                    new Object[0]);
            TarArchiveInputStream in = new TarArchiveInputStream(inputStream);
            boolean has_traversal = false;
            String has_traversal_name = null;
            try {
                TarArchiveEntry entry = in.getNextTarEntry();
                while (entry != null) {
                    if (entry.isDirectory()) {
                        entry = in.getNextTarEntry();
                        continue;
                    }
                    String name = entry.getName();
                    if (has_traversal(name)) {
                        has_traversal_name = name;
                        has_traversal = true;
                        break;
                    }
                    entry = in.getNextTarEntry();
                }
                in.close();
            } catch (Exception e) {
                return null;
            }

            if (has_traversal) {
                SecurityException securityException = new SecurityException("Request blocked by CyberGuard");
                String name = (String) Reflection.invokeMethod(uploadFile, "getOriginalFilename", new Class[0],
                        new Object[0]);
                Reflection.invokeMethod(response, "sendError", new Class[] { Integer.TYPE, String.class },
                        new Object[] { 400, "Request blocked by CyberGuard" });
                Logger.info(" checkUploadOvaFile  uploadFile : " + name + " include :  " + has_traversal_name
                        + " has traversal");
                // 返回异常表示拦截此函数调用
                return securityException;
            }
        }
        return null;
    }

    static String patchId = null;
}
