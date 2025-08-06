package com.example.debug.engine;

import com.example.debug.engine.model.Cut;
import com.example.debug.engine.model.Mosaic;
import com.example.debug.engine.model.Solution;
import com.example.debug.engine.model.Status;
import com.example.debug.engine.model.Task;
import com.example.debug.engine.model.TileDimensions;
import com.example.debug.engine.model.TileNode;
import com.example.debug.engine.stock.StockSolution;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.ListIterator;

/* loaded from: classes.dex */
public class CutListThread implements Runnable {
    private static final int MAX_BIND_PARAMETER_CNT = 999; // Replacement for RoomDatabase.MAX_BIND_PARAMETER_CNT
    


    
    private int accuracyFactor;
    private List<Solution> allSolutions;
    private String auxInfo;
    private boolean considerGrainDirection;
    private CutListLogger cutListLogger;
    private int cutThickness;
    private List<Comparator> finalSolutionPrioritizedComparators;
    private CutDirection firstCutOrientation;
    private String group;
    private List<Solution> solutions;
    private Long startTime;
    private StockSolution stockSolution;
    private Task task;
    private List<Comparator> threadPrioritizedComparators;
    private List<TileDimensions> tiles;
    private Status status = Status.QUEUED;
    private int percentageDone = 0;
    private int minTrimDimension = 0;

    public String getGroup() {
        return this.group;
    }

    public void setGroup(String str) {
        this.group = str;
    }

    public String getAuxInfo() {
        return this.auxInfo;
    }

    public void setAuxInfo(String str) {
        this.auxInfo = str;
    }

    public Task getTask() {
        return this.task;
    }

    public void setTask(Task task) {
        this.task = task;
    }

    public List<Comparator> getThreadPrioritizedComparators() {
        return this.threadPrioritizedComparators;
    }

    public void setThreadPrioritizedComparators(List<Comparator> list) {
        this.threadPrioritizedComparators = list;
    }

    public List<Comparator> getFinalSolutionPrioritizedComparators() {
        return this.finalSolutionPrioritizedComparators;
    }

    public void setFinalSolutionPrioritizedComparators(List<Comparator> list) {
        this.finalSolutionPrioritizedComparators = list;
    }

    public Status getStatus() {
        return this.status;
    }

    public int getCutThickness() {
        return this.cutThickness;
    }

    public void setCutThickness(int i) {
        this.cutThickness = i;
    }

    public int getMinTrimDimension() {
        return this.minTrimDimension;
    }

    public void setMinTrimDimension(int i) {
        this.minTrimDimension = i;
    }

    public CutDirection getFirstCutOrientation() {
        return this.firstCutOrientation;
    }

    public void setFirstCutOrientation(CutDirection cutDirection) {
        this.firstCutOrientation = cutDirection;
    }

    public boolean isConsiderGrainDirection() {
        return this.considerGrainDirection;
    }

    public void setConsiderGrainDirection(boolean z) {
        this.considerGrainDirection = z;
    }

    public int getPercentageDone() {
        return this.percentageDone;
    }

    public List<TileDimensions> getTiles() {
        return this.tiles;
    }

    public void setTiles(List<TileDimensions> list) {
        this.tiles = list;
    }

    public List<Solution> getSolutions() {
        return this.solutions;
    }

    public void setSolutions(List<Solution> list) {
        this.solutions = list;
    }

    public int getAccuracyFactor() {
        return this.accuracyFactor;
    }

    public void setAccuracyFactor(int i) {
        this.accuracyFactor = i;
    }

    public List<Solution> getAllSolutions() {
        return this.allSolutions;
    }

    public void setAllSolutions(List<Solution> list) {
        this.allSolutions = list;
    }

    public StockSolution getStockSolution() {
        return this.stockSolution;
    }

    public void setStockSolution(StockSolution stockSolution) {
        this.stockSolution = stockSolution;
    }

    public CutListLogger getCutListLogger() {
        return this.cutListLogger;
    }

    public void setCutListLogger(CutListLogger cutListLogger) {
        this.cutListLogger = cutListLogger;
    }

