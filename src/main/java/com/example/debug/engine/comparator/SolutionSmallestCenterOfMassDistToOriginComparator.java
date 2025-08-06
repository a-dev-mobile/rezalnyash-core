package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.Comparator;

/* loaded from: classes.dex */
public class SolutionSmallestCenterOfMassDistToOriginComparator implements Comparator<Solution> {
    @Override // java.util.Comparator
    public int compare(Solution solution, Solution solution2) {
        float centerOfMassDistanceToOrigin = solution.getCenterOfMassDistanceToOrigin() - solution2.getCenterOfMassDistanceToOrigin();
        if (centerOfMassDistanceToOrigin == 0.0f) {
            return 0;
        }
        return centerOfMassDistanceToOrigin > 0.0f ? 1 : -1;
    }
}
