package com.example.debug.engine;

import com.example.debug.engine.CutListOptimizerService;
import com.example.debug.engine.CutListThread;
import com.example.debug.engine.comparator.PriorityListFactory;
import com.example.debug.engine.comparator.SolutionComparatorFactory;
import com.example.debug.engine.console.ConsoleCutListLogger;
import com.example.debug.engine.model.CalculationRequest;
import com.example.debug.engine.model.CalculationSubmissionResult;
import com.example.debug.engine.model.ClientInfo;
import com.example.debug.engine.model.Configuration;
import com.example.debug.engine.model.GroupedTileDimensions;
import com.example.debug.engine.model.PerformanceThresholds;
import com.example.debug.engine.model.Solution;
import com.example.debug.engine.model.Stats;
import com.example.debug.engine.model.Status;
import com.example.debug.engine.model.Task;
import com.example.debug.engine.model.TaskStatusResponse;
import com.example.debug.engine.model.TileDimensions;
import com.example.debug.engine.stock.StockPanelPicker;
import com.example.debug.engine.stock.StockSolution;
// import com.fasterxml.jackson.databind.ObjectMapper;
import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.Date;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.Executors;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicLong;
// import org.slf4j.Logger;
// import org.slf4j.LoggerFactory;
import java.util.Map.Entry;

/* loaded from: classes.dex */
public class CutListOptimizerServiceImpl implements CutListOptimizerService {
    private static final int MAX_PERMUTATION_ITERATIONS = 1000;
    private static final int MAX_STOCK_ITERATIONS = 1000;
    private CutListLogger cutListLogger;
    private RunningTasks runningTasks;
    private ThreadPoolExecutor taskExecutor;
    private WatchDog watchDog;

    private static final ConsoleCutListLogger logger = new ConsoleCutListLogger();
    private static AtomicLong taskIdCounter = new AtomicLong();
    private static int MAX_ALLOWED_DIGITS = 6;
    private static int THREAD_QUEUE_SIZE = 1000;
    public static int MAX_ACTIVE_THREADS_PER_TASK = 5;
    private static int MAX_PERMUTATIONS_WITH_SOLUTION = 150;
    private static CutListOptimizerServiceImpl instance = new CutListOptimizerServiceImpl();
    private final DateFormat dateFormat = new SimpleDateFormat("yyyyMMddHHmm", Locale.ENGLISH);
    private boolean allowMultipleTasksPerClient = false;

    private CutListOptimizerServiceImpl() {
    }

