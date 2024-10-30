# 
# npx tailwindcss -i ./input.css -o ./style/tailwind.css 
TRUNK_TOOLS_TAILWINDCSS=3.4.14 trunk build --release -d docs
# rename in /docs/index.html all "/ to "./
sed -i '' 's|\"/|\"./|g' ./docs/index.html
sed -i '' "s|\'/|\'./|g" ./docs/index.html

