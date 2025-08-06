package com.example.debug.engine.comparator;

/* loaded from: classes.dex */
public enum OptimizationPriority {
    MOST_TILES("MOST_TILES"),
    LEAST_WASTED_AREA("LEAST_WASTED_AREA"),
    LEAST_NBR_CUTS("LEAST_NBR_CUTS"),
    MOST_HV_DISCREPANCY("MOST_HV_DISCREPANCY"),
    BIGGEST_UNUSED_TILE_AREA("BIGGEST_UNUSED_TILE_AREA"),
    SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN("SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN"),
    LEAST_NBR_MOSAICS("LEAST_NBR_MOSAICS"),
    LEAST_NBR_UNUSED_TILES("LEAST_NBR_UNUSED_TILES"),
    MOST_UNUSED_PANEL_AREA("MOST_UNUSED_PANEL_AREA");

    private final String text;

    OptimizationPriority(String str) {
        this.text = str;
    }

    @Override // java.lang.Enum
    public String toString() {
        return this.text;
    }
}