    public static CutListOptimizerService getInstance() {
        return instance;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public void setAllowMultipleTasksPerClient(boolean z) {
        this.allowMultipleTasksPerClient = z;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public void setCutListLogger(CutListLogger cutListLogger) {
        this.cutListLogger = cutListLogger;
        this.watchDog.setCutListLogger(cutListLogger);
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public Stats getStats() {
        Stats stats = new Stats();
        stats.setNbrIdleTasks(this.runningTasks.geNbrIdleTasks());
        stats.setNbrRunningTasks(this.runningTasks.geNbrRunningTasks());
        stats.setNbrFinishedTasks(this.runningTasks.geNbrFinishedTasks());
        stats.setNbrStoppedTasks(this.runningTasks.geNbrStoppedTasks());
        stats.setNbrTerminatedTasks(this.runningTasks.geNbrTerminatedTasks());
        stats.setNbrErrorTasks(this.runningTasks.geNbrErrorTasks());
        stats.setNbrRunningThreads(this.taskExecutor.getActiveCount());
        stats.setNbrQueuedThreads(this.taskExecutor.getQueue().size());
        stats.setNbrFinishedThreads(this.taskExecutor.getCompletedTaskCount());
        stats.setTaskReports(this.watchDog.getTaskReports());
        return stats;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public void init(int i) {
        this.runningTasks = RunningTasks.getInstance();
        RejectedExecutionHandlerImpl rejectedExecutionHandlerImpl = new RejectedExecutionHandlerImpl(this.runningTasks);
        this.taskExecutor = new ThreadPoolExecutor(i, i, 10L, TimeUnit.SECONDS, new ArrayBlockingQueue(THREAD_QUEUE_SIZE), Executors.defaultThreadFactory(), rejectedExecutionHandlerImpl);
        WatchDog watchDog = new WatchDog();
        this.watchDog = watchDog;
        watchDog.setCutListLogger(this.cutListLogger);
        this.watchDog.setRunningTasks(this.runningTasks);
        this.watchDog.setTaskExecutor(this.taskExecutor);
        this.watchDog.setCutListOptimizerService(this);
        new Thread(this.watchDog, "watchDog").start();
    }

    public void destroy() {
        this.taskExecutor.shutdown();
    }

    private int removeDuplicatedPermutations(List<List<TileDimensions>> list) {
        ArrayList arrayList = new ArrayList();
        Iterator<List<TileDimensions>> it = list.iterator();
        int i = 0;
        while (it.hasNext()) {
            Iterator<TileDimensions> it2 = it.next().iterator();
            int iDimensionsBasedHashCode = 0;
            while (it2.hasNext()) {
                iDimensionsBasedHashCode = (iDimensionsBasedHashCode * 31) + it2.next().dimensionsBasedHashCode();
            }
            if (arrayList.contains(Integer.valueOf(iDimensionsBasedHashCode))) {
                it.remove();
                i++;
            } else {
                arrayList.add(Integer.valueOf(iDimensionsBasedHashCode));
            }
        }
        return i;
    }

    private boolean isOneDimensionalOptimization(List<TileDimensions> list, List<TileDimensions> list2) {
        ArrayList arrayList = new ArrayList();
        arrayList.add(Integer.valueOf(list.get(0).getWidth()));
        arrayList.add(Integer.valueOf(list.get(0).getHeight()));
        for (TileDimensions tileDimensions : list) {
            if (((Integer) arrayList.get(0)).intValue() != tileDimensions.getWidth() && ((Integer) arrayList.get(0)).intValue() != tileDimensions.getHeight()) {
                arrayList.remove(0);
            }
            if (arrayList.size() == 2 && ((Integer) arrayList.get(1)).intValue() != tileDimensions.getWidth() && ((Integer) arrayList.get(1)).intValue() != tileDimensions.getHeight()) {
                arrayList.remove(1);
            }
            if (arrayList.size() == 0) {
                return false;
            }
        }
        for (TileDimensions tileDimensions2 : list2) {
            if (((Integer) arrayList.get(0)).intValue() != tileDimensions2.getWidth() && ((Integer) arrayList.get(0)).intValue() != tileDimensions2.getHeight()) {
                arrayList.remove(0);
            }
            if (arrayList.size() == 2 && ((Integer) arrayList.get(1)).intValue() != tileDimensions2.getWidth() && ((Integer) arrayList.get(1)).intValue() != tileDimensions2.getHeight()) {
                arrayList.remove(1);
            }
            if (arrayList.size() == 0) {
                return false;
            }
        }
        return true;
    }

    private List<GroupedTileDimensions> generateGroups(List<TileDimensions> list, List<TileDimensions> list2, Task task) {
        HashMap<String, Integer> map = new HashMap<String, Integer>();
        Iterator<TileDimensions> it = list.iterator();
        while (true) {
            int iIntValue = 1;
            if (!it.hasNext()) {
                break;
            }
            String string = it.next().toString();
            if (map.get(string) != null) {
                iIntValue = 1 + ((Integer) map.get(string)).intValue();
            }
            map.put(string, Integer.valueOf(iIntValue));
        }
        StringBuilder sb = new StringBuilder();
        for (String str : map.keySet()) {
            sb.append(str + "*" + map.get(str) + " ");
        }
        int i = 0;
        logger.trace("Task[{}] TotalNbrTiles[{}] Tiles: {}", task.getId(), Integer.valueOf(list.size()), sb);
        int iMax = Math.max(list.size() / 100, 1);
        if (isOneDimensionalOptimization(list, list2)) {
            this.cutListLogger.info(task.getClientInfo().getId(), task.getId(), "Task is one dimensional optimization");
            iMax = 1;
        }
        ArrayList arrayList = new ArrayList();
        HashMap map2 = new HashMap();
        for (TileDimensions tileDimensions : list) {
            String str2 = tileDimensions.toString() + i;
            map2.put(str2, Integer.valueOf(map2.get(str2) != null ? ((Integer) map2.get(str2)).intValue() + 1 : 1));
            arrayList.add(new GroupedTileDimensions(tileDimensions, i));
            if (((Integer) map.get(tileDimensions.toString())).intValue() > iMax && ((Integer) map2.get(str2)).intValue() > ((Integer) map.get(tileDimensions.toString())).intValue() / 4) {
                logger.debug("Task[" + task.getId() + "] Splitting panel set [" + tileDimensions.dimensionsToString() + "] with [" + map.get(tileDimensions.toString()) + "] units into two groups");
                i++;
            }
        }
        return arrayList;
    }

    private <T> HashMap<T, Integer> getDistinctGroupedTileDimensions(List<T> list, Configuration configuration) {
        HashMap<T, Integer> map = new HashMap<>();
        for (T t : list) {
            int iIntValue = 1;
            if (map.get(t) != null) {
                iIntValue = 1 + map.get(t).intValue();
            }
            map.put(t, Integer.valueOf(iIntValue));
        }
        return map;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public List<String> getTasks(String str, Status status) {
        ArrayList arrayList = new ArrayList();
        try {
            for (Task task : this.runningTasks.getTasks()) {
                if (status.equals(task.getStatus()) && str.equals(task.getClientInfo().getId())) {
                    arrayList.add(task.getId());
                }
            }
        } catch (Exception e) {
            this.cutListLogger.error("Error fetching running task ids for client " + str + " with " + status + " status", e);
        }
        return arrayList;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public CalculationSubmissionResult submitTask(final CalculationRequest calculationRequest) {
        String strWriteValueAsString;
        int count = 0;
        PerformanceThresholds performanceThresholds = null;
        if (calculationRequest.getConfiguration() != null) {
            performanceThresholds = calculationRequest.getConfiguration().getPerformanceThresholds();
        }
        if (performanceThresholds == null) {
            performanceThresholds = new PerformanceThresholds();
            performanceThresholds.setMaxSimultaneousThreads(5);
            performanceThresholds.setThreadCheckInterval(1000L);
        }
        ClientInfo clientInfo = new ClientInfo();
        try {
            strWriteValueAsString = calculationRequest.toString();
        } catch (Exception unused) {
            this.cutListLogger.error("Unable to stringify calculation request");
            strWriteValueAsString = null;
        }
        try {
            clientInfo = calculationRequest.getClientInfo();
            this.cutListLogger.logClient(clientInfo);
        } catch (Exception e) {
            this.cutListLogger.error("Error while logging client info" + System.lineSeparator() + "calculationRequest: " + strWriteValueAsString, e);
        }
        int count2 = 0;
        try {
            if (!this.allowMultipleTasksPerClient) {
                int i = 0;
                for (Task task : this.runningTasks.getTasks()) {
                    if (task.getStatus().equals(Status.RUNNING) && task.getClientInfo().getId().equals(calculationRequest.getClientInfo().getId())) {
                        i++;
                    }
                }
                if (i >= performanceThresholds.getMaxSimultaneousTasks()) {
                    this.cutListLogger.warn(clientInfo.getId(), "Rejecting user task due to [" + i + "] already running task(s)");
                    return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.TASK_ALREADY_RUNNING.getStringValue());
                }
            }
        } catch (Exception e2) {
            this.cutListLogger.error(clientInfo.getId(), "Error evaluating if calculation is allowed" + System.lineSeparator() + "calculationRequest: " + strWriteValueAsString, e2);
        }
        try {
            count = 0;
            for (CalculationRequest.Panel panel : calculationRequest.getPanels()) {
                if (panel.isValid()) {
                    count += panel.getCount();
                }
            }
        } catch (Exception e3) {
            this.cutListLogger.error(clientInfo.getId(), "Error Validating request" + System.lineSeparator() + "calculationRequest: " + strWriteValueAsString, e3);
        }
        if (count == 0) {
            return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.INVALID_TILES.getStringValue());
        }
        if (count > 5000) {
            return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.TOO_MANY_PANELS.getStringValue());
        }
        for (CalculationRequest.Panel panel2 : calculationRequest.getStockPanels()) {
            if (panel2.isValid()) {
                count2 += panel2.getCount();
            }
        }
        if (count2 == 0) {
            return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.INVALID_STOCK_TILES.getStringValue());
        }
        if (count2 > 5000) {
            return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.TOO_MANY_STOCK_PANELS.getStringValue());
        }
        try {
            final String str = this.dateFormat.format(new Date()) + taskIdCounter.getAndIncrement();
            new Thread(new Runnable() { // from class: com.cutlistoptimizer.engine.CutListOptimizerServiceImpl.1
                @Override // java.lang.Runnable
                public void run() {
                    CutListOptimizerServiceImpl.this.compute(calculationRequest, str);
                }
            }).start();
            return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.OK.getStringValue(), str);
        } catch (Exception e4) {
            this.cutListLogger.error(clientInfo.getId(), "Error submitting calculation request" + System.lineSeparator() + "calculationRequest: " + strWriteValueAsString, e4);
            return new CalculationSubmissionResult(CutListOptimizerService.StatusCode.SERVER_UNAVAILABLE.getStringValue());
        }
    }

