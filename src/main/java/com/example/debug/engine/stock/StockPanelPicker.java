package com.example.debug.engine.stock;

import com.example.debug.engine.console.ConsoleCutListLogger;
import com.example.debug.engine.model.Task;
import com.example.debug.engine.model.TileDimensions;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
// import org.slf4j.Logger;
// import org.slf4j.LoggerFactory;

/* loaded from: classes.dex */
public class StockPanelPicker {
    private static final int MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE = 10;
    private static final int MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION = 100;
    private static final ConsoleCutListLogger logger =new ConsoleCutListLogger();
    private final StockSolutionGenerator stockSolutionGenerator;
    private Thread stockSolutionSorterThread;
    private final Task task;
    private final List<StockSolution> stockSolutions = new ArrayList();
    private int maxRetrievedIdx = 0;

    public StockPanelPicker(List<TileDimensions> list, List<TileDimensions> list2, Task task, Integer num) {
        this.stockSolutionGenerator = new StockSolutionGenerator(list, list2, num);
        this.task = task;
    }

    public StockPanelPicker(List<TileDimensions> list, List<TileDimensions> list2, Task task) {
        this.stockSolutionGenerator = new StockSolutionGenerator(list, list2, null);
        this.task = task;
    }

    public StockSolution getStockSolution(int i) throws InterruptedException {
        if (this.stockSolutionSorterThread == null) {
            throw new RuntimeException("StockPanelPickerThread not initialized");
        }
        while (this.stockSolutions.size() <= i && this.stockSolutionSorterThread.isAlive()) {
            try {
                logger.debug("Waiting for stock solution generation: idx[" + i + "]");
                Thread.sleep(1000L);
            } catch (InterruptedException e) {
                logger.error("Waiting for stock solution generation interrupted", (Throwable) e);
            }
        }
        if (this.stockSolutions.size() <= i && !this.stockSolutionSorterThread.isAlive()) {
            logger.debug("No more possible stock solutions");
            return null;
        }
        this.maxRetrievedIdx = Math.max(this.maxRetrievedIdx, i);
        return this.stockSolutions.get(i);
    }

    private void sortStockSolutions() {
        try {
            Collections.sort(this.stockSolutions, new Comparator<StockSolution>() { // from class: com.cutlistoptimizer.engine.stock.StockPanelPicker.1
                @Override // java.util.Comparator
                public int compare(StockSolution stockSolution, StockSolution stockSolution2) {
                    return (int) (stockSolution.getTotalArea() - stockSolution2.getTotalArea());
                }
            });
        } catch (Exception unused) {
        }
    }

    public long getRequiredArea() {
        return this.stockSolutionGenerator.getRequiredArea();
    }

    public void init() {
        Thread thread = new Thread() { // from class: com.cutlistoptimizer.engine.stock.StockPanelPicker.2
            @Override // java.lang.Thread, java.lang.Runnable
            public void run() {
                StockSolution stockSolutionGenerateStockSolution;
                StockSolution stockSolution = null;
                while (true) {
                    if (StockPanelPicker.this.maxRetrievedIdx >= StockPanelPicker.this.stockSolutions.size() - 1 || StockPanelPicker.this.stockSolutions.size() <= 10) {
                        synchronized (StockPanelPicker.this.stockSolutions) {
                            stockSolutionGenerateStockSolution = StockPanelPicker.this.stockSolutionGenerator.generateStockSolution();
                            if (stockSolutionGenerateStockSolution != null) {
                                StockPanelPicker.this.stockSolutions.add(stockSolutionGenerateStockSolution);
                                if (!stockSolutionGenerateStockSolution.hasUniquePanelSize()) {
                                    StockSolution stockSolution2 = new StockSolution(stockSolutionGenerateStockSolution);
                                    stockSolution2.sortPanelsDesc();
                                    StockPanelPicker.this.stockSolutions.add(stockSolution2);
                                }
                                ConsoleCutListLogger logger2 = StockPanelPicker.logger;
                                StringBuilder sb = new StringBuilder();
                                sb.append("Added idx[");
                                sb.append(StockPanelPicker.this.stockSolutions.size() - 1);
                                sb.append("] [");
                                sb.append(stockSolutionGenerateStockSolution.getStockTileDimensions().size());
                                sb.append("] area[");
                                sb.append(stockSolutionGenerateStockSolution.getTotalArea());
                                sb.append("][");
                                sb.append(stockSolutionGenerateStockSolution);
                                sb.append("] to stack ");
                                logger2.debug(sb.toString());
                            }
                        }
                        stockSolution = stockSolutionGenerateStockSolution;
                    } else {
                        StockPanelPicker.logger.debug("No need to generate new candidate stock solution: maxRetrievedIdx[" + StockPanelPicker.this.maxRetrievedIdx + "] stockSolutions[" + StockPanelPicker.this.stockSolutions.size() + "]");
                    }
                    if (StockPanelPicker.this.stockSolutions.size() > 10) {
                        try {
                            Thread.sleep(1000L);
                        } catch (InterruptedException e) {
                            StockPanelPicker.logger.error("Stock panel picker interrupted", (Throwable) e);
                        }
                    }
                    if (stockSolution == null || !StockPanelPicker.this.task.isRunning() || (StockPanelPicker.this.task.hasSolutionAllFit() && StockPanelPicker.this.stockSolutions.size() >= 100)) {
                        break;
                    }
                }
                if (stockSolution == null) {
                    StockPanelPicker.logger.debug("Finishing stock picker thread: nbrGeneratedStockSolutions[" + StockPanelPicker.this.stockSolutions.size() + "] - There are no more available stock solutions");
                    return;
                }
                if (!StockPanelPicker.this.task.isRunning()) {
                    StockPanelPicker.logger.debug("Finishing stock picker thread: nbrGeneratedStockSolutions[" + StockPanelPicker.this.stockSolutions.size() + "] - Task has no longer running status");
                    return;
                }
                if (StockPanelPicker.this.task.hasSolutionAllFit()) {
                    StockPanelPicker.logger.debug("Finishing stock picker thread: nbrGeneratedStockSolutions[" + StockPanelPicker.this.stockSolutions.size() + "] - Task has already an all fit solution");
                }
            }
        };
        this.stockSolutionSorterThread = thread;
        thread.start();
    }
}
