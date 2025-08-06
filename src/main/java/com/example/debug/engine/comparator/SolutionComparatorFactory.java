package com.example.debug.engine.comparator;

import com.example.debug.engine.model.Solution;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.Iterator;
import java.util.List;

/* loaded from: classes.dex */
public class SolutionComparatorFactory {
    public static Comparator<Solution> getSolutionComparator(String str) {
        if (str == null) {
            return null;
        }
        if (str.equalsIgnoreCase(OptimizationPriority.MOST_TILES.toString())) {
            return new SolutionMostNbrTilesComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.LEAST_WASTED_AREA.toString())) {
            return new SolutionLeastWastedAreaComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.LEAST_NBR_CUTS.toString())) {
            return new SolutionLeastNbrCutsComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.MOST_HV_DISCREPANCY.toString())) {
            return new SolutionMostHVDiscrepancyComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.BIGGEST_UNUSED_TILE_AREA.toString())) {
            return new SolutionBiggestUnusedTileAreaComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN.toString())) {
            return new SolutionSmallestCenterOfMassDistToOriginComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.LEAST_NBR_MOSAICS.toString())) {
            return new SolutionLeastNbrMosaicsComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.LEAST_NBR_UNUSED_TILES.toString())) {
            return new SolutionLeastNbrUnusedTilesComparator();
        }
        if (str.equalsIgnoreCase(OptimizationPriority.MOST_UNUSED_PANEL_AREA.toString())) {
            return new SolutionMostUnusedPanelAreaComparator();
        }
        return null;
    }

    public static List<Comparator> getSolutionComparatorList(List<String> list) {
        ArrayList arrayList = new ArrayList();
        Iterator<String> it = list.iterator();
        while (it.hasNext()) {
            Comparator<Solution> solutionComparator = getSolutionComparator(it.next());
            if (solutionComparator != null) {
                arrayList.add(solutionComparator);
            }
        }
        return arrayList;
    }
}
