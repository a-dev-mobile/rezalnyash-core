package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.Comparator;

/* loaded from: classes.dex */
public class SolutionBiggestUnusedTileAreaComparator implements Comparator<Solution> {
    @Override // java.util.Comparator
    public int compare(Solution solution, Solution solution2) {
        long biggestArea = solution2.getBiggestArea() - solution.getBiggestArea();
        if (biggestArea == 0) {
            return 0;
        }
        return biggestArea > 0 ? 1 : -1;
    }
}