    private ArrayList<TileDimensions> groupedTileDimensionsList2TileDimensionsList(final List<GroupedTileDimensions> list, List<? extends TileDimensions> list2) {
        ArrayList<TileDimensions> arrayList = new ArrayList<>(list2);
        Collections.sort(arrayList, new Comparator<TileDimensions>() { // from class: com.cutlistoptimizer.engine.CutListOptimizerServiceImpl.2
            @Override // java.util.Comparator
            public int compare(TileDimensions tileDimensions, TileDimensions tileDimensions2) {
                return Integer.valueOf(list.indexOf(tileDimensions)).compareTo(Integer.valueOf(list.indexOf(tileDimensions2)));
            }
        });
        return arrayList;
    }

    Map<String, List<TileDimensions>> getTileDimensionsPerMaterial(List<TileDimensions> list) {
        HashMap<String, List<TileDimensions>> map = new HashMap<String, List<TileDimensions>>();
        for (TileDimensions tileDimensions : list) {
            if (map.containsKey(tileDimensions.getMaterial())) {
                ((List) map.get(tileDimensions.getMaterial())).add(tileDimensions);
            } else {
                ArrayList arrayList = new ArrayList();
                arrayList.add(tileDimensions);
                map.put(tileDimensions.getMaterial(), arrayList);
            }
        }
        return map;
    }

