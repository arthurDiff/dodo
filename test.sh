for idx in $(seq 1 25);
do
  echo "first stdout value from $idx"
  if [[ $(($idx % 2)) -eq 0 ]]; then
    # intentionally slowed
    sleep .5
  fi
done