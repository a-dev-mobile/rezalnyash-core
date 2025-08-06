
package com.example.debug.engine.console;

import com.example.debug.engine.CutListLogger;
import com.example.debug.engine.model.ClientInfo;
import com.example.debug.engine.model.Task;

/**
 * Простая консольная реализация логгера
 */
public class ConsoleCutListLogger implements CutListLogger {
    
    private boolean enableDebug = false;
    
    @Override
    public void error(String str) {
        System.err.println("[ERROR] " + str);
    }

    @Override
    public void error(String str, String str2) {
        System.err.println("[ERROR] " + str + " | " + str2);
    }

    @Override
    public void error(String str, String str2, String str3) {
        System.err.println("[ERROR] " + str + " | " + str2 + " | " + str3);
    }

    @Override
    public void error(String str, String str2, String str3, String str4) {
        System.err.println("[ERROR] " + str + " | " + str2 + " | " + str3 + " | " + str4);
    }

    @Override
    public void error(String str, String str2, String str3, String str4, Throwable th) {
        System.err.println("[ERROR] " + str + " | " + str2 + " | " + str3 + " | " + str4);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void error(String str, String str2, String str3, Throwable th) {
        System.err.println("[ERROR] " + str + " | " + str2 + " | " + str3);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void error(String str, String str2, Throwable th) {
        System.err.println("[ERROR] " + str + " | " + str2);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void error(String str, Throwable th) {
        System.err.println("[ERROR] " + str);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void fatal(String str) {
        System.err.println("[FATAL] " + str);
    }

    @Override
    public void fatal(String str, String str2) {
        System.err.println("[FATAL] " + str + " | " + str2);
    }

    @Override
    public void fatal(String str, String str2, String str3) {
        System.err.println("[FATAL] " + str + " | " + str2 + " | " + str3);
    }

    @Override
    public void fatal(String str, String str2, String str3, String str4) {
        System.err.println("[FATAL] " + str + " | " + str2 + " | " + str3 + " | " + str4);
    }

    @Override
    public void fatal(String str, String str2, String str3, String str4, Throwable th) {
        System.err.println("[FATAL] " + str + " | " + str2 + " | " + str3 + " | " + str4);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void fatal(String str, String str2, String str3, Throwable th) {
        System.err.println("[FATAL] " + str + " | " + str2 + " | " + str3);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void fatal(String str, String str2, Throwable th) {
        System.err.println("[FATAL] " + str + " | " + str2);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void fatal(String str, Throwable th) {
        System.err.println("[FATAL] " + str);
        if (th != null) {
            th.printStackTrace();
        }
    }

    @Override
    public void info(String str) {
        if (enableDebug) {
            System.out.println("[INFO] " + str);
        }
    }

    @Override
    public void info(String str, String str2) {
        if (enableDebug) {
            System.out.println("[INFO] " + str + " | " + str2);
        }
    }

    @Override
    public void info(String str, String str2, String str3) {
        if (enableDebug) {
            System.out.println("[INFO] " + str + " | " + str2 + " | " + str3);
        }
    }

    @Override
    public void info(String str, String str2, String str3, String str4) {
        if (enableDebug) {
            System.out.println("[INFO] " + str + " | " + str2 + " | " + str3 + " | " + str4);
        }
    }

    @Override
    public void log(String str, String str2, String str3, String str4, String str5, String str6) {
        // Минимальное логирование
    }

    @Override
    public void logClient(ClientInfo clientInfo) {
        // Клиент залогирован
    }

    @Override
    public void logExecution(Task task) {
        // Выполнение залогировано
    }

    @Override
    public void warn(String str) {
        System.out.println("[WARN] " + str);
    }

    @Override
    public void warn(String str, String str2) {
        System.out.println("[WARN] " + str + " | " + str2);
    }

    @Override
    public void warn(String str, String str2, String str3) {
        System.out.println("[WARN] " + str + " | " + str2 + " | " + str3);
    }

    @Override
    public void warn(String str, String str2, String str3, String str4) {
        System.out.println("[WARN] " + str + " | " + str2 + " | " + str3 + " | " + str4);
    }

    @Override
    public void debug(String str) {
        if (enableDebug) {
            System.out.println("[DEBUG] " + str);
        }
    }

    @Override
    public void debug(String str, String str2) {
        if (enableDebug) {
            System.out.println("[DEBUG] " + str + " | " + str2);
        }
    }

    @Override
    public void debug(String str, String str2, String str3) {
        if (enableDebug) {
            System.out.println("[DEBUG] " + str + " | " + str2 + " | " + str3);
        }
    }

    @Override
    public void debug(String str, String str2, String str3, String str4) {
        if (enableDebug) {
            System.out.println("[DEBUG] " + str + " | " + str2 + " | " + str3 + " | " + str4);
        }
    }

    @Override
    public void trace(String str) {
        if (enableDebug) {
            System.out.println("[TRACE] " + str);
        }
    }

    @Override
    public void trace(String str, String str2) {
        if (enableDebug) {
            System.out.println("[TRACE] " + str + " | " + str2);
        }
    }

    @Override
    public void trace(String str, String str2, String str3) {
        if (enableDebug) {
            System.out.println("[TRACE] " + str + " | " + str2 + " | " + str3);
        }
    }

    @Override
    public void trace(String str, String str2, String str3, String str4) {
        if (enableDebug) {
            System.out.println("[TRACE] " + str + " | " + str2 + " | " + str3 + " | " + str4);
        }
    }

    @Override
    public void trace(String str, String str2, Integer integer, StringBuilder sb) {
        if (enableDebug) {
            System.out.println("[TRACE] " + str + " | " + str2 + " | " + integer + " | " + sb);
        }
    }
}
