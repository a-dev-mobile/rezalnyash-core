

package com.example.debug.api;

import com.example.debug.api.dto.*;
import java.util.stream.Collectors;
import com.example.debug.engine.CutListOptimizerService;
import com.example.debug.engine.model.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;

import java.io.*;
import java.net.*;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class CuttingOptimizationController {
    private final CutListOptimizerService optimizerService;
    private final ObjectMapper objectMapper;
    private final Map<String, TaskMonitor> taskMonitors = new ConcurrentHashMap<>();
    private final ScheduledExecutorService scheduler = Executors.newScheduledThreadPool(2);
    
    private HttpServer server;
    private int port = 8080;
    
    private static class TaskMonitor {
        private volatile boolean stopRequested = false;
        private volatile TaskStatusResponse lastStatus;
        private volatile long lastUpdateTime = System.currentTimeMillis();
        
        public boolean isStopRequested() { return stopRequested; }
        public void requestStop() { this.stopRequested = true; }
        public TaskStatusResponse getLastStatus() { return lastStatus; }
        public void setLastStatus(TaskStatusResponse status) { 
            this.lastStatus = status;
            this.lastUpdateTime = System.currentTimeMillis();
        }
        public long getLastUpdateTime() { return lastUpdateTime; }
    }
    
    public CuttingOptimizationController(CutListOptimizerService optimizerService) {
        this.optimizerService = optimizerService;
        this.objectMapper = new ObjectMapper();
        
        scheduler.scheduleWithFixedDelay(this::cleanupOldTasks, 60, 60, TimeUnit.SECONDS);
    }
    
    public void start() throws IOException {
        server = HttpServer.create(new InetSocketAddress(port), 0);
        
        server.createContext("/api/tasks/optimize", this::handleOptimizeRequest);
        server.createContext("/api/tasks/status/", this::handleStatusRequest);
        server.createContext("/api/tasks/stop/", this::handleStopRequest);
        server.createContext("/api/tasks/stats", this::handleStatsRequest);
        
        server.setExecutor(Executors.newFixedThreadPool(4));
        server.start();
        
        System.out.println("API Server started on port " + port);
    }
    
    public void stop() {
        if (server != null) {
            server.stop(5);
        }
        scheduler.shutdown();
    }
    
    private void handleOptimizeRequest(HttpExchange exchange) throws IOException {
        if (!"POST".equals(exchange.getRequestMethod())) {
            sendResponse(exchange, 405, new ApiResponse("error", "Method not allowed", null));
            return;
        }
        
        try {
            String requestBody = readRequestBody(exchange);
            OptimizeRequest request = objectMapper.readValue(requestBody, OptimizeRequest.class);
            
            CalculationRequest calcRequest = convertToCalculationRequest(request);
            CalculationSubmissionResult result = optimizerService.submitTask(calcRequest);
            
            if ("0".equals(result.getStatusCode())) {
                String taskId = result.getTaskId();
                TaskMonitor monitor = new TaskMonitor();
                taskMonitors.put(taskId, monitor);
                
                startTaskMonitoring(taskId, request.getAcceptableQuality());
                
                OptimizeResponse response = new OptimizeResponse(taskId, "SUBMITTED");
                sendResponse(exchange, 200, new ApiResponse("success", "Task submitted successfully", response));
            } else {
                String errorMsg = getErrorMessage(result.getStatusCode());
                sendResponse(exchange, 400, new ApiResponse("error", errorMsg, null));
            }
            
        } catch (Exception e) {
            sendResponse(exchange, 500, new ApiResponse("error", "Internal server error: " + e.getMessage(), null));
        }
    }
    
    private void handleStatusRequest(HttpExchange exchange) throws IOException {
        if (!"GET".equals(exchange.getRequestMethod())) {
            sendResponse(exchange, 405, new ApiResponse("error", "Method not allowed", null));
            return;
        }
        
        String path = exchange.getRequestURI().getPath();
        String taskId = extractTaskIdFromPath(path, "/api/tasks/status/");
        
        if (taskId == null) {
            sendResponse(exchange, 400, new ApiResponse("error", "Task ID is required", null));
            return;
        }
        
        try {
            TaskStatusResponse status = optimizerService.getTaskStatus(taskId);
            TaskStatusDto response = convertToTaskStatusDto(status);
            sendResponse(exchange, 200, new ApiResponse("success", "Status retrieved", response));
        } catch (Exception e) {
            sendResponse(exchange, 500, new ApiResponse("error", "Failed to get task status: " + e.getMessage(), null));
        }
    }
    
    private void handleStopRequest(HttpExchange exchange) throws IOException {
        if (!"POST".equals(exchange.getRequestMethod())) {
            sendResponse(exchange, 405, new ApiResponse("error", "Method not allowed", null));
            return;
        }
        
        String path = exchange.getRequestURI().getPath();
        String taskId = extractTaskIdFromPath(path, "/api/tasks/stop/");
        
        if (taskId == null) {
            sendResponse(exchange, 400, new ApiResponse("error", "Task ID is required", null));
            return;
        }
        
        try {
            TaskMonitor monitor = taskMonitors.get(taskId);
            if (monitor != null) {
                monitor.requestStop();
            }
            
            TaskStatusResponse result = optimizerService.stopTask(taskId);
            TaskStatusDto response = convertToTaskStatusDto(result);
            sendResponse(exchange, 200, new ApiResponse("success", "Task stopped", response));
        } catch (Exception e) {
            sendResponse(exchange, 500, new ApiResponse("error", "Failed to stop task: " + e.getMessage(), null));
        }
    }
    
    private void handleStatsRequest(HttpExchange exchange) throws IOException {
        if (!"GET".equals(exchange.getRequestMethod())) {
            sendResponse(exchange, 405, new ApiResponse("error", "Method not allowed", null));
            return;
        }
        System.out.println("Received stats request");
        try {
            Stats stats = optimizerService.getStats();
            StatsDto response = convertToStatsDto(stats);
            sendResponse(exchange, 200, new ApiResponse("success", "Statistics retrieved", response));
        } catch (Exception e) {
            sendResponse(exchange, 500, new ApiResponse("error", "Failed to get statistics: " + e.getMessage(), null));
        }
    }
    
    private void startTaskMonitoring(String taskId, Double acceptableQuality) {
        scheduler.execute(() -> {
            TaskMonitor monitor = taskMonitors.get(taskId);
            if (monitor == null) return;
            
            try {
                while (!monitor.isStopRequested()) {
                    TaskStatusResponse status = optimizerService.getTaskStatus(taskId);
                    monitor.setLastStatus(status);
                    
                    if (status != null && "FINISHED".equals(status.getStatus())) {
                        break;
                    }
                    
                    if (acceptableQuality != null && status != null && status.getSolution() != null) {
                        CalculationResponse solution = status.getSolution();
                        double usageRate = solution.getTotalUsedAreaRatio() * 100;
                        if (usageRate >= acceptableQuality) {
                            System.out.println("Task " + taskId + " reached acceptable quality: " + 
                                             usageRate + "% >= " + acceptableQuality + "%");
                            optimizerService.stopTask(taskId);
                            break;
                        }
                    }
                    
                    Thread.sleep(1000);
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            } catch (Exception e) {
                System.err.println("Error monitoring task " + taskId + ": " + e.getMessage());
            }
        });
    }
    
    private void cleanupOldTasks() {
        long cutoffTime = System.currentTimeMillis() - TimeUnit.HOURS.toMillis(1);
        taskMonitors.entrySet().removeIf(entry -> {
            TaskMonitor monitor = entry.getValue();
            return monitor.getLastUpdateTime() < cutoffTime;
        });
    }
    
    private CalculationRequest convertToCalculationRequest(OptimizeRequest request) {
        CalculationRequest calcRequest = new CalculationRequest();
        
        List<CalculationRequest.Panel> panels = new ArrayList<>();
        for (PanelDto panelDto : request.getPanels()) {
            CalculationRequest.Panel panel = new CalculationRequest.Panel();
            panel.setId(panelDto.getId());
            panel.setWidth(String.valueOf(panelDto.getWidth()));
            panel.setHeight(String.valueOf(panelDto.getHeight()));
            panel.setCount(panelDto.getCount());
            panel.setEnabled(true);
            panel.setLabel(panelDto.getLabel());
            panels.add(panel);
        }
        calcRequest.setPanels(panels);
        
        List<CalculationRequest.Panel> stockPanels = new ArrayList<>();
        for (StockPanelDto stockDto : request.getStockPanels()) {
            CalculationRequest.Panel stock = new CalculationRequest.Panel();
            stock.setId(stockDto.getId());
            stock.setWidth(String.valueOf(stockDto.getWidth()));
            stock.setHeight(String.valueOf(stockDto.getHeight()));
            stock.setCount(stockDto.getCount());
            stock.setEnabled(true);
            stock.setLabel(stockDto.getLabel());
            stockPanels.add(stock);
        }
        calcRequest.setStockPanels(stockPanels);
        
        Configuration config = new Configuration();
        PerformanceThresholds thresholds = new PerformanceThresholds();
        
        // Set default values
        thresholds.setMaxSimultaneousThreads(5);
        thresholds.setThreadCheckInterval(1000L);
        config.setCutThickness("0");
        config.setMinTrimDimension("0");
        config.setOptimizationFactor(1.0);
        
        if (request.getConfig() != null) {
            ConfigDto configDto = request.getConfig();
            if (configDto.getMaxThreads() != null) {
                thresholds.setMaxSimultaneousThreads(configDto.getMaxThreads());
            }
            if (configDto.getCheckInterval() != null) {
                thresholds.setThreadCheckInterval(configDto.getCheckInterval());
            }
            if (configDto.getOptimizationFactor() != null) {
                config.setOptimizationFactor(configDto.getOptimizationFactor());
            }
        }
        config.setPerformanceThresholds(thresholds);
        calcRequest.setConfiguration(config);
        
        ClientInfo clientInfo = new ClientInfo();
        clientInfo.setId("api-client-" + System.currentTimeMillis());
        calcRequest.setClientInfo(clientInfo);
        
        return calcRequest;
    }
    
    private TaskStatusDto convertToTaskStatusDto(TaskStatusResponse status) {
        if (status == null) {
            return new TaskStatusDto("UNKNOWN", 0, null, null);
        }
        
        SolutionDto solution = null;
        if (status.getSolution() != null) {
            CalculationResponse calcResponse = status.getSolution();
            
            List<MosaicDto> mosaics = null;
            if (calcResponse.getMosaics() != null) {
                mosaics = calcResponse.getMosaics().stream()
                    .map(this::convertToMosaicDto)
                    .collect(Collectors.toList());
            }
            
            solution = new SolutionDto(
                calcResponse.getTotalUsedArea(),
                calcResponse.getTotalWastedArea(),
                calcResponse.getTotalUsedAreaRatio() * 100,
                (int)calcResponse.getTotalNbrCuts(),
                calcResponse.getTotalCutLength(),
                calcResponse.getElapsedTime(),
                mosaics
            );
        }
        
        return new TaskStatusDto(
            status.getStatus(),
            status.getPercentageDone(),
            solution,
            status.getInitPercentage() > 0 ? status.getInitPercentage() : null
        );
    }
    
    private StatsDto convertToStatsDto(Stats stats) {
        return new StatsDto(
            stats.getNbrRunningThreads(),
            stats.getNbrQueuedThreads(),
            (int)stats.getNbrFinishedThreads(),
            (int)stats.getNbrRunningTasks(),
            (int)stats.getNbrFinishedTasks()
        );
    }
    
    private MosaicDto convertToMosaicDto(CalculationResponse.Mosaic mosaic) {
        List<TileDto> tiles = null;
        if (mosaic.getTiles() != null) {
            tiles = mosaic.getTiles().stream()
                .map(this::convertToTileDto)
                .collect(Collectors.toList());
        }
        
        List<CutDto> cuts = null;
        if (mosaic.getCuts() != null) {
            cuts = mosaic.getCuts().stream()
                .map(this::convertToCutDto)
                .collect(Collectors.toList());
        }
        
        return new MosaicDto(
            mosaic.getStockLabel(),
            mosaic.getUsedArea(),
            mosaic.getWastedArea(),
            mosaic.getUsedAreaRatio(),
            mosaic.getNbrFinalPanels(),
            mosaic.getNbrWastedPanels(),
            mosaic.getCutLength(),
            mosaic.getMaterial(),
            tiles,
            cuts
        );
    }
    
    private TileDto convertToTileDto(CalculationResponse.Tile tile) {
        return new TileDto(
            tile.getId(),
            tile.getRequestObjId(),
            tile.getX(),
            tile.getY(),
            tile.getWidth(),
            tile.getHeight(),
            tile.getOrientation(),
            tile.getLabel(),
            tile.isFinal(),
            tile.isHasChildren(),
            tile.isRotated()
        );
    }
    
    private CutDto convertToCutDto(CalculationResponse.Cut cut) {
        return new CutDto(
            cut.getX1(),
            cut.getY1(),
            cut.getX2(),
            cut.getY2(),
            cut.getCutCoord(),
            cut.isHorizontal(),
            cut.getOriginalTileId(),
            cut.getOriginalWidth(),
            cut.getOriginalHeight(),
            cut.getChild1TileId(),
            cut.getChild2TileId()
        );
    }
    
    private String getErrorMessage(String statusCode) {
        switch (statusCode) {
            case "1": return "Invalid panels provided";
            case "2": return "Invalid stock panels provided"; 
            case "3": return "Task already running for this client";
            case "4": return "Server is unavailable";
            case "5": return "Too many panels in request";
            case "6": return "Too many stock panels in request";
            default: return "Error: " + statusCode;
        }
    }
    
    private String extractTaskIdFromPath(String path, String prefix) {
        if (path.startsWith(prefix) && path.length() > prefix.length()) {
            return path.substring(prefix.length());
        }
        return null;
    }
    
    private String readRequestBody(HttpExchange exchange) throws IOException {
        StringBuilder sb = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(
                new InputStreamReader(exchange.getRequestBody(), "UTF-8"))) {
            String line;
            while ((line = reader.readLine()) != null) {
                sb.append(line);
            }
        }
        return sb.toString();
    }
    
    private void sendResponse(HttpExchange exchange, int statusCode, Object response) throws IOException {
        String jsonResponse = objectMapper.writeValueAsString(response);
        byte[] responseBytes = jsonResponse.getBytes("UTF-8");
        
        exchange.getResponseHeaders().set("Content-Type", "application/json; charset=UTF-8");
        exchange.getResponseHeaders().set("Access-Control-Allow-Origin", "*");
        exchange.sendResponseHeaders(statusCode, responseBytes.length);
        
        try (OutputStream os = exchange.getResponseBody()) {
            os.write(responseBytes);
        }
    }
}