    @Override // java.lang.Runnable
    public void run() {
        try {
         
            this.status = Status.RUNNING;
            this.startTime = Long.valueOf(System.currentTimeMillis());
            computeSolutions();
            if (this.status != Status.TERMINATED) {
                this.status = Status.FINISHED;
            }
        } catch (Exception e) {
            this.status = Status.ERROR;
          
        }
    }

    public String getMaterial() {
        List<Solution> list = this.allSolutions;
        if (list == null || list.size() <= 0) {
            return null;
        }
        return this.allSolutions.get(0).getMaterial();
    }

    public long getElapsedTimeMillis() {
        if (this.startTime == null) {
            return 0L;
        }
        return System.currentTimeMillis() - this.startTime.longValue();
    }

    public int removeDuplicated(List<Solution> list) {
        ArrayList arrayList = new ArrayList();
        HashSet hashSet = new HashSet();
        int i = 0;
        for (Solution solution : list) {
            Iterator<Mosaic> it = solution.getMosaics().iterator();
            String str = "";
            while (it.hasNext()) {
                str = str + it.next().getRootTileNode().toStringIdentifier();
            }
            if (!hashSet.add(str)) {
                arrayList.add(solution);
                i++;
            }
        }
        list.removeAll(arrayList);
        return i;
    }

    private void sort(List<Solution> list, final List<Comparator> list2) {
        try {
            Collections.sort(list, new Comparator<Solution>() { // from class: com.cutlistoptimizer.engine.CutListThread.1
                @Override // java.util.Comparator
                public int compare(Solution solution, Solution solution2) {
                    int iCompare = 0;
                    for (Comparator comparator : list2) {
                        try {
                            iCompare = comparator.compare(solution, solution2);
                        } catch (Exception e) {
                     
                        }
                        if (iCompare != 0) {
                            break;
                        }
                    }
                    return iCompare;
                }
            });
        } catch (Exception e) {

        }
    }

    void computeSolutions() {
        boolean z;
        TileDimensions next;
        Mosaic mosaic;
        List<Solution> arrayList = new ArrayList<>();
        arrayList.add(new Solution(this.stockSolution));
        if (this.task.isRunning()) {
            int i = 0;
            for (TileDimensions tileDimensions : this.tiles) {
                i++;
                if (i % 3 == 0) {
                    this.percentageDone = (int) ((i / this.tiles.size()) * 100.0f);
                }
                ArrayList<Solution> arrayList2 = new ArrayList();
                Iterator<Solution> it = arrayList.iterator();
                boolean z2 = false;
                while (it.hasNext()) {
                    Solution next2 = it.next();
                    ListIterator<Mosaic> listIterator = next2.getMosaics().listIterator();
                    Mosaic next3 = listIterator.next();
                    while (true) {
                        if (next3 == null) {
                            z = true;
                            break;
                        }
                        if (next3.getMaterial() != null && !next3.getMaterial().equals(tileDimensions.getMaterial())) {
                         
                        } else {
                            List<Mosaic> arrayList3 = new ArrayList<>();
                            add(tileDimensions, next3, arrayList3);
                            for (Mosaic mosaic2 : arrayList3) {
                                Solution solution = new Solution(next2, next3);
                                solution.addMosaic(mosaic2);
                                arrayList2.add(solution);
                            }
                            if (arrayList3.size() > 0) {
                                z = true;
                                z2 = true;
                                break;
                            }
                            if (listIterator.hasNext()) {
                                mosaic = listIterator.next();
                                next3 = mosaic;
                            } else {
                                Iterator<TileDimensions> it2 = next2.getUnusedStockPanels().iterator();
                                while (true) {
                                    next3 = null;
                                    if (it2.hasNext()) {
                                        next = it2.next();
                                        if (next.fits(tileDimensions)) {
                                            break;
                                        }
                                    } else {
                                        next = null;
                                        break;
                                    }
                                }
                                if (next != null) {
                                    next2.getUnusedStockPanels().remove(next);
                                    mosaic = new Mosaic(next);
                                    listIterator.add(mosaic);
                                    next3 = mosaic;
                                } else {
                                    next3 = null;
                                }
                            }
                        }
                    }
                    if (z2 == z) {
                        it.remove();
                    } else {
                        next2.getNoFitPanels().add(tileDimensions);
                    }
                }
                for (Solution solution2 : arrayList2) {
                    solution2.setCreatorThreadGroup(this.group);
                    solution2.setAuxInfo(this.auxInfo);
                }
                arrayList.addAll(arrayList2);
                removeDuplicated(arrayList);
                ArrayList arrayList4 = new ArrayList();
                sort(arrayList, this.threadPrioritizedComparators);
                arrayList4.addAll(arrayList.subList(Math.min(arrayList.size() - 1, this.accuracyFactor), arrayList.size() - 1));
                arrayList.removeAll(arrayList4);
            }
            synchronized (this.allSolutions) {
                this.allSolutions.addAll(arrayList);
                sort(this.allSolutions, this.finalSolutionPrioritizedComparators);
                ArrayList arrayList5 = new ArrayList();
                List<Solution> list = this.allSolutions;
                arrayList5.addAll(list.subList(Math.min(list.size() - 1, this.accuracyFactor), this.allSolutions.size() - 1));
                this.allSolutions.removeAll(arrayList5);
                List<Solution> list2 = this.allSolutions;
                for (Solution solution3 : list2.subList(0, Math.min(list2.size(), 5))) {
                    if (solution3.getMaterial() != null) {
                        this.task.incrementThreadGroupRankings(solution3.getMaterial(), solution3.getCreatorThreadGroup());
                    }
                }
                Iterator<Mosaic> it3 = this.allSolutions.get(0).getMosaics().iterator();
                while (it3.hasNext()) {
                    if (it3.next().getUsedArea() == 0) {
                        it3.remove();
                    }
                }
            }
        }
    }