    private int getNbrDecimalPlaces(String str) {
        int iIndexOf;
        if (str == null || (iIndexOf = str.indexOf(46)) == -1) {
            return 0;
        }
        return (str.length() - iIndexOf) - 1;
    }

    private int getNbrIntegerPlaces(String str) {
        if (str == null) {
            return 0;
        }
        if (str.indexOf(46) == -1) {
            return str.length();
        }
        return (str.length() - getNbrDecimalPlaces(str)) - 1;
    }

    private int getMaxNbrDecimalPlaces(List<CalculationRequest.Panel> list) {
        int iMax = 0;
        for (CalculationRequest.Panel panel : list) {
            if (panel.isValid()) {
                iMax = Math.max(Math.max(iMax, getNbrDecimalPlaces(panel.getWidth())), getNbrDecimalPlaces(panel.getHeight()));
            }
        }
        return iMax;
    }

    private int getMaxNbrIntegerPlaces(List<CalculationRequest.Panel> list) {
        int iMax = 0;
        for (CalculationRequest.Panel panel : list) {
            if (panel.isValid()) {
                iMax = Math.max(Math.max(iMax, getNbrIntegerPlaces(panel.getWidth())), getNbrIntegerPlaces(panel.getHeight()));
            }
        }
        return iMax;
    }

