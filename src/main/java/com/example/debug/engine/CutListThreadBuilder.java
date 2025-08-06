package com.example.debug.engine;

import com.example.debug.engine.model.Configuration;
import com.example.debug.engine.model.Solution;
import com.example.debug.engine.model.Task;
import com.example.debug.engine.model.TileDimensions;
import com.example.debug.engine.stock.StockSolution;
import java.util.Comparator;
import java.util.List;

/* loaded from: classes.dex */
public class CutListThreadBuilder {
    private int accuracyFactor;
    private List<Solution> allSolutions;
    private String auxInfo;
    private Configuration configuration;
    private CutListLogger cutListLogger;
    private int cutThickness;
    private List<Comparator> finalSolutionPrioritizedComparators;
    private CutDirection firstCutOrientation;
    private String group;
    private int minTrimDimension;
    private StockSolution stockSolution;
    private Task task;
    private List<Comparator> threadPrioritizedComparators;
    private List<TileDimensions> tiles;

    public CutListThreadBuilder setGroup(String str) {
        this.group = str;
        return this;
    }

    public CutListThreadBuilder setAuxInfo(String str) {
        this.auxInfo = str;
        return this;
    }

    public CutListThreadBuilder setAllSolutions(List<Solution> list) {
        this.allSolutions = list;
        return this;
    }

    public CutListThreadBuilder setTiles(List<TileDimensions> list) {
        this.tiles = list;
        return this;
    }

    public CutListThreadBuilder setConfiguration(Configuration configuration) {
        this.configuration = configuration;
        return this;
    }

    public CutListThreadBuilder setCutThickness(int i) {
        this.cutThickness = i;
        return this;
    }

    public CutListThreadBuilder setMinTrimDimension(int i) {
        this.minTrimDimension = i;
        return this;
    }

    public CutListThreadBuilder setFirstCutOrientation(CutDirection cutDirection) {
        this.firstCutOrientation = cutDirection;
        return this;
    }

    public CutListThreadBuilder setThreadPrioritizedComparators(List<Comparator> list) {
        this.threadPrioritizedComparators = list;
        return this;
    }

    public CutListThreadBuilder setFinalSolutionPrioritizedComparators(List<Comparator> list) {
        this.finalSolutionPrioritizedComparators = list;
        return this;
    }

    public CutListThreadBuilder setTask(Task task) {
        this.task = task;
        return this;
    }

    public CutListThreadBuilder setAccuracyFactor(int i) {
        this.accuracyFactor = i;
        return this;
    }

    public CutListThreadBuilder setStockSolution(StockSolution stockSolution) {
        this.stockSolution = stockSolution;
        return this;
    }

    public CutListThreadBuilder setCutListLogger(CutListLogger cutListLogger) {
        this.cutListLogger = cutListLogger;
        return this;
    }

    public CutListThread build() {
        CutListThread cutListThread = new CutListThread();
        cutListThread.setGroup(this.group);
        cutListThread.setAuxInfo(this.auxInfo);
        cutListThread.setAllSolutions(this.allSolutions);
        cutListThread.setTiles(this.tiles);
        cutListThread.setConsiderGrainDirection(this.configuration.isConsiderOrientation());
        cutListThread.setCutThickness(this.cutThickness);
        cutListThread.setMinTrimDimension(this.minTrimDimension);
        cutListThread.setFirstCutOrientation(this.firstCutOrientation);
        cutListThread.setThreadPrioritizedComparators(this.threadPrioritizedComparators);
        cutListThread.setFinalSolutionPrioritizedComparators(this.finalSolutionPrioritizedComparators);
        cutListThread.setTask(this.task);
        cutListThread.setAccuracyFactor(this.accuracyFactor);
        cutListThread.setStockSolution(this.stockSolution);
        cutListThread.setCutListLogger(this.cutListLogger);
        this.task.addThread(cutListThread);
        return cutListThread;
    }
}