    private void add(TileDimensions tileDimensions, Mosaic mosaic, List<Mosaic> list) {
        if (!this.considerGrainDirection || mosaic.getOrientation() == 0 || tileDimensions.getOrientation() == 0) {
            fitTile(tileDimensions, mosaic, list, this.cutThickness);
            if (tileDimensions.isSquare()) {
                return;
            }
            fitTile(tileDimensions.rotate90(), mosaic, list, this.cutThickness);
            return;
        }
        if (mosaic.getOrientation() != tileDimensions.getOrientation()) {
            tileDimensions = tileDimensions.rotate90();
        }
        fitTile(tileDimensions, mosaic, list, this.cutThickness);
    }

    private void fitTile(TileDimensions tileDimensions, Mosaic mosaic, List<Mosaic> list, int i) {
        ArrayList<TileNode> arrayList = new ArrayList<TileNode>();
        findCandidates(tileDimensions.getWidth(), tileDimensions.getHeight(), mosaic.getRootTileNode(), arrayList);
        for (TileNode tileNode : arrayList) {
            if (tileNode.getWidth() == tileDimensions.getWidth() && tileNode.getHeight() == tileDimensions.getHeight()) {
                TileNode tileNodeCopy = copy(mosaic.getRootTileNode(), tileNode);
                TileNode tileNodeFindTile = tileNodeCopy.findTile(tileNode);
                tileNodeFindTile.setExternalId(tileDimensions.getId());
                tileNodeFindTile.setFinal(true);
                tileNodeFindTile.setRotated(tileDimensions.isRotated());
                Mosaic mosaic2 = new Mosaic(tileNodeCopy, mosaic.getMaterial());
                mosaic2.setStockId(mosaic.getStockId());
                mosaic2.getCuts().addAll(mosaic.getCuts());
                mosaic2.setOrientation(mosaic.getOrientation());
                list.add(mosaic2);
            } else {
                if (CutDirection.BOTH == this.firstCutOrientation || CutDirection.HORIZONTAL == this.firstCutOrientation) {
                    TileNode tileNodeCopy2 = copy(mosaic.getRootTileNode(), tileNode);
                    List<Cut> listSplitHV = splitHV(tileNodeCopy2.findTile(tileNode), tileDimensions, i);
                    Mosaic mosaic3 = new Mosaic(tileNodeCopy2, mosaic.getMaterial());
                    mosaic3.setStockId(mosaic.getStockId());
                    mosaic3.getCuts().addAll(mosaic.getCuts());
                    mosaic3.getCuts().addAll(listSplitHV);
                    mosaic3.setOrientation(mosaic.getOrientation());
                    list.add(mosaic3);
                    if (tileNode.getWidth() == tileDimensions.getWidth() || tileNode.getHeight() == tileDimensions.getHeight()) {
                    }
                }
                if (CutDirection.BOTH == this.firstCutOrientation || CutDirection.VERTICAL == this.firstCutOrientation) {
                    TileNode tileNodeCopy3 = copy(mosaic.getRootTileNode(), tileNode);
                    List<Cut> listSplitVH = splitVH(tileNodeCopy3.findTile(tileNode), tileDimensions, i);
                    Mosaic mosaic4 = new Mosaic(tileNodeCopy3, mosaic.getMaterial());
                    mosaic4.setStockId(mosaic.getStockId());
                    mosaic4.getCuts().addAll(mosaic.getCuts());
                    mosaic4.getCuts().addAll(listSplitVH);
                    mosaic4.setOrientation(mosaic.getOrientation());
                    list.add(mosaic4);
                }
            }
        }
    }

