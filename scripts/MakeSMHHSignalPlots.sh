#!/bin/bash

DIR=$1

OUT=${DIR}/Node_SM/SignalShapes

mkdir ${OUT}

for icat in {0..11}
do
    python scripts/MakeSigPlot.py -p ${OUT} -w ${DIR}/Node_SM/ws_hhbbgg.HH.sig.mH125_13TeV.root -c ${icat} -o "mjj,mgg" -l "2016+2017 analysis" -a "#font[61]{pp#rightarrowHH#rightarrowb#bar{b}#gamma#gamma}|SM-like HH" -b 24,160 -d 1

done


