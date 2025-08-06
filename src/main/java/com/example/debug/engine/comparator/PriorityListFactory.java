package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Configuration;
import java.util.ArrayList;
import java.util.List;

/* loaded from: classes.dex */
public class PriorityListFactory {
    public static List<String> getFinalSolutionPrioritizedComparatorList(Configuration configuration) {
        ArrayList arrayList = new ArrayList();
        if (configuration.getOptimizationPriority() == 0) {
            arrayList.add(OptimizationPriority.MOST_TILES.toString());
            arrayList.add(OptimizationPriority.LEAST_WASTED_AREA.toString());
            arrayList.add(OptimizationPriority.LEAST_NBR_CUTS.toString());
        } else {
            arrayList.add(OptimizationPriority.MOST_TILES.toString());
            arrayList.add(OptimizationPriority.LEAST_NBR_CUTS.toString());
            arrayList.add(OptimizationPriority.LEAST_WASTED_AREA.toString());
        }
        arrayList.add(OptimizationPriority.LEAST_NBR_MOSAICS.toString());
        arrayList.add(OptimizationPriority.BIGGEST_UNUSED_TILE_AREA.toString());
        arrayList.add(OptimizationPriority.MOST_HV_DISCREPANCY.toString());
        return arrayList;
    }
}
