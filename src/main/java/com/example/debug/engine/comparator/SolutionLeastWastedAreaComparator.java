package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.Comparator;

/* loaded from: classes.dex */
public class SolutionLeastWastedAreaComparator implements Comparator<Solution> {
    @Override // java.util.Comparator
    public int compare(Solution solution, Solution solution2) {
        long unusedArea = solution.getUnusedArea() - solution2.getUnusedArea();
        if (unusedArea == 0) {
            return 0;
        }
        return unusedArea > 0 ? 1 : -1;
    }
}
