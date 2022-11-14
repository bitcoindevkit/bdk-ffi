./gradlew dokkaHtml
cd build/dokka/html
git init .
git add .
git switch --create gh-pages
git commit -m "Deploy"
git remote add origin git@github.com:bitcoindevkit/bdk-kotlin.git
git push --set-upstream origin gh-pages --force
