package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.Comparator;

/* loaded from: classes.dex */
public class SolutionLeastNbrCutsComparator implements Comparator<Solution> {
    @Override // java.util.Comparator
    public int compare(Solution solution, Solution solution2) {
        return solution.getNbrCuts() - solution2.getNbrCuts();
    }
}
