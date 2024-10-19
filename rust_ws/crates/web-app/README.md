Experimental web app using only backend and htmx

No more experimental, it's the way to build fast web application.

# stack 
hotwired
rust 
template: sailfish and maud html macro!

# templates
- Always use sailfish first, maud is for simple html and move to sailfish template if we have reuse component.

# commands 
```
# compile typescript 
swc -d dist ts --watch

# compile tailwind
pnpm dlx tailwindcss -i .\public\global.css -o .\dist\output.css --minify --watch

# run server 
rmrf .......//* ; cargo run # TODO
```