    /* JADX INFO: Access modifiers changed from: private */
    /* JADX WARN: Type inference failed for: r13v0, types: [com.cutlistoptimizer.engine.CutListOptimizerServiceImpl$3] */
    public void compute(CalculationRequest calculationRequest, String str) {
        List<CalculationRequest.Panel> panels = calculationRequest.getPanels();
        List<CalculationRequest.Panel> stockPanels = calculationRequest.getStockPanels();
        final Configuration configuration = calculationRequest.getConfiguration();
        ClientInfo clientInfo = calculationRequest.getClientInfo();
        ArrayList arrayList = new ArrayList();
        ArrayList arrayList2 = new ArrayList();
        int maxNbrDecimalPlaces = getMaxNbrDecimalPlaces(panels);
        int maxNbrDecimalPlaces2 = getMaxNbrDecimalPlaces(stockPanels);
        int iMax = Math.max(Math.max(Math.max(maxNbrDecimalPlaces, maxNbrDecimalPlaces2), getNbrDecimalPlaces(calculationRequest.getConfiguration().getCutThickness())), getNbrDecimalPlaces(calculationRequest.getConfiguration().getMinTrimDimension()));
        int maxNbrIntegerPlaces = getMaxNbrIntegerPlaces(panels);
        int maxNbrIntegerPlaces2 = getMaxNbrIntegerPlaces(stockPanels);
        int iMax2 = Math.max(Math.max(Math.max(maxNbrIntegerPlaces, maxNbrIntegerPlaces2), getNbrIntegerPlaces(calculationRequest.getConfiguration().getCutThickness())), getNbrIntegerPlaces(calculationRequest.getConfiguration().getMinTrimDimension()));
        if (iMax + iMax2 > MAX_ALLOWED_DIGITS) {
            this.cutListLogger.warn(calculationRequest.getClientInfo().getId(), "Maximum allowed digits exceeded: maxDecimalPlaces[" + iMax + "] maxIntegerPlaces[" + iMax2 + "] maxAllowedDigits[" + MAX_ALLOWED_DIGITS + "]");
            iMax = Math.max(MAX_ALLOWED_DIGITS - iMax2, 0);
        }
        double dPow = Math.pow(10.0d, iMax);
        for (CalculationRequest.Panel panel : panels) {
            if (panel.isValid()) {
                for (int i = 0; i < panel.getCount(); i++) {
                    arrayList.add(new TileDimensions(panel.getId(), (int) Math.round(Double.parseDouble(panel.getWidth()) * dPow), (int) Math.round(Double.parseDouble(panel.getHeight()) * dPow), panel.getMaterial(), panel.getOrientation(), panel.getLabel()));
                }
            }
        }
        for (CalculationRequest.Panel panel2 : stockPanels) {
            if (panel2.isValid()) {
                for (int i2 = 0; i2 < panel2.getCount(); i2++) {
                    arrayList2.add(new TileDimensions(panel2.getId(), (int) Math.round(Double.parseDouble(panel2.getWidth()) * dPow), (int) Math.round(Double.parseDouble(panel2.getHeight()) * dPow), panel2.getMaterial(), panel2.getOrientation(), panel2.getLabel()));
                }
            }
        }
        final Task task = new Task(str);
        task.setCalculationRequest(calculationRequest);
        task.setClientInfo(clientInfo);
        task.setFactor(dPow);
        task.buildSolution();
        this.runningTasks.addTask(task);
        this.cutListLogger.logExecution(task);
        Map<String, List<TileDimensions>> tileDimensionsPerMaterial = getTileDimensionsPerMaterial(arrayList);
        Map<String, List<TileDimensions>> tileDimensionsPerMaterial2 = getTileDimensionsPerMaterial(arrayList2);
        task.setTileDimensionsPerMaterial(tileDimensionsPerMaterial);
        task.setStockDimensionsPerMaterial(tileDimensionsPerMaterial2);
        HashSet<String> hashSet = new HashSet();
        hashSet.addAll(tileDimensionsPerMaterial.keySet());
        hashSet.addAll(tileDimensionsPerMaterial2.keySet());
        for (String str2 : hashSet) {
            if (tileDimensionsPerMaterial.get(str2) != null) {
                if (tileDimensionsPerMaterial2.get(str2) != null) {
                    task.addMaterialToCompute(str2);
                } else {
                    task.getNoMaterialTiles().addAll(tileDimensionsPerMaterial.get(str2));
                }
            }
        }
        for (final String str3 : hashSet) {
            final List<TileDimensions> list = tileDimensionsPerMaterial.get(str3);
            final List<TileDimensions> list2 = tileDimensionsPerMaterial2.get(str3);
            if (list != null && list2 != null) {
                new Thread() { // from class: com.cutlistoptimizer.engine.CutListOptimizerServiceImpl.3
                    @Override // java.lang.Thread, java.lang.Runnable
                    public void run() {
                        try {
                            CutListOptimizerServiceImpl.this.compute(list, list2, configuration, task, str3);
                        } catch (InterruptedException e) {
                            Thread.currentThread().interrupt();
                        }
                    }
                }.start();
            }
        }
        task.checkIfFinished();
    }

    private boolean isThreadEligibleToStart(String str, Task task, String str2) {
        try {
            Iterator<Integer> it = task.getThreadGroupRankings(str2).values().iterator();
            int iIntValue = 0;
            while (it.hasNext()) {
                iIntValue += it.next().intValue();
            }
            if (task.getNbrFinishedThreads(str2) < 10) {
                return true;
            }
            return (task.getThreadGroupRankings(str2).get(str) != null ? task.getThreadGroupRankings(str2).get(str).intValue() : 0) > iIntValue / 5;
        } catch (Exception e) {
            this.cutListLogger.fatal("Error evaluating if thread of material[" + str2 + "] group[" + str + "] is eligible to start", e);
            return true;
        }
    }

