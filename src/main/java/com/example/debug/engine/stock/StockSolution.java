package com.example.debug.engine.stock;

import com.example.debug.engine.model.TileDimensions;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;

/* loaded from: classes.dex */
public class StockSolution {
    private List<TileDimensions> stockTileDimensions;

    public StockSolution(StockSolution stockSolution) {
        this.stockTileDimensions = new ArrayList();
        Iterator<TileDimensions> it = stockSolution.getStockTileDimensions().iterator();
        while (it.hasNext()) {
            this.stockTileDimensions.add(new TileDimensions(it.next()));
        }
    }

    public StockSolution(List<TileDimensions> list) {
        new ArrayList();
        this.stockTileDimensions = list;
    }

    public StockSolution(TileDimensions... tileDimensionsArr) {
        this.stockTileDimensions = new ArrayList();
        for (TileDimensions tileDimensions : tileDimensionsArr) {
            this.stockTileDimensions.add(tileDimensions);
        }
    }

    public void addStockTile(TileDimensions tileDimensions) {
        this.stockTileDimensions.add(tileDimensions);
    }

    public List<TileDimensions> getStockTileDimensions() {
        return this.stockTileDimensions;
    }

    public void setStockTileDimensions(List<TileDimensions> list) {
        this.stockTileDimensions = list;
    }

    public void sortPanelsAsc() {
        Collections.sort(this.stockTileDimensions, new Comparator<TileDimensions>() { // from class: com.cutlistoptimizer.engine.stock.StockSolution.1
            @Override // java.util.Comparator
            public int compare(TileDimensions tileDimensions, TileDimensions tileDimensions2) {
                return (int) (tileDimensions.getArea() - tileDimensions2.getArea());
            }
        });
    }

    public void sortPanelsDesc() {
        Collections.sort(this.stockTileDimensions, new Comparator<TileDimensions>() { // from class: com.cutlistoptimizer.engine.stock.StockSolution.2
            @Override // java.util.Comparator
            public int compare(TileDimensions tileDimensions, TileDimensions tileDimensions2) {
                return (int) (tileDimensions2.getArea() - tileDimensions.getArea());
            }
        });
    }

    public boolean hasUniquePanelSize() {
        Iterator<TileDimensions> it = this.stockTileDimensions.iterator();
        TileDimensions next = it.next();
        while (it.hasNext()) {
            if (!it.next().hasSameDimensions(next)) {
                return false;
            }
        }
        return true;
    }

    /* JADX WARN: Code restructure failed: missing block: B:21:0x0052, code lost:
    
        r4.remove();
     */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj == null || getClass() != obj.getClass()) {
            return false;
        }
        StockSolution stockSolution = (StockSolution) obj;
        if (this.stockTileDimensions.size() != stockSolution.stockTileDimensions.size()) {
            return false;
        }
        ArrayList arrayList = new ArrayList(stockSolution.stockTileDimensions);
        for (TileDimensions tileDimensions : this.stockTileDimensions) {
            Iterator it = arrayList.iterator();
            while (it.hasNext()) {
                if (tileDimensions.hasSameDimensions((TileDimensions) it.next())) {
                    break;
                }
            }
            return false;
        }
        return true;
    }

    public int hashCode() {
        List<TileDimensions> list = this.stockTileDimensions;
        if (list != null) {
            return list.hashCode();
        }
        return 0;
    }

    public String toString() {
        String str = new String();
        for (TileDimensions tileDimensions : this.stockTileDimensions) {
            str = str + "[" + tileDimensions.getWidth() + "x" + tileDimensions.getHeight() + "]";
        }
        return str;
    }

    public String toStringGrouped() {
        String str = new String();
        HashMap<String, Integer> map = new HashMap<String, Integer>();
        for (TileDimensions tileDimensions : this.stockTileDimensions) {
            String str2 = tileDimensions.getWidth() + "x" + tileDimensions.getHeight();
            if (map.containsKey(str2)) {
                map.put(str2, Integer.valueOf(((Integer) map.get(str2)).intValue() + 1));
            } else {
                map.put(str2, 1);
            }
        }
        for (String str3 : map.keySet()) {
            str = str + str3 + "*" + map.get(str3) + " ";
        }
        return str.substring(0, str.length() - 1);
    }

    public long getTotalArea() {
        Iterator<TileDimensions> it = this.stockTileDimensions.iterator();
        long area = 0;
        while (it.hasNext()) {
            area += it.next().getArea();
        }
        return area;
    }
}
