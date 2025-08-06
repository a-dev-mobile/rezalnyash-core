package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.Comparator;

/* loaded from: classes.dex */
public class SolutionMostUnusedPanelAreaComparator implements Comparator<Solution> {
    @Override // java.util.Comparator
    public int compare(Solution solution, Solution solution2) {
        long mostUnusedPanelArea = solution2.getMostUnusedPanelArea() - solution.getMostUnusedPanelArea();
        if (mostUnusedPanelArea == 0) {
            return 0;
        }
        return mostUnusedPanelArea > 0 ? 1 : -1;
    }
}