    /* JADX INFO: Access modifiers changed from: private */
    public void compute(List<TileDimensions> list, List<TileDimensions> list2, final Configuration configuration, final Task task, String str) throws InterruptedException {
        ArrayList arrayList;
        Task task2 = task;
        final String str2 = str;
        PerformanceThresholds performanceThresholds = new PerformanceThresholds();
        if (configuration.getPerformanceThresholds() != null) {
            performanceThresholds.setMaxSimultaneousThreads(configuration.getPerformanceThresholds().getMaxSimultaneousThreads());
            performanceThresholds.setThreadCheckInterval(configuration.getPerformanceThresholds().getThreadCheckInterval());
        } else {
            this.cutListLogger.warn(task.getClientInfo().getId(), task.getId(), "No performance thresholds specified");
            performanceThresholds.setMaxSimultaneousThreads(5);
            performanceThresholds.setThreadCheckInterval(1000L);
        }
        final List<Solution> solutions = task.getSolutions(str);
        StringBuilder sb = new StringBuilder();
        List<GroupedTileDimensions> listGenerateGroups = generateGroups(list, list2, task2);
        HashMap<GroupedTileDimensions, Integer> distinctGroupedTileDimensions = getDistinctGroupedTileDimensions(listGenerateGroups, configuration);
        int i = 0;
        sb.setLength(0);
        int i2 = 0;
        for (Map.Entry<GroupedTileDimensions, Integer> entry : distinctGroupedTileDimensions.entrySet()) {
            i2++;
            sb.append(" group[" + i2 + ":" + entry.getKey() + "*" + entry.getValue() + "] ");
        }
        logger.debug("Task[" + task.getId() + "] Calculating permutations...");
        ArrayList arrayList2 = new ArrayList(distinctGroupedTileDimensions.keySet());
        try {
            Collections.sort(arrayList2, new Comparator<GroupedTileDimensions>() { // from class: com.cutlistoptimizer.engine.CutListOptimizerServiceImpl.4
                @Override // java.util.Comparator
                public int compare(GroupedTileDimensions groupedTileDimensions, GroupedTileDimensions groupedTileDimensions2) {
                    return (int) (groupedTileDimensions2.getArea() - groupedTileDimensions.getArea());
                }
            });
        } catch (Exception e) {
            this.cutListLogger.error("Error sorting distinct tile dimensions", e);
        }
        if (arrayList2.size() > 7) {
            ArrayList arrayList3 = new ArrayList(arrayList2.subList(0, 7));
            arrayList = new ArrayList(arrayList2.subList(7, arrayList2.size()));
            arrayList2 = arrayList3;
        } else {
            arrayList = new ArrayList();
        }
        List listGeneratePermutations = Arrangement.generatePermutations(arrayList2);
        Iterator it = listGeneratePermutations.iterator();
        while (it.hasNext()) {
            ((List) it.next()).addAll(arrayList);
        }
        logger.debug("Task[" + task.getId() + "] Sorting tiles according to permutations...");
        ArrayList arrayList4 = new ArrayList();
        Iterator it2 = listGeneratePermutations.iterator();
        while (it2.hasNext()) {
            arrayList4.add(groupedTileDimensionsList2TileDimensionsList((List) it2.next(), listGenerateGroups));
        }
        logger.debug("Removing duplicated permutations...");
        removeDuplicatedPermutations(arrayList4);
        task.setRunningStatus();
        StockPanelPicker stockPanelPicker = new StockPanelPicker(list, list2, task2, configuration.isUseSingleStockUnit() ? 1 : null);
        stockPanelPicker.init();
        int i3 = 100;
        int optimizationFactor = configuration.getOptimizationFactor() > 0.0d ? (int) (100 * configuration.getOptimizationFactor()) : 100;
        String str3 = "]";
        if (list.size() > 100) {
            optimizationFactor = (int) (optimizationFactor * (0.5d / (list.size() / 100)));
            this.cutListLogger.info(task.getClientInfo().getId(), task.getId(), "Limiting solution pool elements to [" + optimizationFactor + "]");
        }
        final int i4 = optimizationFactor;
        PermutationThreadSpawner permutationThreadSpawner = new PermutationThreadSpawner();
        ProgressTracker progressTracker = new ProgressTracker(permutationThreadSpawner, arrayList4.size(), task2, str2);
        permutationThreadSpawner.setProgressTracker(progressTracker);
        permutationThreadSpawner.setMaxAliveSpawnerThreads(performanceThresholds.getMaxSimultaneousThreads());
        permutationThreadSpawner.setIntervalBetweenMaxAliveCheck(performanceThresholds.getThreadCheckInterval());
        while (true) {
            if (i >= arrayList4.size()) {
                break;
            }
            final List<TileDimensions> list3 = (List<TileDimensions>) arrayList4.get(i);
            if (!task.isRunning()) {
                logger.debug("Tasked no longer has running status. Stopping permutationSpawnerThread spawner at permutationIdx[" + i + str3);
                break;
            }
            if (task.hasSolutionAllFit() && permutationThreadSpawner.getNbrTotalThreads() > MAX_PERMUTATIONS_WITH_SOLUTION) {
                task2.setMaterialPercentageDone(str2, Integer.valueOf(i3));
                logger.debug("Task has solution and spawned max permutations threads");
                break;
            }
            final StockPanelPicker stockPanelPicker2 = stockPanelPicker;
            final int i5 = i;
            final ProgressTracker progressTracker2 = progressTracker;
            PermutationThreadSpawner permutationThreadSpawner2 = permutationThreadSpawner;
            final ArrayList arrayList5 = arrayList4;
            ArrayList arrayList6 = arrayList4;
            final PerformanceThresholds performanceThresholds2 = performanceThresholds;
            final String materialStr = str;
            permutationThreadSpawner2.spawn(new Thread(new Runnable() { // from class: com.cutlistoptimizer.engine.CutListOptimizerServiceImpl$$ExternalSyntheticLambda0
                @Override // java.lang.Runnable
                public final void run() {
                    try {
                        CutListOptimizerServiceImpl.this.m301x52dbbde3(stockPanelPicker2, i5, task, solutions, arrayList5, configuration, list3, i4, performanceThresholds2, progressTracker2, materialStr);
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                    }
                }
            }));
            i++;
            permutationThreadSpawner = permutationThreadSpawner2;
            stockPanelPicker = stockPanelPicker;
            progressTracker = progressTracker2;
            str3 = str3;
            arrayList4 = arrayList6;
            performanceThresholds = performanceThresholds;
            i3 = 100;
            task2 = task;
        }
        PermutationThreadSpawner permutationThreadSpawner3 = permutationThreadSpawner;
        String str4 = str3;
        while (true) {
            try {
                Thread.sleep(1000L);
            } catch (InterruptedException e2) {
                logger.error("Interrupted", (Throwable) e2);
            }
            ConsoleCutListLogger logger2 = logger;
            StringBuilder sb2 = new StringBuilder("Waiting for permutationSpawner[");
            sb2.append(permutationThreadSpawner3.getNbrUnfinishedThreads());
            sb2.append("] queued[");
            sb2.append(task.getNbrQueuedThreads());
            sb2.append("] running[");
            sb2.append(task.getNbrRunningThreads());
            String str5 = str4;
            sb2.append(str5);
            logger2.debug(sb2.toString());
            if (permutationThreadSpawner3.getNbrUnfinishedThreads() <= 0 && task.getNbrRunningThreads() + task.getNbrQueuedThreads() <= 0) {
                break;
            } else {
                str4 = str5;
            }
        }
        if (task.getStatus().equals(Status.RUNNING)) {
            task.setMaterialPercentageDone(str, 100);
        }
    }

