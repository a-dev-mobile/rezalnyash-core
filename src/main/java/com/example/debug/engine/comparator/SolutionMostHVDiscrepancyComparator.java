package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.Comparator;

/* loaded from: classes.dex */
public class SolutionMostHVDiscrepancyComparator implements Comparator<Solution> {
    @Override // java.util.Comparator
    public int compare(Solution solution, Solution solution2) {
        return solution.getDistictTileSet() - solution2.getDistictTileSet();
    }
}
