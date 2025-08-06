package com.example.debug.engine.stock;

import com.example.debug.engine.model.TileDimensions;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/* loaded from: classes.dex */
class StockSolutionGenerator {
    private static int NBR_STOCK_SOLUTION_MAX_LENGTH = 1000;
    private static final Logger logger = LoggerFactory.getLogger((Class<?>) StockSolutionGenerator.class);
    private StockSolution allPanelStockSolution;
    private final Integer maxStockSolutionLenghtHint;
    private int prevIndexToIterate;
    private final List<Integer> previousReturnedStockTilesIndexes;
    private long requiredArea;
    private int requiredMaxDimension;
    private long smallestTilleArea;
    private final List<StockSolution> stockSolutionsToExclude;
    private final List<TileDimensions> stockTiles;
    private final List<TileDimensions> tilesToFit;

    public StockSolutionGenerator(List<TileDimensions> list, List<TileDimensions> list2, Integer num) {
        this.stockSolutionsToExclude = new ArrayList();
        this.previousReturnedStockTilesIndexes = new ArrayList();
        this.requiredArea = 0L;
        this.requiredMaxDimension = 0;
        this.smallestTilleArea = Long.MAX_VALUE;
        this.tilesToFit = list;
        this.stockTiles = list2;
        this.maxStockSolutionLenghtHint = num;
        sortStockTilesAreaAsc(list2);
        calcRequiredArea();
        this.allPanelStockSolution = genAllPanelStockSolution();
    }

    public StockSolutionGenerator(List<TileDimensions> list, List<TileDimensions> list2) {
        this(list, list2, null);
    }

    private void calcRequiredArea() {
        for (TileDimensions tileDimensions : this.tilesToFit) {
            this.requiredArea += tileDimensions.getArea();
            if (tileDimensions.getMaxDimension() > this.requiredMaxDimension) {
                this.requiredMaxDimension = tileDimensions.getMaxDimension();
            }
            if (tileDimensions.getArea() < this.smallestTilleArea) {
                this.smallestTilleArea = tileDimensions.getArea();
            }
        }
    }

    private boolean isExcluded(StockSolution stockSolution) {
        Iterator<StockSolution> it = this.stockSolutionsToExclude.iterator();
        while (it.hasNext()) {
            if (it.next().equals(stockSolution)) {
                return true;
            }
        }
        return false;
    }

    private boolean isExcluded(List<TileDimensions> list, List<Integer> list2) {
        List<StockSolution> list3 = this.stockSolutionsToExclude;
        if (list3 == null || list3.size() == 0) {
            return false;
        }
        ArrayList arrayList = new ArrayList();
        Iterator<Integer> it = list2.iterator();
        while (it.hasNext()) {
            arrayList.add(list.get(it.next().intValue()));
        }
        return isExcluded(new StockSolution(arrayList));
    }

    private Integer getNextUnusedStockTile(List<TileDimensions> list, List<Integer> list2, int i, TileDimensions tileDimensions) {
        while (true) {
            i++;
            if (i >= list.size()) {
                return null;
            }
            if (!list2.contains(Integer.valueOf(i)) && (list.get(i).getWidth() > tileDimensions.getWidth() || list.get(i).getHeight() > tileDimensions.getHeight())) {
                break;
            }
        }
        return Integer.valueOf(i);
    }

    private StockSolution getCandidateStockSolution(List<TileDimensions> list, long j, int i, long j2, int i2) {
        ArrayList arrayList;
        int i3;
        ArrayList arrayList2 = new ArrayList();
        if (this.previousReturnedStockTilesIndexes.size() == i2) {
            arrayList = new ArrayList(this.previousReturnedStockTilesIndexes);
            i3 = this.prevIndexToIterate;
        } else {
            for (int i4 = 0; i4 < i2; i4++) {
                arrayList2.add(Integer.valueOf(i4));
            }
            arrayList = arrayList2;
            i3 = 0;
        }
        return iterate(list, j, i, j2, i2, arrayList, i3);
    }

    private boolean isValid(List<Integer> list) {
        HashSet hashSet = new HashSet();
        Iterator<Integer> it = list.iterator();
        while (it.hasNext()) {
            if (!hashSet.add(it.next())) {
                return false;
            }
        }
        return true;
    }

