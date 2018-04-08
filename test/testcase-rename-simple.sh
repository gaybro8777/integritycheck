#!/bin/bash
set -ue
source test/test-util.sh
mkdir "${TEST_TMPDIR}/repo"
cd "${TEST_TMPDIR}/repo"

echo "A" > testA
echo "B" > testB
echo "C" > testC

fhistory init
fhistory status

mv testB testX

if fhistory status --colours=off > "../status.raw"; then
  echo "exit code must be one"
  exit 1
fi

cat "../status.raw" | grep -vE "^Repository" | grep -vE "^Last Snapshot" > "../status"

echo "---"
cat "../status"
echo "---"

(cat > "../status.expected") <<EOF
Status: DIRTY

    renamed  "testB" -> "testX"

EOF

diff "../status" "../status.expected"
 
sleep 1

fhistory ack .
fhistory status
