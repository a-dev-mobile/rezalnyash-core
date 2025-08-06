package com.example.debug.engine;

import com.example.debug.engine.model.ClientInfo;
import com.example.debug.engine.model.Task;

/* loaded from: classes.dex */
public interface CutListLogger {
    void error(String str);

    void error(String str, String str2);

    void error(String str, String str2, String str3);

    void error(String str, String str2, String str3, String str4);

    void error(String str, String str2, String str3, String str4, Throwable th);

    void error(String str, String str2, String str3, Throwable th);

    void error(String str, String str2, Throwable th);

    void error(String str, Throwable th);

    void fatal(String str);

    void fatal(String str, String str2);

    void fatal(String str, String str2, String str3);

    void fatal(String str, String str2, String str3, String str4);

    void fatal(String str, String str2, String str3, String str4, Throwable th);

    void fatal(String str, String str2, String str3, Throwable th);

    void fatal(String str, String str2, Throwable th);

    void fatal(String str, Throwable th);

    void info(String str);

    void info(String str, String str2);

    void info(String str, String str2, String str3);

    void info(String str, String str2, String str3, String str4);

    void log(String str, String str2, String str3, String str4, String str5, String str6);

    void logClient(ClientInfo clientInfo);

    void logExecution(Task task);

    void warn(String str);

    void warn(String str, String str2);

    void warn(String str, String str2, String str3);

    void warn(String str, String str2, String str3, String str4);

    void debug(String str);

    void debug(String str, String str2);

    void debug(String str, String str2, String str3);

    void debug(String str, String str2, String str3, String str4);

    void trace(String str);

    void trace(String str, String str2);

    void trace(String str, String str2, String str3);

    void trace(String str, String str2, String str3, String str4);

    void trace(String str, String str2, Integer integer, StringBuilder sb);
}
