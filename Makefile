# Makefile for Cutting Optimization API

JAVA_SOURCES := $(shell find src -name "*.java")
LIB_JARS := $(shell find lib -name "*.jar" | tr '\n' ':' | sed 's/:$$//')
CLASSPATH := lib/*:src/main/java
BUILD_DIR := build
MAIN_CLASS := com.example.debug.api.ApiServer
PID_FILE := api.pid
LOG_FILE := api.log

.PHONY: all clean compile run start stop restart help logs status

# Default target
all: compile

# Help target
help:
	@echo "Available targets:"
	@echo "  compile  - Compile all Java sources"
	@echo "  run      - Compile and run the API server (foreground)"
	@echo "  start    - Start the API server in background with logging"
	@echo "  stop     - Stop the API server"
	@echo "  restart  - Stop and start the API server"
	@echo "  status   - Check if API server is running"
	@echo "  logs     - Show API server logs (tail -f)"
	@echo "  clean    - Remove compiled classes and logs"
	@echo "  help     - Show this help"

# Create build directory
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Compile all Java sources
compile: $(BUILD_DIR)
	@echo "Compiling Java sources..."
	javac -cp "$(CLASSPATH)" -d $(BUILD_DIR) $(JAVA_SOURCES)
	@echo "Compilation completed successfully"

# Run the API server (foreground)
run: compile
	@echo "Starting API server in foreground..."
	java -cp "$(CLASSPATH):$(BUILD_DIR)" $(MAIN_CLASS)

# Start API server in background
start: compile stop
	@echo "Starting API server in background..."
	@nohup java -cp "$(CLASSPATH):$(BUILD_DIR)" $(MAIN_CLASS) > $(LOG_FILE) 2>&1 & echo $! > $(PID_FILE)
	@sleep 3
	@if [ -f $(PID_FILE) ] && kill -0 `cat $(PID_FILE)` 2>/dev/null; then \
		echo "API server started successfully (PID: `cat $(PID_FILE)`)"; \
		echo "Log file: $(LOG_FILE)"; \
		echo "Use 'make logs' to view logs or 'make stop' to stop the server"; \
	else \
		echo "Failed to start API server. Check $(LOG_FILE) for errors."; \
		rm -f $(PID_FILE); \
		if [ -f $(LOG_FILE) ]; then \
			echo "Last few lines from log:"; \
			tail -n 10 $(LOG_FILE); \
		fi; \
		exit 1; \
	fi

# Stop API server
stop:
	@echo "Stopping API server..."
	@if [ -f $(PID_FILE) ]; then \
		PID=`cat $(PID_FILE)`; \
		if kill -0 $PID 2>/dev/null; then \
			echo "Sending TERM signal to process $PID..."; \
			kill -TERM $PID 2>/dev/null || true; \
			sleep 3; \
			if kill -0 $PID 2>/dev/null; then \
				echo "Process still running, sending KILL signal..."; \
				kill -KILL $PID 2>/dev/null || true; \
				sleep 2; \
			fi; \
			echo "API server stopped (PID: $PID)"; \
		else \
			echo "Process $PID not found, removing stale PID file"; \
		fi; \
		rm -f $(PID_FILE); \
	fi
	@# Kill any remaining processes by name as fallback
	@if pgrep -f "$(MAIN_CLASS)" >/dev/null 2>&1; then \
		echo "Killing remaining $(MAIN_CLASS) processes..."; \
		pkill -TERM -f "$(MAIN_CLASS)" >/dev/null 2>&1 || true; \
		sleep 3; \
		if pgrep -f "$(MAIN_CLASS)" >/dev/null 2>&1; then \
			echo "Forcing kill of remaining processes..."; \
			pkill -KILL -f "$(MAIN_CLASS)" >/dev/null 2>&1 || true; \
			sleep 2; \
		fi; \
	fi
	@if ! pgrep -f "$(MAIN_CLASS)" >/dev/null 2>&1; then \
		echo "All API server processes stopped"; \
	else \
		echo "Warning: Some processes may still be running"; \
		pgrep -f "$(MAIN_CLASS)"; \
	fi

# Check server status
status:
	@if [ -f $(PID_FILE) ] && kill -0 `cat $(PID_FILE)` 2>/dev/null; then \
		echo "API server is running (PID: `cat $(PID_FILE)`)"; \
		echo "Listening on port 8080"; \
	elif pgrep -f "$(MAIN_CLASS)" >/dev/null 2>&1; then \
		echo "API server process found but no PID file:"; \
		pgrep -f "$(MAIN_CLASS)"; \
	else \
		echo "API server is not running"; \
	fi

# Clean compiled classes and logs
clean:
	@echo "Cleaning build directory and logs..."
	@# Stop any running processes first - more aggressive approach
	@if pgrep -f "$(MAIN_CLASS)" >/dev/null 2>&1; then \
		echo "Stopping running processes..."; \
		pkill -TERM -f "$(MAIN_CLASS)" >/dev/null 2>&1 || true; \
		sleep 3; \
		if pgrep -f "$(MAIN_CLASS)" >/dev/null 2>&1; then \
			echo "Processes still running, forcing kill..."; \
			pkill -KILL -f "$(MAIN_CLASS)" >/dev/null 2>&1 || true; \
			sleep 2; \
		fi; \
	fi
	rm -rf $(BUILD_DIR)
	rm -f *.class
	rm -f $(LOG_FILE) $(PID_FILE) nohup.out
	@echo "Clean completed"

# Restart server
restart: stop
	@sleep 1
	@$(MAKE) start

# Show logs
logs:
	@if [ -f $(LOG_FILE) ]; then \
		echo "Showing logs from $(LOG_FILE) (Press Ctrl+C to exit):"; \
		tail -f $(LOG_FILE); \
	else \
		echo "Log file $(LOG_FILE) not found. Server may not be running in background."; \
	fi

# Show recent logs without following
show-logs:
	@if [ -f $(LOG_FILE) ]; then \
		echo "Recent logs from $(LOG_FILE):"; \
		tail -n 50 $(LOG_FILE); \
	else \
		echo "Log file $(LOG_FILE) not found."; \
	fi