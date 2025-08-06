package com.example.debug.engine.model;

import com.example.debug.engine.CutListThread;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

/* loaded from: classes.dex */
public class Task {

    
    private CalculationRequest calculationRequest;
    private ClientInfo clientInfo;
    private long endTime;
    private double factor;
    private String id;
    private boolean isMinTrimDimensionInfluenced;
    private String log;
    private CalculationResponse solution;
    private Map<String, List<TileDimensions>> stockDimensionsPerMaterial;
    private Map<String, List<TileDimensions>> tileDimensionsPerMaterial;
    private Status status = Status.IDLE;
    private final Map<String, List<Solution>> solutions = new HashMap();
    private final Map<String, Integer> perMaterialPercentageDone = new HashMap();
    private long lastQueried = System.currentTimeMillis();
    private List<TileDimensions> noMaterialTiles = new ArrayList();
    private List<CutListThread> threads = new ArrayList();
    private Map<String, HashMap<String, Integer>> threadGroupRankings = new HashMap();
    private long startTime = System.currentTimeMillis();


    public Task(String str) {
        this.id = str;
    }

    public String getId() {
        return this.id;
    }

    public void setId(String str) {
        this.id = str;
    }

    public Status getStatus() {
        return this.status;
    }

    public CalculationRequest getCalculationRequest() {
        return this.calculationRequest;
    }

    public void setCalculationRequest(CalculationRequest calculationRequest) {
        this.calculationRequest = calculationRequest;
    }

    public HashMap<String, Integer> getThreadGroupRankings(String str) {
        HashMap<String, Integer> map;
        synchronized (this.threadGroupRankings) {
            map = this.threadGroupRankings.get(str);
        }
        return map;
    }

    public void incrementThreadGroupRankings(String str, String str2) {
        synchronized (this.threadGroupRankings) {
            Integer num = this.threadGroupRankings.get(str).get(str2);
            if (num == null) {
                this.threadGroupRankings.get(str).put(str2, 1);
            } else {
                this.threadGroupRankings.get(str).put(str2, Integer.valueOf(num.intValue() + 1));
            }
        }
    }

    public boolean isRunning() {
        return Status.RUNNING.equals(this.status);
    }

    public int setRunningStatus() {
        if (this.status != Status.IDLE) {
            return -1;
        }
        this.status = Status.RUNNING;
        return 0;
    }

    public int stop() {
        this.endTime = System.currentTimeMillis();
        if (this.status != Status.RUNNING) {
            return -1;
        }
        this.status = Status.STOPPED;
        return 0;
    }

    public int terminate() {
        this.endTime = System.currentTimeMillis();
        if (this.status != Status.RUNNING) {
            return -1;
        }
        this.status = Status.TERMINATED;
        return 0;
    }

    public void terminateError() {
        this.endTime = System.currentTimeMillis();
        this.status = Status.ERROR;
    }

    public CalculationResponse getSolution() {
        return this.solution;
    }

    public void setSolution(CalculationResponse calculationResponse) {
        this.solution = calculationResponse;
    }

    public void addMaterialToCompute(String str) {
        this.solutions.put(str, new ArrayList());
        this.perMaterialPercentageDone.put(str, 0);
        this.threadGroupRankings.put(str, new HashMap<>());
    }

    public List<Solution> getSolutions(String str) {
        return this.solutions.get(str);
    }

    public String getLog() {
        return this.log;
    }

    public void setLog(String str) {
        this.log = str;
    }

    public void appendLineToLog(String str) {
        String str2 = this.log;
        if (str2 == null) {
            this.log = "";
        } else if (!str2.isEmpty()) {
            this.log += System.lineSeparator();
        }
        this.log += str;
    }

    public int getPercentageDone() {
        int size = this.perMaterialPercentageDone.entrySet().size();
        int iIntValue = 0;
        if (size == 0) {
            return 0;
        }
        for (Map.Entry<String, Integer> entry : this.perMaterialPercentageDone.entrySet()) {
            if (entry.getValue() != null) {
                iIntValue += entry.getValue().intValue();
            }
        }
        return iIntValue / size;
    }

    public void setMaterialPercentageDone(String str, Integer num) {
        this.perMaterialPercentageDone.put(str, num);
        if (num.intValue() == 100) {
            checkIfFinished();
        }
    }

    public void checkIfFinished() {
        if (this.status == Status.FINISHED) {
            return;
        }
        Iterator<Map.Entry<String, Integer>> it = this.perMaterialPercentageDone.entrySet().iterator();
        boolean z = true;
        while (it.hasNext()) {
            if (it.next().getValue().intValue() != 100) {
                z = false;
            }
        }
        if (z) {
            this.endTime = System.currentTimeMillis();
            this.status = Status.FINISHED;
            if (this.solution == null) {
                buildSolution();
            }
        }
    }

