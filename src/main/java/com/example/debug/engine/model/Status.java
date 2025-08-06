package com.example.debug.engine.model;

/**
 * Статусы задач
 */
public enum Status {
    IDLE,
    QUEUED,
    RUNNING,
    FINISHED, 
    STOPPED,
    TERMINATED,
    ERROR
}
