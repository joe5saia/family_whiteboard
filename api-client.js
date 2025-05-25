// API Client for Backend Communication

class TodoApiClient {
    constructor(baseUrl = 'http://localhost:3000') {
        this.baseUrl = baseUrl;
        this.websocket = null;
        this.eventListeners = new Map();
    }

    // REST API Methods
    async getTodos() {
        const response = await fetch(`${this.baseUrl}/api/todos`);
        if (!response.ok) {
            throw new Error(`Failed to fetch todos: ${response.statusText}`);
        }
        return response.json();
    }

    async createTodo(text, assignee, dueDate) {
        const response = await fetch(`${this.baseUrl}/api/todos`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                text,
                assignee,
                due_date: dueDate || null,
            }),
        });
        
        if (!response.ok) {
            throw new Error(`Failed to create todo: ${response.statusText}`);
        }
        return response.json();
    }

    async updateTodo(id, updates) {
        const response = await fetch(`${this.baseUrl}/api/todos/${id}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                text: updates.text,
                assignee: updates.assignee,
                due_date: updates.date || null,
                completed: updates.completed,
            }),
        });
        
        if (!response.ok) {
            throw new Error(`Failed to update todo: ${response.statusText}`);
        }
        return response.json();
    }

    async toggleTodo(id) {
        const response = await fetch(`${this.baseUrl}/api/todos/${id}/toggle`, {
            method: 'PUT',
        });
        
        if (!response.ok) {
            throw new Error(`Failed to toggle todo: ${response.statusText}`);
        }
        return response.json();
    }

    async deleteTodo(id) {
        const response = await fetch(`${this.baseUrl}/api/todos/${id}`, {
            method: 'DELETE',
        });
        
        if (!response.ok) {
            throw new Error(`Failed to delete todo: ${response.statusText}`);
        }
        return true;
    }

    // WebSocket Methods
    connectWebSocket() {
        if (this.websocket) {
            return; // Already connected
        }

        const wsUrl = this.baseUrl.replace('http', 'ws') + '/ws';
        this.websocket = new WebSocket(wsUrl);

        this.websocket.onopen = () => {
            console.log('WebSocket connected');
            this.emit('connected');
        };

        this.websocket.onmessage = (event) => {
            try {
                const message = JSON.parse(event.data);
                console.log('WebSocket message received:', message);
                this.emit(message.message_type, message.data);
            } catch (error) {
                console.error('Failed to parse WebSocket message:', error);
            }
        };

        this.websocket.onclose = () => {
            console.log('WebSocket disconnected');
            this.websocket = null;
            this.emit('disconnected');
            
            // Attempt to reconnect after 3 seconds
            setTimeout(() => {
                this.connectWebSocket();
            }, 3000);
        };

        this.websocket.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.emit('error', error);
        };
    }

    disconnectWebSocket() {
        if (this.websocket) {
            this.websocket.close();
            this.websocket = null;
        }
    }

    // Event handling for WebSocket messages
    on(eventType, callback) {
        if (!this.eventListeners.has(eventType)) {
            this.eventListeners.set(eventType, []);
        }
        this.eventListeners.get(eventType).push(callback);
    }

    off(eventType, callback) {
        if (this.eventListeners.has(eventType)) {
            const listeners = this.eventListeners.get(eventType);
            const index = listeners.indexOf(callback);
            if (index > -1) {
                listeners.splice(index, 1);
            }
        }
    }

    emit(eventType, data) {
        if (this.eventListeners.has(eventType)) {
            this.eventListeners.get(eventType).forEach(callback => {
                try {
                    callback(data);
                } catch (error) {
                    console.error(`Error in event listener for ${eventType}:`, error);
                }
            });
        }
    }

    // Utility method to transform backend format to frontend format
    transformTodoFromBackend(todo) {
        return {
            id: todo.id,
            text: todo.text,
            assignee: todo.assignee,
            date: todo.due_date || '',
            completed: todo.completed,
        };
    }

    transformTodosGroupedFromBackend(groupedTodos) {
        return groupedTodos.map(group => [
            group.date,
            group.todos.map(todo => this.transformTodoFromBackend(todo))
        ]);
    }
}

export default TodoApiClient;