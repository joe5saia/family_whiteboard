// Family Todo App JavaScript Module - Backend API Version

import TodoApiClient from './api-client.js';

class TodoAppController {
    constructor() {
        this.apiClient = new TodoApiClient();
        this.todosData = [];
        this.currentFilters = {
            assignee: 'all',
            status: 'all',
            dateFrom: '',
            dateTo: ''
        };
    }

    async initialize() {
        this.setupEventListeners();
        this.setupWebSocket();
        await this.loadTodos();
    }

    setupEventListeners() {
        // Enter key for adding todos
        document.getElementById('todoText').addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                this.addTodo();
            }
        });

        // Close modal when clicking outside
        document.getElementById('filterModal').addEventListener('click', (e) => {
            if (e.target === e.currentTarget) {
                this.toggleFilterModal();
            }
        });
    }

    setupWebSocket() {
        // Set up real-time WebSocket listeners
        this.apiClient.on('todo_created', (todo) => {
            console.log('Todo created:', todo);
            this.loadTodos(); // Refresh the list
        });

        this.apiClient.on('todo_updated', (todo) => {
            console.log('Todo updated:', todo);
            this.loadTodos(); // Refresh the list
        });

        this.apiClient.on('todo_toggled', (todo) => {
            console.log('Todo toggled:', todo);
            this.loadTodos(); // Refresh the list
        });

        this.apiClient.on('todo_deleted', (data) => {
            console.log('Todo deleted:', data);
            this.loadTodos(); // Refresh the list
        });

        this.apiClient.on('connected', () => {
            console.log('Real-time connection established');
        });

        this.apiClient.on('disconnected', () => {
            console.log('Real-time connection lost');
        });

        // Connect to WebSocket
        this.apiClient.connectWebSocket();
    }

    // Data loading
    async loadTodos() {
        try {
            const groupedTodos = await this.apiClient.getTodos();
            this.todosData = this.apiClient.transformTodosGroupedFromBackend(groupedTodos);
            this.renderTodos();
        } catch (error) {
            console.error('Failed to load todos:', error);
            // Fallback to empty state
            this.todosData = [];
            this.renderTodos();
        }
    }

    // Todo Management Methods
    async addTodo() {
        const text = document.getElementById('todoText').value.trim();
        const assignee = document.getElementById('assignee').value;
        const date = document.getElementById('dueDate').value;
        
        if (!text) {
            alert('Please enter a task');
            return;
        }
        
        try {
            await this.apiClient.createTodo(text, assignee, date);
            this.clearForm();
            // The WebSocket will trigger a refresh automatically
        } catch (error) {
            console.error('Failed to create todo:', error);
            alert('Failed to create todo. Please try again.');
        }
    }

    async toggleTodo(id) {
        try {
            await this.apiClient.toggleTodo(id);
            // The WebSocket will trigger a refresh automatically
        } catch (error) {
            console.error('Failed to toggle todo:', error);
            alert('Failed to update todo. Please try again.');
        }
    }

    editTodo(id) {
        const todoItem = document.querySelector(`[data-id="${id}"]`);
        const todo = this.findTodoById(id);
        
        if (!todo) return;
        
        todoItem.innerHTML = this.generateEditForm(todo);
    }

    async saveTodo(id) {
        const text = document.getElementById(`editText-${id}`).value.trim();
        const assignee = document.getElementById(`editAssignee-${id}`).value;
        const date = document.getElementById(`editDate-${id}`).value;
        
        if (!text) {
            alert('Please enter a task');
            return;
        }
        
        try {
            await this.apiClient.updateTodo(id, { text, assignee, date });
            // The WebSocket will trigger a refresh automatically
        } catch (error) {
            console.error('Failed to update todo:', error);
            alert('Failed to update todo. Please try again.');
        }
    }

    cancelEdit() {
        this.renderTodos();
    }

    // Helper Methods
    findTodoById(id) {
        for (const [date, todos] of this.todosData) {
            const todo = todos.find(t => t.id === id);
            if (todo) return todo;
        }
        return null;
    }

    clearForm() {
        document.getElementById('todoText').value = '';
        document.getElementById('dueDate').value = '';
    }

    // Rendering Methods (keeping most of the existing logic)
    renderTodos() {
        const todoList = document.getElementById('todoList');
        const filteredGroupedTodos = this.applyFiltersToGroups(this.todosData);
        
        if (filteredGroupedTodos.length === 0) {
            todoList.innerHTML = this.generateEmptyState();
            return;
        }
        
        todoList.innerHTML = filteredGroupedTodos
            .map(([date, todos]) => this.generateDateGroup(date, todos))
            .join('');
    }

    applyFiltersToGroups(groupedTodos) {
        return groupedTodos
            .map(([date, todos]) => [date, this.filterTodos(todos)])
            .filter(([date, todos]) => todos.length > 0);
    }

    generateEmptyState() {
        const hasFilters = this.hasActiveFilters();
        const message = hasFilters 
            ? 'No todos match the current filters.' 
            : 'No todos yet. Add one above!';
        return `<div class="empty-state">${message}</div>`;
    }

    generateDateGroup(date, todos) {
        const dateKey = date.replace(/[^a-zA-Z0-9]/g, '_');
        const isCollapsed = localStorage.getItem(`collapsed_${dateKey}`) === 'true';
        const completedCount = todos.filter(todo => todo.completed).length;
        const totalCount = todos.length;
        
        return `
            <div class="date-group">
                <div class="date-header ${isCollapsed ? 'collapsed' : ''}" onclick="todoController.toggleDateGroup('${dateKey}')">
                    <div class="date-title">${this.formatDateHeader(date)}</div>
                    <div style="display: flex; align-items: center; gap: 10px;">
                        <div class="date-count">${completedCount}/${totalCount}</div>
                        <div class="collapse-arrow ${isCollapsed ? 'collapsed' : ''}">▼</div>
                    </div>
                </div>
                <div class="date-todos ${isCollapsed ? 'collapsed' : ''}" id="todos_${dateKey}">
                    ${todos.map(todo => this.generateTodoItem(todo)).join('')}
                </div>
            </div>
        `;
    }

    generateTodoItem(todo) {
        return `
            <div class="todo-item ${todo.completed ? 'completed' : ''}" data-id="${todo.id}">
                <input type="checkbox" class="todo-checkbox" 
                       ${todo.completed ? 'checked' : ''} 
                       onchange="todoController.toggleTodo(${todo.id})">
                <div class="todo-content">
                    <div class="todo-text">${this.escapeHtml(todo.text)}</div>
                    <div class="todo-meta">
                        <span class="assignee ${todo.assignee.toLowerCase()}">${todo.assignee}</span>
                        ${todo.date ? ` • Due: ${todo.date}` : ''}
                    </div>
                </div>
                <div class="todo-actions">
                    <button class="edit-btn" onclick="todoController.editTodo(${todo.id})">Edit</button>
                </div>
            </div>
        `;
    }

    generateEditForm(todo) {
        return `
            <input type="checkbox" class="todo-checkbox" 
                   ${todo.completed ? 'checked' : ''} 
                   onchange="todoController.toggleTodo(${todo.id})">
            <div class="todo-content">
                <div class="edit-form">
                    <input type="text" id="editText-${todo.id}" value="${this.escapeHtml(todo.text)}">
                    <select id="editAssignee-${todo.id}">
                        <option value="Unassigned" ${todo.assignee === 'Unassigned' ? 'selected' : ''}>Unassigned</option>
                        <option value="Joe" ${todo.assignee === 'Joe' ? 'selected' : ''}>Joe</option>
                        <option value="Shannon" ${todo.assignee === 'Shannon' ? 'selected' : ''}>Shannon</option>
                    </select>
                    <input type="date" id="editDate-${todo.id}" value="${todo.date}">
                    <div class="edit-actions">
                        <button class="save-btn" onclick="todoController.saveTodo(${todo.id})">Save</button>
                        <button class="cancel-btn" onclick="todoController.cancelEdit()">Cancel</button>
                    </div>
                </div>
            </div>
        `;
    }

    // Date and Time Utilities
    formatDateHeader(date) {
        if (date === 'No Due Date') {
            return 'No Due Date';
        }
        
        try {
            const dateObj = new Date(date + 'T00:00:00');
            const today = new Date();
            const tomorrow = new Date(today);
            tomorrow.setDate(tomorrow.getDate() + 1);
            const yesterday = new Date(today);
            yesterday.setDate(yesterday.getDate() - 1);
            
            if (dateObj.toDateString() === today.toDateString()) {
                return 'Today';
            } else if (dateObj.toDateString() === tomorrow.toDateString()) {
                return 'Tomorrow';
            } else if (dateObj.toDateString() === yesterday.toDateString()) {
                return 'Yesterday';
            } else {
                return dateObj.toLocaleDateString('en-US', { 
                    weekday: 'long', 
                    year: 'numeric', 
                    month: 'long', 
                    day: 'numeric' 
                });
            }
        } catch (e) {
            return date;
        }
    }

    // UI Interaction Methods
    toggleDateGroup(dateKey) {
        const todosElement = document.getElementById(`todos_${dateKey}`);
        const headerElement = document.querySelector(`[onclick="todoController.toggleDateGroup('${dateKey}')"]`);
        const arrowElement = headerElement.querySelector('.collapse-arrow');
        
        const isCollapsed = todosElement.classList.contains('collapsed');
        
        if (isCollapsed) {
            todosElement.classList.remove('collapsed');
            headerElement.classList.remove('collapsed');
            arrowElement.classList.remove('collapsed');
            localStorage.setItem(`collapsed_${dateKey}`, 'false');
        } else {
            todosElement.classList.add('collapsed');
            headerElement.classList.add('collapsed');
            arrowElement.classList.add('collapsed');
            localStorage.setItem(`collapsed_${dateKey}`, 'true');
        }
    }

    toggleFilterModal() {
        const modal = document.getElementById('filterModal');
        modal.classList.toggle('show');
    }

    // Filter Methods
    applyFilters() {
        this.currentFilters.assignee = document.getElementById('filterAssignee').value;
        this.currentFilters.status = document.getElementById('filterStatus').value;
        this.currentFilters.dateFrom = document.getElementById('filterDateFrom').value;
        this.currentFilters.dateTo = document.getElementById('filterDateTo').value;
        
        this.renderTodos();
        this.updateFilterStatus();
    }

    clearFilters() {
        this.currentFilters = {
            assignee: 'all',
            status: 'all',
            dateFrom: '',
            dateTo: ''
        };
        
        document.getElementById('filterAssignee').value = 'all';
        document.getElementById('filterStatus').value = 'all';
        document.getElementById('filterDateFrom').value = '';
        document.getElementById('filterDateTo').value = '';
        
        this.renderTodos();
        this.updateFilterStatus();
    }

    updateFilterStatus() {
        const statusEl = document.getElementById('filterStatus');
        const filterBtn = document.getElementById('filterBtn');
        const filterBadge = document.getElementById('filterBadge');
        const activeFilters = this.getActiveFiltersDescription();
        
        if (activeFilters.length > 0) {
            filterBtn.classList.add('active');
            filterBadge.textContent = activeFilters.length;
            statusEl.textContent = `Active filters: ${activeFilters.join(', ')}`;
        } else {
            filterBtn.classList.remove('active');
            filterBadge.textContent = '0';
            statusEl.textContent = '';
        }
    }

    getActiveFiltersDescription() {
        const activeFilters = [];
        
        if (this.currentFilters.assignee !== 'all') {
            activeFilters.push(`Assignee: ${this.currentFilters.assignee}`);
        }
        if (this.currentFilters.status !== 'all') {
            activeFilters.push(`Status: ${this.currentFilters.status}`);
        }
        if (this.currentFilters.dateFrom || this.currentFilters.dateTo) {
            const fromDate = this.currentFilters.dateFrom || 'any';
            const toDate = this.currentFilters.dateTo || 'any';
            activeFilters.push(`Date: ${fromDate} to ${toDate}`);
        }
        
        return activeFilters;
    }

    filterTodos(todos) {
        return todos.filter(todo => {
            if (this.currentFilters.assignee !== 'all' && todo.assignee !== this.currentFilters.assignee) {
                return false;
            }
            
            if (this.currentFilters.status !== 'all') {
                const isCompleted = todo.completed;
                if (this.currentFilters.status === 'completed' && !isCompleted) {
                    return false;
                }
                if (this.currentFilters.status === 'pending' && isCompleted) {
                    return false;
                }
            }
            
            if (this.currentFilters.dateFrom || this.currentFilters.dateTo) {
                if (!todo.date) {
                    return !this.currentFilters.dateFrom && !this.currentFilters.dateTo;
                }
                
                const todoDate = new Date(todo.date + 'T00:00:00');
                
                if (this.currentFilters.dateFrom) {
                    const fromDate = new Date(this.currentFilters.dateFrom + 'T00:00:00');
                    if (todoDate < fromDate) {
                        return false;
                    }
                }
                
                if (this.currentFilters.dateTo) {
                    const toDate = new Date(this.currentFilters.dateTo + 'T23:59:59');
                    if (todoDate > toDate) {
                        return false;
                    }
                }
            }
            
            return true;
        });
    }

    hasActiveFilters() {
        return this.currentFilters.assignee !== 'all' || 
               this.currentFilters.status !== 'all' || 
               this.currentFilters.dateFrom || 
               this.currentFilters.dateTo;
    }

    // Utility Methods
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Initialize the application
const todoController = new TodoAppController();

// Make controller globally available for onclick handlers
window.todoController = todoController;

// Legacy function support for existing HTML onclick handlers
window.addTodo = () => todoController.addTodo();
window.toggleTodo = (id) => todoController.toggleTodo(id);
window.editTodo = (id) => todoController.editTodo(id);
window.saveTodo = (id) => todoController.saveTodo(id);
window.cancelEdit = () => todoController.cancelEdit();
window.toggleDateGroup = (dateKey) => todoController.toggleDateGroup(dateKey);
window.toggleFilterModal = () => todoController.toggleFilterModal();
window.applyFilters = () => todoController.applyFilters();
window.clearFilters = () => todoController.clearFilters();

// Start the application
todoController.initialize();