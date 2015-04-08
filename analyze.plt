file = "results.txt"
# W = F * t
# W/t = F
#f(x) = 1/((x-4)*(x-2)*(x-1))
#  plot f(x) title "func" with lines

#set dgrid3d 30, 30
#
#set table file.'.grid'
#splot file u 1:2:3
#unset table
#
#set table file.'.color'
#splot file u 1:2:4
#unset table
#
#
#set xyplane 0
#set hidden3d
#set autoscale cbfix
#set pm3d
#unset dgrid3d
#set ticslevel 0
#splot sprintf('< paste %s.grid %s.color', file, file) u 1:2:3:7 with pm3d notitle
#splot file using 1:2:3:4 with pm3d
#plot 'results.txt' using 1:2 with lines, \
#     'results.txt' using 1:3 with lines

#set xrange [0:10]
#set yrange [0:4]
#set zrange [0:10]
set view equal xyz
set xlabel "x axis" 
set ylabel "z axis" 
set zlabel "y axis" 
splot file using 1:3:2 pt 7 ps 0.3
pause -1
