package com.example.debug.api;

import com.example.debug.engine.CutListOptimizerService;
import com.example.debug.engine.CutListOptimizerServiceImpl;

public class ApiServer {
    public static void main(String[] args) {
        try {
            CutListOptimizerService optimizerService = CutListOptimizerServiceImpl.getInstance();
            optimizerService.init(8);
            optimizerService.setAllowMultipleTasksPerClient(true);
            optimizerService.setCutListLogger(new com.example.debug.engine.console.ConsoleCutListLogger());
            
            CuttingOptimizationController controller = new CuttingOptimizationController(optimizerService);
            controller.start();
            
            System.out.println("API Server is running...");
            System.out.println("Press Ctrl+C to stop");
            
            Runtime.getRuntime().addShutdownHook(new Thread(() -> {
                System.out.println("Shutting down API Server...");
                controller.stop();
            }));
            
            Thread.currentThread().join();
            
        } catch (Exception e) {
            System.err.println("Failed to start API Server: " + e.getMessage());
            e.printStackTrace();
        }
    }
}