    public long getElapsedTime() {
        long j;
        long jCurrentTimeMillis = this.endTime;
        if (jCurrentTimeMillis == 0) {
            jCurrentTimeMillis = System.currentTimeMillis();
            j = this.startTime;
        } else {
            j = this.startTime;
        }
        return jCurrentTimeMillis - j;
    }

    public long getStartTime() {
        return this.startTime;
    }

    public long getEndTime() {
        return this.endTime;
    }

    public void setEndTime(long j) {
        this.endTime = j;
    }

    public boolean isMinTrimDimensionInfluenced() {
        return this.isMinTrimDimensionInfluenced;
    }

    public void setMinTrimDimensionInfluenced(boolean z) {
        this.isMinTrimDimensionInfluenced = z;
    }



    public List<TileDimensions> getNoMaterialTiles() {
        return this.noMaterialTiles;
    }

    public void setNoMaterialTiles(List<TileDimensions> list) {
        this.noMaterialTiles = list;
    }

    public synchronized void addThread(CutListThread cutListThread) {
        this.threads.add(cutListThread);
    }

    public synchronized int getNbrRunningThreads() {
        int i;
        Iterator<CutListThread> it = this.threads.iterator();
        i = 0;
        while (it.hasNext()) {
            if (Status.RUNNING.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public synchronized int getNbrQueuedThreads() {
        int i;
        Iterator<CutListThread> it = this.threads.iterator();
        i = 0;
        while (it.hasNext()) {
            if (Status.QUEUED.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public synchronized int getNbrFinishedThreads() {
        int i;
        Iterator<CutListThread> it = this.threads.iterator();
        i = 0;
        while (it.hasNext()) {
            if (Status.FINISHED.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public synchronized int getNbrFinishedThreads(String str) {
        int i;
        i = 0;
        for (CutListThread cutListThread : this.threads) {
            if (Status.FINISHED.equals(cutListThread.getStatus()) && cutListThread.getMaterial() != null && cutListThread.getMaterial().equals(str)) {
                i++;
            }
        }
        return i;
    }

    public synchronized int getNbrTerminatedThreads() {
        int i;
        Iterator<CutListThread> it = this.threads.iterator();
        i = 0;
        while (it.hasNext()) {
            if (Status.TERMINATED.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public synchronized int getNbrErrorThreads() {
        int i;
        Iterator<CutListThread> it = this.threads.iterator();
        i = 0;
        while (it.hasNext()) {
            if (Status.ERROR.equals(it.next().getStatus())) {
                i++;
            }
        }
        return i;
    }

    public synchronized int getMaxThreadProgressPercentage() {
        int percentageDone;
        percentageDone = 0;
        for (CutListThread cutListThread : this.threads) {
            if (cutListThread.getPercentageDone() > percentageDone) {
                percentageDone = cutListThread.getPercentageDone();
            }
        }
        return percentageDone;
    }

    public synchronized int getNbrTotalThreads() {
        return this.threads.size();
    }

    public double getFactor() {
        return this.factor;
    }

    public void setFactor(double d) {
        this.factor = d;
    }

    public long getLastQueried() {
        return this.lastQueried;
    }

    public void setLastQueried(long j) {
        this.lastQueried = j;
    }

    public Map<String, List<TileDimensions>> getTileDimensionsPerMaterial() {
        return this.tileDimensionsPerMaterial;
    }

    public void setTileDimensionsPerMaterial(Map<String, List<TileDimensions>> map) {
        this.tileDimensionsPerMaterial = map;
    }

    public Map<String, List<TileDimensions>> getStockDimensionsPerMaterial() {
        return this.stockDimensionsPerMaterial;
    }

    public void setStockDimensionsPerMaterial(Map<String, List<TileDimensions>> map) {
        this.stockDimensionsPerMaterial = map;
    }

    public boolean hasSolution() {
        CalculationResponse calculationResponse = this.solution;
        return (calculationResponse == null || calculationResponse.getPanels() == null || this.solution.getPanels().size() <= 0) ? false : true;
    }

    public boolean hasSolutionAllFit() {
        return hasSolution() && (this.solution.getNoFitPanels() == null || this.solution.getNoFitPanels().size() == 0);
    }

    public void buildSolution() {
        this.solution = new CalculationResponseBuilder().setTask(this).setCalculationRequest(this.calculationRequest).setSolutions(this.solutions).setNoStockMaterialPanels(this.noMaterialTiles).build();
    }
    
    public ClientInfo getClientInfo() {
        return this.clientInfo;
    }
    
    public void setClientInfo(ClientInfo clientInfo) {
        this.clientInfo = clientInfo;
    }
}