    private List<Cut> splitHV(TileNode tileNode, TileDimensions tileDimensions, int i) {
        ArrayList arrayList = new ArrayList();
        if (tileNode.getWidth() > tileDimensions.getWidth()) {
            arrayList.add(splitHorizontally(tileNode, tileDimensions.getWidth(), i));
            if (tileNode.getHeight() > tileDimensions.getHeight()) {
                arrayList.add(splitVertically(tileNode.getChild1(), tileDimensions.getHeight(), i, tileDimensions.getId()));
                tileNode.getChild1().getChild1().setFinal(true);
                tileNode.getChild1().getChild1().setRotated(tileDimensions.isRotated());
            } else {
                tileNode.getChild1().setFinal(true);
                tileNode.getChild1().setRotated(tileDimensions.isRotated());
                tileNode.getChild1().setExternalId(tileDimensions.getId());
            }
        } else {
            arrayList.add(splitVertically(tileNode, tileDimensions.getHeight(), i, tileDimensions.getId()));
            tileNode.getChild1().setFinal(true);
            tileNode.getChild1().setRotated(tileDimensions.isRotated());
        }
        return arrayList;
    }

    private List<Cut> splitVH(TileNode tileNode, TileDimensions tileDimensions, int i) {
        ArrayList arrayList = new ArrayList();
        if (tileNode.getHeight() > tileDimensions.getHeight()) {
            arrayList.add(splitVertically(tileNode, tileDimensions.getHeight(), i));
            if (tileNode.getWidth() > tileDimensions.getWidth()) {
                arrayList.add(splitHorizontally(tileNode.getChild1(), tileDimensions.getWidth(), i, tileDimensions.getId()));
                tileNode.getChild1().getChild1().setFinal(true);
                tileNode.getChild1().getChild1().setRotated(tileDimensions.isRotated());
            } else {
                tileNode.getChild1().setFinal(true);
                tileNode.getChild1().setRotated(tileDimensions.isRotated());
                tileNode.getChild1().setExternalId(tileDimensions.getId());
            }
        } else {
            arrayList.add(splitHorizontally(tileNode, tileDimensions.getWidth(), i, tileDimensions.getId()));
            tileNode.getChild1().setFinal(true);
            tileNode.getChild1().setRotated(tileDimensions.isRotated());
        }
        return arrayList;
    }

    private static Cut splitHorizontally(TileNode tileNode, int i, int i2) {
        return splitHorizontally(tileNode, i, i2, MAX_BIND_PARAMETER_CNT);
    }

