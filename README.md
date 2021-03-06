# How to run the code for limits extraction

* Get combine tools:

```
cmsrel CMSSW_8_1_0
cd CMSSW_8_1_0/src 
cmsenv
git clone https://github.com/cms-analysis/HiggsAnalysis-CombinedLimit.git HiggsAnalysis/CombinedLimit
cd HiggsAnalysis/CombinedLimit
git fetch origin
git checkout v7.0.9
scramv1 b clean; scramv1 b
```
Get a stable version of the code:
```
cd ../
git clone git@github.com:ivovtin/bbggLimits2018.git -b run2_analysis 
cd bbggLimits2018
scramv1 b
```
## 1. NOTE: Use samples obtained from the default branch of flashgg ([link](https://github.com/cms-analysis/flashgg)) WITHOUT categorization applied.

## 2. Making limit trees (LT)
```
. run1.sh
```
The goal of this code is to categorize events and make a new tree which *catID* variable, as well as *mgg* and *mjj*. By default 2D directory would be produed. But you can go to run1.sh and uncomment 6 1D lines to produce also 2D directory

### Notes on limit trees

The core code that makes the trees is `bbggLTMaker.C`. It is based on
[TSelector](https://root.cern.ch/developing-tselector) and does not depend on CMSSW, just
the ROOT.  
Different type of categorizations can be done chosen by option `-c Y`
The recommended option is Y = 2:  
```
 Y = 0: HIG-17-008 categorization used in 2016 analysis (4 categories)
 Y = 1: HIG-19-018 tagger, using 2016 style categorization (4 categories);
 Y = 2: HIG-19-018 tagger, 2D with optimized categorization without mjj cut (12 categories);
 Y = 3: HIG-19-018 tagger, 1D with optimized categorization and mjj cuts (12 categories);
```

## 3. Performing fit and extraction of limits

 1. Prepare a template file of the following structure: ([Envelopejson](https://github.com/ivovtin/bbggLimits2018/blob/run2_analysis/jsonsForEnvelope/Env_json_2D_ttHon0.26_31012020_emplty.dat)) for *mgg* and *mjj* proections without indicating of the best functions for each category. Put a link to the created files here: [link](https://github.com/ivovtin/bbggLimits2018/blob/8480c7351cfd4db16818f292bcc78c01bf567ba1/runLimit.py#L300)

 2. Create a workspace using the produced LTs:
```
. run2.sh
```

Edit the config file `json/conf_default.json`. Put the path to the LTs there:
```
LTDIR: location of the input Limit Trees (expected to be in the local diractory, after running previous step)
```
To choose the number of categories and fit strategy edit the following parameters:
```
ncat: number of categories. This should much the number of categories produced in limit tries (currently, should be 4 or 12)
fitStrategy: 2 - for 2D fit of (mgg, mjj); 1 - for 1D fit of mgg
```

The results of the limit will be put at `LIMS_OutDir/Node_SM/result_1.log`. You can uncomment 1D if you want to process it.


 3. Run the FTest on the workspace ([link](https://github.com/ivovtin/Envelop#ftest))

 4. Write the orders obtaing from the step 3 to the file from step 1. Example here: [Envelopejson](https://github.com/ivovtin/bbggLimits2018/blob/run2_analysis/jsonsForEnvelope/Env_json_2D_ttHon0.26_31012020.dat)

 5. Update workspace with the received orders from FTest:
```
. run2.sh
```

 6. Choose the best function for each category using the code below: [link](https://github.com/ivovtin/Envelop#ftest)

 7. Write the best functions to the file from the step 1 like here: [Envelopejson](https://github.com/ivovtin/bbggLimits2018/blob/run2_analysis/jsonsForEnvelope/Env_json_2D_ttHon0.26_31012020.dat) for each category at the column after order numbers. Use comma for separation.

 8. Update workspace with received functions:
```
. run2.sh
```

 9. Extract the limit: 
```
. run3.sh
```

The process may take a while to complete, especially when running with many categories (about 30 minuts for 2D method and 12 categories). You can uncomment 1D if you want to process it.



