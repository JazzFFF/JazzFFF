
type JObject = jni.JObject;

function jniTraceBack(env: jni.JNIEnv) {
    let Thread = env.findclass('java/lang/Thread');
    let currentThread = <jni.JObject>Thread.call('currentThread', '()Ljava/lang/Thread;')
    // print('currentThread', currentThread)
    let stack = <jni.JObjectArray>currentThread.call('getStackTrace', '()[Ljava/lang/StackTraceElement;')
    for (let i = 0; i < math.min(stack.$len(), 10); ++i) {
        let frame = stack[i]
        let desc = <jni.JObject>frame.call('toString', '()Ljava/lang/String;')
        log.info(i, desc.string)
    }
}

function loadPatchClass(env: jni.JNIEnv, clazz: string) {
    let loader = env.findclass('com/cyberkl/CyAgent').getfield('classLoader', 'Ljava/lang/ClassLoader;') as jni.JObject;
    assert(loader, 'classLoader');
    return env.castclass(loader.call('loadClass', '(Ljava/lang/String;)Ljava/lang/Class;', clazz) as jni.JObject);
}

let handles: Record<string, jni.HookHandle[]> = {};

function java_patch(env: jni.JNIEnv) {
    // 从jar包中加载java类
    let Test = loadPatchClass(env, 'cyberkl.patch.Test');
    log.info('Test:', Test);
    Test?.setfield('patchId', 'Ljava/lang/String;', MANIFEST.id);

    assert(Test, 'class not found');
    Test?.call('init', '()V');

    let hooks: Record<string, jni.HookOption[]> = {
        'java.io.File': [
            // 构造函数Hook
            {
                name: '<init>', sign: '(Ljava/lang/String;)V',
                enter(args) {
                    log.info('[File(lua)]', args[0].string);
                    // jniTraceBack(args._env);
                }
            },
            // 基于正则的Hook规则
            {
                name: '<init>', sign: '(Ljava/lang/String;)V',
                enter: rule.create({
                    block: [
                        {regex: 'Temp.*', index: 0},
                    ]
                }),
            },
            // Java实现的Hook
            {
                name: 'createNewFile', sign: '()Z',
                enter(args) {
                    log.info('[createNewFile]', args[0].string);
                }
            }
        ],
        // log4j漏洞
        'org.apache.logging.log4j.core.lookup.StrSubstitutor': [
            {
                name: 'resolveVariable', sign: '(Lorg/apache/logging/log4j/core/LogEvent;Ljava/lang/String;Ljava/lang/StringBuilder;II)Ljava/lang/String;',
                enter(args) {
                    let val = args[2].string
                    log.info('var', val)
                    if (string.find(val, '^jndi:')) {
                        return 'block'
                    }
                }
            },
        ],
    };

    for (const className in hooks) {
        log.info('[hook]', className);
        let methods = hooks[className];
        handles[className] = jni.hook(className, methods);
        for (let i = 0; i < methods.length; ++i) {
            let item = methods[i];
            log.info('  ' + item.name, handles[className][i]);
        }
    }
    log.info('[hook]', 'all success')

    // Hook之后要调用 applyHook 使Hook生效
    jni.applyHook();

    // 调用jni.attach，确保在java上下文中调用java代码
    jni.attach((env: jni.JNIEnv) => {
        let File = env.findclass('java/io/File');
        let file = File("(Ljava/lang/String;)V", 'D:/aaa.txt')
        // for (let i = 0; i < 10; ++i) {
        //     file.call('createNewFile', '()Z')
        // }
        // let System = env.findclass('java/lang/System');
        // System.call('gc', '()V');
    });
}

export function patch(manifest: any) {
    jni.attach(java_patch);
}

export function unpatch() {
    for (const className in handles) {
        for (const h of handles[className]) {
            jni.detachHook(h);
        }
    }
}

export function check() {}