    private static Cut splitHorizontally(TileNode tileNode, int i, int i2, int i3) {
        if (tileNode == null) {
            return null;
        }
        int width = tileNode.getWidth();
        int height = tileNode.getHeight();
        TileNode tileNode2 = new TileNode(tileNode.getX1(), tileNode.getX1() + i, tileNode.getY1(), tileNode.getY2());
        tileNode2.setExternalId(i3);
        if (tileNode2.getArea() > 0) {
            tileNode.setChild1(tileNode2);
        }
        TileNode tileNode3 = new TileNode(tileNode.getX1() + i + i2, tileNode.getX2(), tileNode.getY1(), tileNode.getY2());
        if (tileNode3.getArea() > 0) {
            tileNode.setChild2(tileNode3);
        }
        return new Cut.Builder().setX1(tileNode.getX1() + i).setY1(tileNode.getY1()).setX2(tileNode.getX1() + i).setY2(tileNode.getY2()).setOriginalWidth(width).setOriginalHeight(height).setHorizontal(true).setCutCoords(i).setOriginalTileId(tileNode.getId()).setChild1TileId(tileNode2.getId()).setChild2TileId(tileNode3.getId()).build();
    }

    private static Cut splitVertically(TileNode tileNode, int i, int i2) {
        return splitVertically(tileNode, i, i2, MAX_BIND_PARAMETER_CNT);
    }

    private static Cut splitVertically(TileNode tileNode, int i, int i2, int i3) {
        if (tileNode == null) {
            return null;
        }
        int width = tileNode.getWidth();
        int height = tileNode.getHeight();
        TileNode tileNode2 = new TileNode(tileNode.getX1(), tileNode.getX2(), tileNode.getY1(), tileNode.getY1() + i);
        tileNode2.setExternalId(i3);
        if (tileNode2.getArea() > 0) {
            tileNode.setChild1(tileNode2);
        }
        TileNode tileNode3 = new TileNode(tileNode.getX1(), tileNode.getX2(), tileNode.getY1() + i + i2, tileNode.getY2());
        if (tileNode3.getArea() > 0) {
            tileNode.setChild2(tileNode3);
        }
        return new Cut.Builder().setX1(tileNode.getX1()).setY1(tileNode.getY1() + i).setX2(tileNode.getX2()).setY2(tileNode.getY1() + i).setOriginalWidth(width).setOriginalHeight(height).setHorizontal(false).setCutCoords(i).setOriginalTileId(tileNode.getId()).setChild1TileId(tileNode2.getId()).setChild2TileId(tileNode3.getId()).build();
    }

    private void findCandidates(int i, int i2, TileNode tileNode, List<TileNode> list) {
        boolean z;
        if (tileNode == null || tileNode.isFinal() || tileNode.getWidth() < i || tileNode.getHeight() < i2) {
            return;
        }
        if (tileNode.getChild1() == null && tileNode.getChild2() == null) {
            boolean z2 = false;
            if (tileNode.getWidth() == i || tileNode.getWidth() >= this.minTrimDimension + i) {
                z = true;
            } else {
                if (tileNode.getWidth() > i) {
                    this.task.setMinTrimDimensionInfluenced(true);
                }
                z = false;
            }
            if (tileNode.getHeight() == i2 || tileNode.getHeight() >= this.minTrimDimension + i2) {
                z2 = true;
            } else if (tileNode.getHeight() > i2) {
                this.task.setMinTrimDimensionInfluenced(true);
            }
            if (z && z2) {
                list.add(tileNode);
                return;
            }
            return;
        }
        if (tileNode.getChild1() != null) {
            findCandidates(i, i2, tileNode.getChild1(), list);
        }
        if (tileNode.getChild2() != null) {
            findCandidates(i, i2, tileNode.getChild2(), list);
        }
    }

    private static TileNode copy(TileNode tileNode, TileNode tileNode2) {
        TileNode tileNode3 = new TileNode(tileNode);
        copyChildren(tileNode, tileNode3, tileNode2);
        return tileNode3;
    }

    private static void copyChildren(TileNode tileNode, TileNode tileNode2, TileNode tileNode3) {
        if (tileNode == tileNode3) {
            return;
        }
        if (tileNode.getChild1() != null) {
            tileNode2.setChild1(new TileNode(tileNode.getChild1()));
            copyChildren(tileNode.getChild1(), tileNode2.getChild1(), tileNode3);
        }
        if (tileNode.getChild2() != null) {
            tileNode2.setChild2(new TileNode(tileNode.getChild2()));
            copyChildren(tileNode.getChild2(), tileNode2.getChild2(), tileNode3);
        }
    }
}
