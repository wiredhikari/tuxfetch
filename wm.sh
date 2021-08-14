#!/bin/sh
id=$(xprop -root -notype _NET_SUPPORTING_WM_CHECK)
id=${id##* }

#echo $id
 wm=$(xprop -id "$id" -notype -len 25 -f _NET_WM_NAME 8t)
  wm=${wm##*_NET_WM_NAME = \"}
  wm=${wm%%\"*}
 echo $wm