    /* renamed from: lambda$compute$0$com-cutlistoptimizer-engine-CutListOptimizerServiceImpl, reason: not valid java name */
    /* synthetic */ void m301x52dbbde3(StockPanelPicker stockPanelPicker, int i, Task task, List list, List list2, Configuration configuration, List list3, int i2, PerformanceThresholds performanceThresholds, ProgressTracker progressTracker, String str) throws InterruptedException {
        int iRound;
        int iRound2;
        int i3 = 0;
        int i4 = 0;
        while (i4 < 1000) {
            StockSolution stockSolution = stockPanelPicker.getStockSolution(i4);
            if (stockSolution == null) {
                logger.debug("No more possible stock solutions: stockSolution[" + i4 + "] permutationIdx[" + i + "]");
                return;
            }
            if (!task.isRunning()) {
                logger.debug("Task no longer has running status. Stopping stock loop for permutationIdx[" + i + "]");
                return;
            }
            if (!task.hasSolutionAllFit() || list.size() <= 0 || ((Solution) list.get(i3)).getMosaics().size() != 1 || ((Solution) list.get(i3)).getTotalArea() >= stockSolution.getTotalArea()) {
                logger.debug("Starting permutationIdx[" + i + "/" + list2.size() + "] with stock solution [" + i4 + "] {nbrPanels[" + stockSolution.getStockTileDimensions().size() + "] area[" + stockSolution.getTotalArea() + "] " + stockSolution.toString() + "}");
                List<Comparator> solutionComparatorList = SolutionComparatorFactory.getSolutionComparatorList(PriorityListFactory.getFinalSolutionPrioritizedComparatorList(configuration));
                try {
                    String cutThicknessStr = configuration.getCutThickness();
                    if (cutThicknessStr != null && !cutThicknessStr.trim().isEmpty()) {
                        iRound = (int) Math.round(Double.parseDouble(cutThicknessStr) * task.getFactor());
                    } else {
                        iRound = 0;
                    }
                } catch (Exception unused) {
                    logger.error("Error parsing cut thickness value: [" + configuration.getCutThickness() + "]");
                    iRound = 0;
                }
                try {
                    String minTrimStr = configuration.getMinTrimDimension();
                    if (minTrimStr != null && !minTrimStr.trim().isEmpty()) {
                        iRound2 = (int) Math.round(Double.parseDouble(minTrimStr) * task.getFactor());
                    } else {
                        iRound2 = 0;
                    }
                } catch (Exception unused2) {
                    logger.error("Error parsing minimum trim dimension value: [" + configuration.getMinTrimDimension() + "]");
                    iRound2 = 0;
                }
                CutListThreadBuilder cutListLogger = new CutListThreadBuilder().setAuxInfo("stock[" + i4 + "] permutation[" + i + "]").setAllSolutions(list).setTiles(list3).setConfiguration(configuration).setCutThickness(iRound).setMinTrimDimension(iRound2).setFinalSolutionPrioritizedComparators(solutionComparatorList).setTask(task).setAccuracyFactor(i2).setStockSolution(stockSolution).setCutListLogger(this.cutListLogger);
                while (task.getNbrRunningThreads() + task.getNbrQueuedThreads() >= performanceThresholds.getMaxSimultaneousThreads()) {
                    try {
                        logger.trace("Maximum number of active threads per task reached: running[" + task.getNbrRunningThreads() + "] queued[" + task.getNbrQueuedThreads() + "]");
                        progressTracker.refreshTaskStatusInfo();
                        Thread.sleep(performanceThresholds.getThreadCheckInterval());
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }
                if (isThreadEligibleToStart("AREA", task, str) && configuration.getCutOrientationPreference() == 0) {
                    cutListLogger.setGroup("AREA");
                    cutListLogger.setThreadPrioritizedComparators(solutionComparatorList);
                    cutListLogger.setFirstCutOrientation(CutDirection.BOTH);
                    this.taskExecutor.execute(cutListLogger.build());
                }
                if (isThreadEligibleToStart("AREA_HCUTS_1ST", task, str) && (configuration.getCutOrientationPreference() == 0 || configuration.getCutOrientationPreference() == 1)) {
                    cutListLogger.setGroup("AREA_HCUTS_1ST");
                    cutListLogger.setThreadPrioritizedComparators(solutionComparatorList);
                    cutListLogger.setFirstCutOrientation(CutDirection.HORIZONTAL);
                    this.taskExecutor.execute(cutListLogger.build());
                }
                if (isThreadEligibleToStart("AREA_VCUTS_1ST", task, str) && (configuration.getCutOrientationPreference() == 0 || configuration.getCutOrientationPreference() == 2)) {
                    cutListLogger.setGroup("AREA_VCUTS_1ST");
                    cutListLogger.setThreadPrioritizedComparators(solutionComparatorList);
                    cutListLogger.setFirstCutOrientation(CutDirection.VERTICAL);
                    this.taskExecutor.execute(cutListLogger.build());
                }
            } else {
                logger.debug("Stopping stock loop for permutationIdx[" + i + "/" + list2.size() + "] at stock solution " + stockSolution.toString() + " with area [" + stockSolution.getTotalArea() + "] because there's already an all fit solution using stock solution with area [" + ((Solution) list.get(i3)).getTotalArea() + "]");
            }
            i4++;
            i3 = 0;
        }
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public TaskStatusResponse getTaskStatus(String str) {
        Task task = this.runningTasks.getTask(str);
        if (task == null) {
            return null;
        }
        task.buildSolution();
        task.setLastQueried(System.currentTimeMillis());
        TaskStatusResponse taskStatusResponse = new TaskStatusResponse();
        taskStatusResponse.setStatus(task.getStatus().toString());
        taskStatusResponse.setInitPercentage(task.getMaxThreadProgressPercentage());
        taskStatusResponse.setPercentageDone(task.getPercentageDone());
        taskStatusResponse.setSolution(task.getSolution());
        return taskStatusResponse;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public TaskStatusResponse stopTask(String str) {
        Task task = this.runningTasks.getTask(str);
        if (task == null) {
            return null;
        }
        if (task.stop() != 0) {
            this.cutListLogger.warn(task.getClientInfo().getId(), task.getId(), "Unable to stop task. Current status is: " + task.getStatus());
        }
        TaskStatusResponse taskStatusResponse = new TaskStatusResponse();
        taskStatusResponse.setStatus(task.getStatus().toString());
        taskStatusResponse.setInitPercentage(task.getMaxThreadProgressPercentage());
        taskStatusResponse.setPercentageDone(task.getPercentageDone());
        taskStatusResponse.setSolution(task.getSolution());
        return taskStatusResponse;
    }

    @Override // com.cutlistoptimizer.engine.CutListOptimizerService
    public int terminateTask(String str) {
        Task task = this.runningTasks.getTask(str);
        if (task == null) {
            return -1;
        }
        int iTerminate = task.terminate();
        if (iTerminate != 0) {
            this.cutListLogger.warn("Unable to terminate task. Current status is: " + task.getStatus());
        }
        return iTerminate;
    }
}