    private StockSolution iterate(List<TileDimensions> list, long j, int i, long j2, int i2, List<Integer> list2, int i3) {
        Integer nextUnusedStockTile;
        HashSet hashSet = new HashSet();
        for (int i4 = 0; i4 < i3; i4++) {
            if (!hashSet.add(list2.get(i4))) {
                return null;
            }
        }
        if (i3 < i2 - 1) {
            int height = 0;
            int width = 0;
            int i5 = 0;
            while (i5 < list.size()) {
                int i6 = height;
                int i7 = width;
                int i8 = i5;
                StockSolution stockSolutionIterate = iterate(list, j, i, j2, i2, list2, i3 + 1);
                if (stockSolutionIterate != null) {
                    return stockSolutionIterate;
                }
                i5 = i8;
                while (true) {
                    i5++;
                    if (i5 >= list.size()) {
                        break;
                    }
                    if (list.get(i5).getWidth() != i7 || list.get(i5).getHeight() != i6) {
                        if (list.get(i5).getArea() >= j2) {
                            break;
                        }
                    }
                }
                if (i5 < list.size()) {
                    width = list.get(i5).getWidth();
                    height = list.get(i5).getHeight();
                    int i9 = i3;
                    while (i9 < list2.size() && i5 < list.size()) {
                        list2.set(i9, Integer.valueOf(i5));
                        i9++;
                        i5++;
                    }
                } else {
                    width = i7;
                    height = i6;
                }
            }
        }
        do {
            long area = j;
            boolean z = false;
            for (Integer num : list2) {
                area -= list.get(num.intValue()).getArea();
                if (list.get(num.intValue()).getMaxDimension() >= i) {
                    z = true;
                }
            }
            if (area <= 0 && z && isValid(list2) && !isExcluded(list, list2)) {
                StockSolution stockSolution = new StockSolution(new TileDimensions[0]);
                Iterator<Integer> it = list2.iterator();
                while (it.hasNext()) {
                    stockSolution.addStockTile(list.get(it.next().intValue()));
                }
                this.previousReturnedStockTilesIndexes.clear();
                this.previousReturnedStockTilesIndexes.addAll(list2);
                this.prevIndexToIterate = i3;
                return stockSolution;
            }
            nextUnusedStockTile = getNextUnusedStockTile(list, list2, list2.get(i3).intValue(), list.get(list2.get(i3).intValue()));
            if (nextUnusedStockTile != null) {
                list2.set(i3, nextUnusedStockTile);
            }
        } while (nextUnusedStockTile != null);
        return null;
    }

    private void sortStockTilesAreaAsc(List<TileDimensions> list) {
        Collections.sort(list, new Comparator<TileDimensions>() { // from class: com.cutlistoptimizer.engine.stock.StockSolutionGenerator.1
            @Override // java.util.Comparator
            public int compare(TileDimensions tileDimensions, TileDimensions tileDimensions2) {
                return Long.compare(tileDimensions.getArea(), tileDimensions2.getArea());
            }
        });
    }

    private boolean isUniqueStockPanel() {
        int id = this.stockTiles.get(0).getId();
        Iterator<TileDimensions> it = this.stockTiles.iterator();
        while (it.hasNext()) {
            if (it.next().getId() != id) {
                return false;
            }
        }
        return true;
    }

    private long getBiggestStockTileArea() {
        Iterator<TileDimensions> it = this.stockTiles.iterator();
        long jMax = 0;
        while (it.hasNext()) {
            jMax = Math.max(jMax, it.next().getArea());
        }
        return jMax;
    }

    public long getRequiredArea() {
        return this.requiredArea;
    }

    private StockSolution genAllPanelStockSolution() {
        StockSolution stockSolution = new StockSolution(new TileDimensions[0]);
        for (int i = 0; i < this.stockTiles.size() && i < NBR_STOCK_SOLUTION_MAX_LENGTH; i++) {
            stockSolution.addStockTile(this.stockTiles.get((this.stockTiles.size() - i) - 1));
        }
        stockSolution.sortPanelsAsc();
        return stockSolution;
    }

    public StockSolution generateStockSolution() {
        if (isUniqueStockPanel()) {
            if (isExcluded(this.allPanelStockSolution)) {
                return null;
            }
            this.stockSolutionsToExclude.add(this.allPanelStockSolution);
            return this.allPanelStockSolution;
        }
        int iCeil = (int) Math.ceil(this.requiredArea / getBiggestStockTileArea());
        int iIntValue = NBR_STOCK_SOLUTION_MAX_LENGTH;
        Integer num = this.maxStockSolutionLenghtHint;
        if (num != null && num.intValue() >= iCeil) {
            iIntValue = this.maxStockSolutionLenghtHint.intValue();
        }
        if (iIntValue == NBR_STOCK_SOLUTION_MAX_LENGTH && !isExcluded(this.allPanelStockSolution)) {
            this.stockSolutionsToExclude.add(this.allPanelStockSolution);
            return this.allPanelStockSolution;
        }
        while (iCeil < this.stockTiles.size() && iCeil <= iIntValue) {
            StockSolution candidateStockSolution = getCandidateStockSolution(this.stockTiles, this.requiredArea, this.requiredMaxDimension, this.smallestTilleArea, iCeil);
            if (candidateStockSolution != null) {
                this.stockSolutionsToExclude.add(candidateStockSolution);
                candidateStockSolution.sortPanelsAsc();
                return candidateStockSolution;
            }
            iCeil++;
        }
        return null;
    }
}


