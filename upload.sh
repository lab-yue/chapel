# upload log

cd build-log
git add .

DATE=`date '+%Y-%m-%d %H:%M:%S'`
git commit -m "build-log update $DATE"

git push origin master

cd ..

# upload rs

git commit -m "prototype dev $DATE"

git push origin master
