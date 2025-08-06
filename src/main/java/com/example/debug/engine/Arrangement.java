package com.example.debug.engine;

import java.util.ArrayList;
import java.util.List;

/* loaded from: classes.dex */
public class Arrangement {
    public static <T> List<List<T>> generatePermutations(List<T> list) {
        if (list.size() == 0) {
            ArrayList arrayList = new ArrayList();
            arrayList.add(new ArrayList());
            return arrayList;
        }
        T tRemove = list.remove(0);
        ArrayList arrayList2 = new ArrayList();
        for (List list2 : generatePermutations(list)) {
            for (int i = 0; i <= list2.size(); i++) {
                ArrayList arrayList3 = new ArrayList(list2);
                arrayList3.add(i, tRemove);
                arrayList2.add(arrayList3);
            }
        }
        return arrayList2;
    }
}
