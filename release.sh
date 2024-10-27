# 
npx tailwindcss -i ./input.css -o ./style/tailwind.css 
trunk build --release -d docs
# rename in /docs/index.html all "/ to "./
sed -i '' 's|\"/|\"./|g' ./docs/index.html
sed -i '' "s|\'/|\'./|g" ./docs/index.html

