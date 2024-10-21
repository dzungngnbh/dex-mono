Experimental web app using only backend and htmx

No more experimental, it's the way to build fast web application.

# stack 
- hotwired
- rust 
- template: sailfish and maud html macro!

# templates
- Always use sailfish first, maud is for simple html and move to sailfish template if we have reuse component.

# How to run application
1. Running redis, clickhouse
2. Create def_exchange database, and int seed data
3. Generate key for https locally
```
openssl genrsa -out key.pem 2048
openssl req -new -key key.pem -out csr.pem
openssl req -x509 -new -nodes -key key.pem -subj "/C=US/ST=California/L=San Francisco/O=YourCompany/CN=localhost" -days 365 -out cert.pem
```

4. Build css files
```
cd web-app; 
just compile-js-webapp
just compile-css-prototype
```

5. Running application in dev mode

```
cd web-app; cargo run;
# enter https://0.0.0.0:3000/prototypes/trade/index.html
```