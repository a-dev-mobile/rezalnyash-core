package com.example.debug.engine;

import org.apache.commons.lang3.time.DateUtils;

/* loaded from: classes.dex */
public class Utils {
    public static String longElapsedTime2HumanReadable(long j) {
        int i = ((int) (j / 1000)) % 60;
        int i2 = (int) ((j / DateUtils.MILLIS_PER_MINUTE) % 60);
        int i3 = (int) ((j / DateUtils.MILLIS_PER_HOUR) % 24);
        String str = "";
        if (i3 > 0) {
            str = "" + i3 + "h";
        }
        if (i2 > 0) {
            str = str + i2 + "m";
        }
        if (i <= 0) {
            return str;
        }
        return str + i + "s";
    }
}
