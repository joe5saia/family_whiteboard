# Use nginx to serve static files
FROM nginx:alpine

# Copy static files
COPY frontend/index.html /usr/share/nginx/html/
COPY frontend/styles.css /usr/share/nginx/html/
COPY frontend/app.js /usr/share/nginx/html/
COPY frontend/app-wasm.js /usr/share/nginx/html/
COPY frontend/api-client.js /usr/share/nginx/html/

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]